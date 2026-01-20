use crate::config::Config;
use crate::related::{NoteChain, RelatedEntry};
use crate::web::escape::{escape_attr, escape_html};
use crate::web::markdown::{note_excerpt, render_markdown};
use crate::web::render::login_uri;
use crate::web::templates;

pub fn note_html(config: &Config, chain: &NoteChain, related: &[RelatedEntry]) -> String {
    let markdown = chain_markdown(chain);
    let body_html = render_markdown(&markdown);
    let chain_items = render_chain_items(&chain.prev, &chain.next);
    let chain_summary = format!("{} prev, {} next", chain.prev.len(), chain.next.len());
    let related_items = render_related_items(related, &chain.center.id);
    let version_section = render_version_section(related, &chain.center.id);
    let has_newer_version = related
        .iter()
        .any(|entry| entry.association.kind == "version" && entry.association.from_id == chain.center.id);
    let has_newer_version = if has_newer_version { "true" } else { "false" };
    let client_id = escape_attr(&config.google_client_id);
    let login_uri = escape_attr(&login_uri(config));
    let note_id_raw = &chain.center.id;
    let note_id = escape_attr(note_id_raw);
    let author_id = escape_attr(&chain.center.author.user_id.to_string());
    let account_note_id = escape_attr(chain.center.author.account_note_id.as_deref().unwrap_or(""));
    let note_description = escape_attr(&note_excerpt(&chain.center.value, 160));
    let note_url = escape_attr(&format!("{}/{}", config.public_base_url, note_id_raw));
    let base = templates::NOTE
        .replace("{{CLIENT_ID}}", &client_id)
        .replace("{{LOGIN_URI}}", &login_uri)
        .replace("{{NOTE_ID}}", &note_id)
        .replace("{{NOTE_CREATED_AT}}", &escape_html(&chain.center.created_at))
        .replace("{{NOTE_AUTHOR}}", &escape_html(&chain.center.author.email))
        .replace("{{NOTE_AUTHOR_ID}}", &author_id)
        .replace("{{NOTE_ACCOUNT_NOTE_ID}}", &account_note_id)
        .replace("{{NOTE_HAS_NEWER_VERSION}}", has_newer_version)
        .replace("{{CHAIN_SUMMARY}}", &escape_html(&chain_summary))
        .replace("{{NOTE_DESCRIPTION}}", &note_description)
        .replace("{{NOTE_URL}}", &note_url)
        .replace("{{VERSION_SECTION}}", "__lgxpkf_version_section__")
        .replace("{{CHAIN_ITEMS}}", "__lgxpkf_chain_items__")
        .replace("{{RELATED_ITEMS}}", "__lgxpkf_related_items__")
        .replace("{{NOTE_BODY}}", "__lgxpkf_note_body__")
        .replace("{{NOTE_RAW}}", "__lgxpkf_note_raw__");
    base.replace("__lgxpkf_chain_items__", &chain_items)
        .replace("__lgxpkf_version_section__", &version_section)
        .replace("__lgxpkf_related_items__", &related_items)
        .replace("__lgxpkf_note_body__", &body_html)
        .replace("__lgxpkf_note_raw__", &escape_html(&markdown))
}

fn chain_markdown(chain: &NoteChain) -> String {
    let mut parts = Vec::new();
    for note in &chain.prev {
        parts.push(note.value.as_str());
    }
    parts.push(chain.center.value.as_str());
    for note in &chain.next {
        parts.push(note.value.as_str());
    }
    parts.join("\n\n")
}

fn render_chain_items(prev: &[crate::domain::Note], next: &[crate::domain::Note]) -> String {
    if prev.is_empty() && next.is_empty() {
        return "<div class=\"empty\">No chained notes.</div>".to_string();
    }
    let mut items = Vec::new();
    items.extend(prev.iter().map(|note| render_chain_item(note, "Prev")));
    items.extend(next.iter().map(|note| render_chain_item(note, "Next")));
    items.join("")
}

fn render_chain_item(note: &crate::domain::Note, label: &str) -> String {
    let note_id = escape_attr(&note.id);
    let summary = escape_html(&note_excerpt(&note.value, 120));
    let label = escape_html(label);
    format!(
        "<a class=\"chain-item\" href=\"/{note_id}\"><span class=\"chain-label\">{label}</span><span class=\"chain-text\">{summary}</span></a>"
    )
}

fn render_related_items(related: &[RelatedEntry], center_id: &str) -> String {
    let items: Vec<String> = related
        .iter()
        .filter(|entry| !matches!(entry.association.kind.as_str(), "next" | "prev"))
        .map(|entry| render_related_item(entry, center_id))
        .collect();
    if items.is_empty() {
        return "<div class=\"empty\">No linked notes.</div>".to_string();
    }
    items.join("")
}

fn render_related_item(entry: &RelatedEntry, center_id: &str) -> String {
    let note_id = escape_attr(&entry.note.id);
    let summary = escape_html(&note_excerpt(&entry.note.value, 120));
    let created = escape_html(&entry.note.created_at);
    let kind = escape_html(&association_label(&entry.association, center_id));
    let citation = escape_html(&entry.note.id);
    let version_class = if entry.association.kind == "version" {
        " related-item-version"
    } else {
        ""
    };
    format!(
        "<a class=\"related-item{version_class}\" href=\"/{note_id}\"><span class=\"related-kind\">{kind}</span><span class=\"related-text\">{summary}</span><span class=\"related-meta\">{created}</span><span class=\"related-cite\">Citation: {citation}</span></a>"
    )
}

fn render_version_section(related: &[RelatedEntry], center_id: &str) -> String {
    let items: Vec<String> = related
        .iter()
        .filter(|entry| entry.association.kind == "version" && entry.association.from_id == center_id)
        .map(|entry| render_related_item(entry, center_id))
        .collect();
    let hidden = if items.is_empty() { " hidden" } else { "" };
    let body = if items.is_empty() { "".to_string() } else { items.join("") };
    format!(
        "<section class=\"card\" id=\"version-card\"{hidden}><div class=\"card-title\">Newer versions</div><div class=\"related-list\" id=\"version-list\">{body}</div></section>"
    )
}

fn association_label(association: &crate::domain::Association, center_id: &str) -> String {
    match association.kind.as_str() {
        "version" => {
            if association.from_id == center_id {
                "Newer version".to_string()
            } else {
                "Older version".to_string()
            }
        }
        "reply" => {
            if association.from_id == center_id {
                "Reply to".to_string()
            } else {
                "Reply from".to_string()
            }
        }
        _ => association.kind.clone(),
    }
}
