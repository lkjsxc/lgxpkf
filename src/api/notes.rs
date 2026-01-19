use rand::RngCore;
use serde::Deserialize;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

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
    let author = parse_param(&params, "author")
        .map(|v| v.parse::<Uuid>().ok())
        .unwrap_or(None);
    if parse_param(&params, "author").is_some() && author.is_none() {
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

fn parse_param<'a>(params: &'a [(String, String)], key: &str) -> Option<&'a str> {
    params.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
}

fn parse_time_param(
    params: &[(String, String)],
    key: &str,
) -> Result<Option<OffsetDateTime>, ApiError<serde_json::Value>> {
    if let Some(value) = parse_param(params, key) {
        let parsed = OffsetDateTime::parse(value, &Rfc3339).map_err(|_| {
            ApiError::bad_request("invalid_timestamp", "Invalid timestamp", None)
        })?;
        Ok(Some(parsed))
    } else {
        Ok(None)
    }
}

fn generate_note_id() -> NoteId {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    NoteId(bytes)
}

async fn require_user(
    req: &Request,
    state: &AppState,
) -> Result<crate::domain::User, ApiError<serde_json::Value>> {
    let header = req
        .headers
        .get("authorization")
        .ok_or_else(|| ApiError::unauthorized("unauthorized", "Missing authorization token"))?;
    let mut parts = header.split_whitespace();
    let token = match (parts.next(), parts.next()) {
        (Some(scheme), Some(token)) if scheme.eq_ignore_ascii_case("bearer") => token.to_string(),
        _ => {
            return Err(ApiError::unauthorized(
                "unauthorized",
                "Missing authorization token",
            ))
        }
    };

    let user = state
        .storage
        .get_session_user(&token)
        .await
        .map_err(|_| ApiError::internal())?;

    user.ok_or_else(|| ApiError::unauthorized("unauthorized", "Invalid session"))
}

fn parse_json<T: serde::de::DeserializeOwned>(
    body: &[u8],
) -> Result<T, ApiError<serde_json::Value>> {
    serde_json::from_slice(body).map_err(|_| {
        ApiError::bad_request("invalid_json", "Invalid JSON body", None)
    })
}
