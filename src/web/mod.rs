use crate::config::Config;
use crate::domain::NoteId;
use crate::errors::ApiError;
use crate::http::response::Response;
use pulldown_cmark::{html, Event, Options, Parser};
use crate::related::{fetch_chain, fetch_related, NoteChain, RelatedEntry};
use crate::state::AppState;
use crate::urls::base32::decode_id;
const HOME_TEMPLATE: &str = include_str!("home.html");
const NOTE_TEMPLATE: &str = include_str!("note.html");
pub fn home_html(config: &Config) -> String {
    render_home(&config.google_client_id)
}
pub fn home(config: &Config) -> Response {
    Response::html(home_html(config))
}
pub async fn note_page(
    note_id: &str,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let id_bytes = decode_id(note_id)
        .ok_or_else(|| ApiError::bad_request("invalid_id", "Invalid note id", None))?;
    let note_id = NoteId::from_bytes(id_bytes);
    let chain = fetch_chain(&state, note_id).await?;
    let related = fetch_related(&state, note_id).await?;
    Ok(Response::html(note_html(&state.config, &chain, &related.related)))
}
fn render_home(client_id: &str) -> String {
    let client_id = escape_attr(client_id);
    HOME_TEMPLATE.replace("{{CLIENT_ID}}", &client_id)
}
fn note_html(config: &Config, chain: &NoteChain, related: &[RelatedEntry]) -> String {
    let markdown = chain_markdown(chain);
    let body_html = render_markdown(&markdown);
    let chain_items = render_chain_items(&chain.prev, &chain.next);
    let chain_summary = format!("{} prev, {} next", chain.prev.len(), chain.next.len());
    let related_items = render_related_items(related, &chain.center.id);
    let newer_version = render_newer_version(related, &chain.center.id);
    let client_id = escape_attr(&config.google_client_id);
    let base = NOTE_TEMPLATE
        .replace("{{CLIENT_ID}}", &client_id)
        .replace("{{NOTE_ID}}", &escape_attr(&chain.center.id))
        .replace("{{NOTE_CREATED_AT}}", &escape_html(&chain.center.created_at))
        .replace("{{NOTE_AUTHOR}}", &escape_html(&chain.center.author.email))
        .replace("{{CHAIN_SUMMARY}}", &escape_html(&chain_summary))
        .replace("{{CHAIN_ITEMS}}", "__LGXPKF_CHAIN_ITEMS__")
        .replace("{{RELATED_ITEMS}}", "__LGXPKF_RELATED_ITEMS__")
        .replace("{{NEWER_VERSION}}", "__LGXPKF_NEWER_VERSION__")
        .replace("{{NOTE_BODY}}", "__LGXPKF_NOTE_BODY__")
        .replace("{{NOTE_RAW}}", "__LGXPKF_NOTE_RAW__");
    base.replace("__LGXPKF_CHAIN_ITEMS__", &chain_items)
        .replace("__LGXPKF_RELATED_ITEMS__", &related_items)
        .replace("__LGXPKF_NEWER_VERSION__", &newer_version)
        .replace("__LGXPKF_NOTE_BODY__", &body_html)
        .replace("__LGXPKF_NOTE_RAW__", &escape_html(&markdown))
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
        "<a class=\"chain-item\" href=\"/{note_id}\">\
         <span class=\"chain-label\">{label}</span>\
         <span class=\"chain-text\">{summary}</span>\
         </a>"
    )
}
fn render_related_items(related: &[RelatedEntry], center_id: &str) -> String {
    let items: Vec<String> = related
        .iter()
        .filter(|entry| match entry.association.kind.as_str() {
            "next" | "prev" => false,
            "version" if entry.association.from_id == center_id => false,
            _ => true,
        })
        .map(|entry| render_related_item(entry))
        .collect();
    if items.is_empty() {
        return "<div class=\"empty\">No linked notes.</div>".to_string();
    }
    items.join("")
}
fn render_related_item(entry: &RelatedEntry) -> String {
    let note_id = escape_attr(&entry.note.id);
    let summary = escape_html(&note_excerpt(&entry.note.value, 120));
    let created = escape_html(&entry.note.created_at);
    let kind = escape_html(&entry.association.kind);
    format!(
        "<a class=\"related-item\" href=\"/{note_id}\">\
         <span class=\"related-kind\">{kind}</span>\
         <span class=\"related-text\">{summary}</span>\
         <span class=\"related-meta\">{created}</span>\
         </a>"
    )
}
fn render_newer_version(related: &[RelatedEntry], center_id: &str) -> String {
    let newer = related.iter().find(|entry| {
        entry.association.kind == "version" && entry.association.from_id == center_id
    });
    let Some(entry) = newer else { return String::new() };
    let note_id = escape_attr(&entry.note.id);
    let summary = escape_html(&note_excerpt(&entry.note.value, 96));
    let created = escape_html(&entry.note.created_at);
    format!(
        "<div class=\"card version-card\" id=\"version-card\">\
         <div class=\"card-title\">Newer version</div>\
         <a class=\"version-link\" href=\"/{note_id}\">{summary}</a>\
         <div class=\"helper mono\">{created} · {note_id}</div>\
         </div>"
    )
}
fn note_excerpt(value: &str, max_len: usize) -> String {
    let mut excerpt = String::new();
    let mut count = 0;
    for ch in value.chars().filter(|ch| *ch != '\n' && *ch != '\r') {
        if count >= max_len {
            break;
        }
        excerpt.push(ch);
        count += 1;
    }
    if excerpt.is_empty() {
        return "Empty note".to_string();
    }
    if value.chars().count() > max_len {
        excerpt.push_str("...");
    }
    excerpt
}
fn render_markdown(value: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(value, options).map(|event| match event {
        Event::Html(html) | Event::InlineHtml(html) => Event::Text(html),
        other => other,
    });
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}
fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&#39;")
}
fn escape_html(value: &str) -> String {
    escape_attr(value)
}
