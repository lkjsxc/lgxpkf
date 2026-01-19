use serde::Deserialize;

use crate::api::helpers::{parse_json, parse_query_param, require_user};
use crate::domain::NoteId;
use crate::errors::ApiError;
use crate::http::parser::Request;
use crate::http::response::Response;
use crate::http::router::parse_query;
use crate::state::AppState;
use crate::urls::base32::{decode_id, is_base32_url};

#[derive(Deserialize)]
struct CreateAssociation {
    kind: String,
    from_id: String,
    to_id: String,
}

pub async fn post_associations(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    require_user(&req, &state).await?;
    let body: CreateAssociation = parse_json(&req.body)?;
    if body.kind.trim().is_empty() {
        return Err(ApiError::bad_request(
            "invalid_kind",
            "Association kind required",
            None,
        ));
    }

    let from_id = parse_note_id(&body.from_id)?;
    let to_id = parse_note_id(&body.to_id)?;

    let association = state
        .storage
        .create_association(&body.kind, from_id, to_id)
        .await
        .map_err(|_| ApiError::internal())?;
    let json = serde_json::to_vec(&association).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(201, json))
}

pub async fn get_associations(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let params = parse_query(req.query.as_deref());
    let note_id = parse_query_param(&params, "note").ok_or_else(|| {
        ApiError::bad_request("missing_note", "Missing note parameter", None)
    })?;
    let note_id = parse_note_id(note_id)?;

    let associations = state
        .storage
        .list_associations(note_id)
        .await
        .map_err(|_| ApiError::internal())?;
    let json = serde_json::to_vec(&associations).unwrap_or_else(|_| b"[]".to_vec());
    Ok(Response::json(200, json))
}

fn parse_note_id(value: &str) -> Result<NoteId, ApiError<serde_json::Value>> {
    if value.is_empty() || !is_base32_url(value) {
        return Err(ApiError::bad_request(
            "invalid_id",
            "Invalid note id",
            None,
        ));
    }
    decode_id(value)
        .map(NoteId::from_bytes)
        .ok_or_else(|| ApiError::bad_request("invalid_id", "Invalid note id", None))
}
