use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::helpers::{parse_json, parse_query_param, parse_time_param, require_user};
use crate::domain::Note;
use crate::errors::ApiError;
use crate::http::parser::Request;
use crate::http::response::Response;
use crate::http::router::parse_query;
use crate::state::AppState;
use crate::urls::base32::{decode_id, is_base32_url};
use crate::domain::NoteId;

#[derive(Deserialize)]
struct CreateNote {
    value: String,
}

#[derive(Serialize)]
struct PostResponse {
    root: Note,
    segments: Vec<String>,
}

pub async fn post_notes(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let user = require_user(&req, &state).await?;
    let body: CreateNote = parse_json(&req.body)?;
    let segments = split_note_value(&body.value, 1024);
    let bytes: Vec<Vec<u8>> = segments.iter().map(|value| value.as_bytes().to_vec()).collect();
    let account_note_id = user
        .account_note_id
        .as_deref()
        .and_then(decode_id)
        .map(NoteId::from_bytes)
        .ok_or_else(ApiError::internal)?;
    let (root, segments) = state
        .storage
        .create_note_chain(&bytes, user.user_id, account_note_id)
        .await
        .map_err(|_| ApiError::internal())?;

    let response = PostResponse { root, segments };
    let json = serde_json::to_vec(&response).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(201, json))
}

pub async fn get_note_by_id(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let id_str = req.path.trim_start_matches("/notes/");
    if id_str.is_empty() || !is_base32_url(id_str) {
        return Err(ApiError::bad_request(
            "invalid_id",
            "Invalid note id",
            None,
        ));
    }
    let id_bytes = decode_id(id_str).ok_or_else(|| {
        ApiError::bad_request("invalid_id", "Invalid note id", None)
    })?;

    let note = state
        .storage
        .find_note(NoteId::from_bytes(id_bytes))
        .await
        .map_err(|_| ApiError::internal())?;

    let note = note.ok_or_else(|| ApiError::not_found("note_not_found", "Note not found"))?;
    let json = serde_json::to_vec(&note).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(200, json))
}

pub async fn get_notes(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let params = parse_query(req.query.as_deref());
    let author = parse_query_param(&params, "author")
        .map(|v| v.parse::<Uuid>().ok())
        .unwrap_or(None);
    if parse_query_param(&params, "author").is_some() && author.is_none() {
        return Err(ApiError::bad_request(
            "invalid_author",
            "Invalid author id",
            None,
        ));
    }

    let from = parse_time_param(&params, "from")?;
    let to = parse_time_param(&params, "to")?;

    let notes = state
        .storage
        .list_notes(author, from, to)
        .await
        .map_err(|_| ApiError::internal())?;
    let json = serde_json::to_vec(&notes).unwrap_or_else(|_| b"[]".to_vec());
    Ok(Response::json(200, json))
}

fn split_note_value(value: &str, max_bytes: usize) -> Vec<String> {
    if value.is_empty() {
        return vec![String::new()];
    }
    let mut chunks = Vec::new();
    let mut start = 0;
    let mut current = 0;
    for (idx, ch) in value.char_indices() {
        let ch_len = ch.len_utf8();
        if current + ch_len > max_bytes {
            if start < idx {
                chunks.push(value[start..idx].to_string());
            }
            start = idx;
            current = 0;
        }
        current += ch_len;
    }
    if start < value.len() {
        chunks.push(value[start..].to_string());
    }
    chunks
}
