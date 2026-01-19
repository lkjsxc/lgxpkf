use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use std::collections::HashMap;

use crate::domain::NoteId;
use crate::errors::ApiError;
use crate::handlers::helpers::{parse_json, query_param, require_user};
use crate::state::AppState;
use crate::urls::base32::{decode_id, is_base32_url};

#[derive(Deserialize)]
struct CreateAssociation {
    kind: String,
    from_id: String,
    to_id: String,
}

pub async fn post_associations(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Bytes,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    require_user(&req, &state).await?;
    let payload: CreateAssociation = parse_json(&body)?;
    if payload.kind.trim().is_empty() {
        return Err(ApiError::bad_request(
            "invalid_kind",
            "Association kind required",
            None,
        ));
    }

    let from_id = parse_note_id(&payload.from_id)?;
    let to_id = parse_note_id(&payload.to_id)?;

    let association = state
        .storage
        .create_association(&payload.kind, from_id, to_id)
        .await
        .map_err(|_| ApiError::internal())?;

    Ok(HttpResponse::Created().json(association))
}

pub async fn get_associations(
    query: web::Query<HashMap<String, String>>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let params = query.into_inner();
    let note_id = query_param(&params, "note").ok_or_else(|| {
        ApiError::bad_request("missing_note", "Missing note parameter", None)
    })?;
    let note_id = parse_note_id(note_id)?;

    let associations = state
        .storage
        .list_associations(note_id)
        .await
        .map_err(|_| ApiError::internal())?;

    Ok(HttpResponse::Ok().json(associations))
}

fn parse_note_id(value: &str) -> Result<NoteId, ApiError<serde_json::Value>> {
    if value.is_empty() || !is_base32_url(value) {
        return Err(ApiError::bad_request("invalid_id", "Invalid note id", None));
    }
    decode_id(value)
        .map(NoteId::from_bytes)
        .ok_or_else(|| ApiError::bad_request("invalid_id", "Invalid note id", None))
}
