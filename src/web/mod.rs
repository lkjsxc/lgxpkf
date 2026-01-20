use crate::config::Config;
use crate::domain::NoteId;
use crate::errors::ApiError;
use crate::http::response::Response;
use crate::related::{fetch_chain, fetch_related, NoteChain, RelatedEntry};
use crate::state::AppState;
use crate::urls::base32::decode_id;
use pulldown_cmark::{html, Event, Options, Parser};

const HOME_TEMPLATE: &str = include_str!("home.html"); const NOTE_TEMPLATE: &str = include_str!("note.html"); const SIGNIN_TEMPLATE: &str = include_str!("signin.html");
const TERMS_TEMPLATE: &str = include_str!("terms.html"); const PRIVACY_TEMPLATE: &str = include_str!("privacy.html"); const GUIDELINE_TEMPLATE: &str = include_str!("guideline.html"); const NETWORK_TEMPLATE: &str = include_str!("network.html");
const NOTE_JS: &str = include_str!("assets/note.js"); const APP_JS: &str = include_str!("assets/app.js"); const HOME_JS: &str = include_str!("assets/home.js"); const SIGNIN_JS: &str = include_str!("assets/signin.js"); const NETWORK_JS: &str = include_str!("assets/network.js");
const FAVICON: &[u8] = include_bytes!("assets/icon_256.ico"); const CACHE_STATIC: &str = "public, max-age=31536000, immutable";

pub fn favicon() -> Response { Response::bytes(200, "image/vnd.microsoft.icon", FAVICON.to_vec()).with_header("Cache-Control", CACHE_STATIC) }
pub fn note_js() -> Response { Response::text(200, "text/javascript; charset=utf-8", NOTE_JS).with_header("Cache-Control", CACHE_STATIC) }
pub fn app_js() -> Response { Response::text(200, "text/javascript; charset=utf-8", APP_JS).with_header("Cache-Control", CACHE_STATIC) }
pub fn home_js() -> Response { Response::text(200, "text/javascript; charset=utf-8", HOME_JS).with_header("Cache-Control", CACHE_STATIC) }
pub fn signin_js() -> Response { Response::text(200, "text/javascript; charset=utf-8", SIGNIN_JS).with_header("Cache-Control", CACHE_STATIC) }
pub fn network_js() -> Response { Response::text(200, "text/javascript; charset=utf-8", NETWORK_JS).with_header("Cache-Control", CACHE_STATIC) }
pub fn home_html(config: &Config) -> String { render_home(&config.google_client_id, &login_uri(config), "home") }
pub fn home(config: &Config) -> Response { Response::html(home_html(config)) }
pub fn signin_html(config: &Config) -> String { render_signin(&config.google_client_id, &login_uri(config)) }
pub fn signin(config: &Config) -> Response { Response::html(signin_html(config)) }
pub fn terms() -> Response { Response::html(TERMS_TEMPLATE.to_string()) }
pub fn privacy() -> Response { Response::html(PRIVACY_TEMPLATE.to_string()) }
pub fn guideline() -> Response { Response::html(GUIDELINE_TEMPLATE.to_string()) }
pub fn network() -> Response { Response::html(NETWORK_TEMPLATE.to_string()) }

pub async fn note_page(note_id: &str, state: AppState) -> Result<Response, ApiError<serde_json::Value>> {
    let id_bytes = decode_id(note_id).ok_or_else(|| ApiError::bad_request("invalid_id", "Invalid note id", None))?;
    let note_id = NoteId::from_bytes(id_bytes);
    let chain = fetch_chain(&state, note_id).await?;
    let related = fetch_related(&state, note_id).await?;
    Ok(Response::html(note_html(&state.config, &chain, &related.related)))
}

fn render_home(client_id: &str, login_uri: &str, view: &str) -> String {
    let client_id = escape_attr(client_id); let login_uri = escape_attr(login_uri); let view = escape_attr(view);
    HOME_TEMPLATE.replace("{{CLIENT_ID}}", &client_id).replace("{{LOGIN_URI}}", &login_uri).replace("{{VIEW}}", &view)
}
fn render_signin(client_id: &str, login_uri: &str) -> String {
    let client_id = escape_attr(client_id); let login_uri = escape_attr(login_uri);
    SIGNIN_TEMPLATE.replace("{{CLIENT_ID}}", &client_id).replace("{{LOGIN_URI}}", &login_uri)
}
fn note_html(config: &Config, chain: &NoteChain, related: &[RelatedEntry]) -> String {
    let markdown = chain_markdown(chain);
    let body_html = render_markdown(&markdown);
    let chain_items = render_chain_items(&chain.prev, &chain.next);
    let chain_summary = format!("{} prev, {} next", chain.prev.len(), chain.next.len());
    let related_items = render_related_items(related, &chain.center.id);
    let version_section = render_version_section(related, &chain.center.id);
    let client_id = escape_attr(&config.google_client_id);
    let login_uri = escape_attr(&login_uri(config));
    let note_id_raw = &chain.center.id;
    let note_id = escape_attr(note_id_raw);
    let author_id = escape_attr(&chain.center.author.user_id.to_string());
    let account_note_id = escape_attr(chain.center.author.account_note_id.as_deref().unwrap_or(""));
    let note_description = escape_attr(&note_excerpt(&chain.center.value, 160));
    let note_url = escape_attr(&format!("{}/{}", config.public_base_url, note_id_raw));
    let base = NOTE_TEMPLATE
        .replace("{{CLIENT_ID}}", &client_id)
        .replace("{{LOGIN_URI}}", &login_uri)
        .replace("{{NOTE_ID}}", &note_id)
        .replace("{{NOTE_CREATED_AT}}", &escape_html(&chain.center.created_at))
        .replace("{{NOTE_AUTHOR}}", &escape_html(&chain.center.author.email))
        .replace("{{NOTE_AUTHOR_ID}}", &author_id)
        .replace("{{NOTE_ACCOUNT_NOTE_ID}}", &account_note_id)
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
    for note in &chain.prev { parts.push(note.value.as_str()); }
    parts.push(chain.center.value.as_str());
    for note in &chain.next { parts.push(note.value.as_str()); }
    parts.join("\n\n")
}
fn render_chain_items(prev: &[crate::domain::Note], next: &[crate::domain::Note]) -> String {
    if prev.is_empty() && next.is_empty() { return "<div class=\"empty\">No chained notes.</div>".to_string(); }
    let mut items = Vec::new();
    items.extend(prev.iter().map(|note| render_chain_item(note, "Prev")));
    items.extend(next.iter().map(|note| render_chain_item(note, "Next")));
    items.join("")
}
fn render_chain_item(note: &crate::domain::Note, label: &str) -> String {
    let note_id = escape_attr(&note.id);
    let summary = escape_html(&note_excerpt(&note.value, 120));
    let label = escape_html(label);
    format!("<a class=\"chain-item\" href=\"/{note_id}\"><span class=\"chain-label\">{label}</span><span class=\"chain-text\">{summary}</span></a>")
}
fn render_related_items(related: &[RelatedEntry], center_id: &str) -> String {
    let items: Vec<String> = related.iter().filter(|entry| !matches!(entry.association.kind.as_str(), "next" | "prev")).map(|entry| render_related_item(entry, center_id)).collect();
    if items.is_empty() { return "<div class=\"empty\">No linked notes.</div>".to_string(); }
    items.join("")
}
fn render_related_item(entry: &RelatedEntry, center_id: &str) -> String {
    let note_id = escape_attr(&entry.note.id);
    let summary = escape_html(&note_excerpt(&entry.note.value, 120));
    let created = escape_html(&entry.note.created_at);
    let kind = escape_html(&association_label(&entry.association, center_id));
    let citation = escape_html(&entry.note.id);
    let version_class = if entry.association.kind == "version" { " related-item-version" } else { "" };
    format!("<a class=\"related-item{version_class}\" href=\"/{note_id}\"><span class=\"related-kind\">{kind}</span><span class=\"related-text\">{summary}</span><span class=\"related-meta\">{created}</span><span class=\"related-cite\">Citation: {citation}</span></a>")
}
fn render_version_section(related: &[RelatedEntry], center_id: &str) -> String {
    let items: Vec<String> = related.iter().filter(|entry| entry.association.kind == "version" && entry.association.from_id == center_id).map(|entry| render_related_item(entry, center_id)).collect();
    let hidden = if items.is_empty() { " hidden" } else { "" };
    let body = if items.is_empty() { "".to_string() } else { items.join("") };
    format!("<section class=\"card\" id=\"version-card\"{hidden}><div class=\"card-title\">Newer versions</div><div class=\"related-list\" id=\"version-list\">{body}</div></section>")
}
fn association_label(association: &crate::domain::Association, center_id: &str) -> String {
    match association.kind.as_str() {
        "version" => if association.from_id == center_id { "Newer version".to_string() } else { "Older version".to_string() },
        "reply" => if association.from_id == center_id { "Reply to".to_string() } else { "Reply from".to_string() },
        _ => association.kind.clone(),
    }
}
fn note_excerpt(value: &str, max_len: usize) -> String {
    let mut excerpt = String::new();
    let mut count = 0;
    for ch in value.chars().filter(|ch| *ch != '\n' && *ch != '\r') {
        if count >= max_len { break; }
        excerpt.push(ch); count += 1;
    }
    if excerpt.is_empty() { return "Empty note".to_string(); }
    if value.chars().count() > max_len { excerpt.push_str("..."); }
    excerpt
}
fn render_markdown(value: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES); options.insert(Options::ENABLE_STRIKETHROUGH); options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(value, options).map(|event| match event { Event::Html(html) | Event::InlineHtml(html) => Event::Text(html), other => other });
    let mut output = String::new(); html::push_html(&mut output, parser); output
}
fn escape_attr(value: &str) -> String { value.replace('&', "&amp;").replace('"', "&quot;").replace('<', "&lt;").replace('>', "&gt;").replace('\'', "&#39;") }
fn escape_html(value: &str) -> String { escape_attr(value) }
fn login_uri(config: &Config) -> String { format!("{}/auth/google/redirect", config.public_base_url) }
