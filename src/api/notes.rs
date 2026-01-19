use rand::RngCore;
use serde::Deserialize;
use uuid::Uuid;

use crate::api::helpers::{parse_json, parse_query_param, parse_time_param, require_user};
use crate::errors::ApiError;
use crate::http::parser::Request;
use crate::http::response::Response;
use crate::http::router::{parse_query, AppState};
use crate::urls::base32::{decode_id, is_base32_url};
use crate::domain::NoteId;

#[derive(Deserialize)]
struct CreateNote {
    value: String,
}

pub async fn post_notes(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let user = require_user(&req, &state).await?;
    let body: CreateNote = parse_json(&req.body)?;
    let bytes = body.value.as_bytes();
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

    let json = serde_json::to_vec(&note).unwrap_or_else(|_| b"{}".to_vec());
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

fn generate_note_id() -> NoteId {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    NoteId(bytes)
}
