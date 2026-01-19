use actix_web::{web, HttpRequest, HttpResponse};
use rand::RngCore;
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::NoteId;
use crate::errors::ApiError;
use crate::handlers::helpers::{parse_json, parse_time_param, query_param, require_user};
use crate::state::AppState;
use crate::urls::base32::{decode_id, is_base32_url};

#[derive(Deserialize)]
struct CreateNote {
    value: String,
}

pub async fn post_notes(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Bytes,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let user = require_user(&req, &state).await?;
    let payload: CreateNote = parse_json(&body)?;
    let bytes = payload.value.as_bytes();
    if bytes.len() > 1024 {
        return Err(ApiError::unprocessable(
            "value_too_large",
            "Note value exceeds 1024 bytes",
            None,
        ));
    }

    let note_id = generate_note_id();
    let note = state
        .storage
        .create_note(note_id, bytes, user.user_id)
        .await
        .map_err(|_| ApiError::internal())?;

    Ok(HttpResponse::Created().json(note))
}

pub async fn get_note_by_id(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let id_str = path.into_inner();
    if id_str.is_empty() || !is_base32_url(&id_str) {
        return Err(ApiError::bad_request("invalid_id", "Invalid note id", None));
    }
    let id_bytes = decode_id(&id_str).ok_or_else(|| {
        ApiError::bad_request("invalid_id", "Invalid note id", None)
    })?;

    let note = state
        .storage
        .find_note(NoteId::from_bytes(id_bytes))
        .await
        .map_err(|_| ApiError::internal())?;

    let note = note.ok_or_else(|| ApiError::not_found("note_not_found", "Note not found"))?;
    Ok(HttpResponse::Ok().json(note))
}

pub async fn get_notes(
    query: web::Query<HashMap<String, String>>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let params = query.into_inner();
    let author = query_param(&params, "author")
        .map(|v| v.parse::<Uuid>().ok())
        .unwrap_or(None);
    if query_param(&params, "author").is_some() && author.is_none() {
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
    Ok(HttpResponse::Ok().json(notes))
}

fn generate_note_id() -> NoteId {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    NoteId(bytes)
}
