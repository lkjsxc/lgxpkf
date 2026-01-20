use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::api::helpers::{
    parse_json, parse_limit_param, parse_note_id, parse_query, parse_query_param,
    parse_time_param, parse_uuid, require_user,
};
use crate::domain::Note;
use crate::errors::ApiError;
use crate::state::AppState;

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
    req: HttpRequest,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let user = require_user(&req, &state).await?;
    let payload: CreateNote = parse_json(body.as_ref())?;
    let segments = split_note_value(&payload.value, 1024);
    let bytes: Vec<Vec<u8>> = segments.iter().map(|value| value.as_bytes().to_vec()).collect();
    let account_note_id = user
        .account_note_id
        .as_deref()
        .and_then(|value| parse_note_id(value).ok())
        .ok_or_else(ApiError::internal)?;
    let (root, segments) = state
        .storage
        .create_note_chain(&bytes, user.user_id, account_note_id)
        .await
        .map_err(|_| ApiError::internal())?;

    Ok(HttpResponse::Created().json(PostResponse { root, segments }))
}

pub async fn get_note_by_id(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let note_id = parse_note_id(path.as_str())?;
    let note = state
        .storage
        .find_note(note_id)
        .await
        .map_err(|_| ApiError::internal())?;

    let note = note.ok_or_else(|| ApiError::not_found("note_not_found", "Note not found"))?;
    Ok(HttpResponse::Ok().json(note))
}

pub async fn get_notes(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let params = parse_query(&req);
    let author = match parse_query_param(&params, "author") {
        Some(value) => Some(parse_uuid(value, "invalid_author", "Invalid author id")?),
        None => None,
    };

    let from = parse_time_param(&params, "from")?;
    let to = parse_time_param(&params, "to")?;

    let notes = state
        .storage
        .list_notes(author, from, to)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(HttpResponse::Ok().json(notes))
}

pub async fn get_random_notes(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let params = parse_query(&req);
    let limit = parse_limit_param(&params, "limit", 9, 30)? as i64;
    let notes = state
        .storage
        .list_random_notes(limit)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(HttpResponse::Ok().json(notes))
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
