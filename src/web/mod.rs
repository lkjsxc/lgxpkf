use crate::config::Config;
use crate::domain::NoteId;
use crate::errors::ApiError;
use crate::http::response::Response;
use crate::related::{fetch_related, RelatedEntry};
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
    let payload = fetch_related(&state, NoteId::from_bytes(id_bytes)).await?;
    Ok(Response::html(note_html(&payload.center, &payload.related)))
}

fn render_home(client_id: &str) -> String {
    let client_id = escape_attr(client_id);
    HOME_TEMPLATE.replace("{{CLIENT_ID}}", &client_id)
}

fn note_html(note: &crate::domain::Note, related: &[RelatedEntry]) -> String {
    let related_html = if related.is_empty() {
        "<div class=\"empty\">No related posts yet.</div>".to_string()
    } else {
        related
            .iter()
            .map(render_related_item)
            .collect::<Vec<String>>()
            .join("")
    };

    NOTE_TEMPLATE
        .replace("{{NOTE_ID}}", &escape_attr(&note.id))
        .replace("{{NOTE_CREATED_AT}}", &escape_html(&note.created_at))
        .replace("{{NOTE_AUTHOR}}", &escape_html(&note.author.email))
        .replace("{{NOTE_VALUE}}", &escape_html(&note.value))
        .replace("{{RELATED_ITEMS}}", &related_html)
}

fn render_related_item(entry: &RelatedEntry) -> String {
    let note_id = escape_attr(&entry.note.id);
    let note_value = escape_html(&entry.note.value);
    let kind = escape_html(&entry.association.kind);
    let created_at = escape_html(&entry.note.created_at);
    format!(
        "<article class=\"note-card\">\
         <div class=\"note-meta\">\
         <span class=\"pill\">{kind}</span>\
         <span>{created_at}</span>\
         <span class=\"mono\">{note_id}</span>\
         </div>\
         <div class=\"note-value\">{note_value}</div>\
         <div class=\"note-link\"><a href=\"/{note_id}\">Open note</a></div>\
         </article>"
    )
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
