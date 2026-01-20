use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::api::helpers::{parse_json, parse_note_id, parse_query, parse_query_param, require_user};
use crate::errors::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
struct CreateAssociation {
    kind: String,
    from_id: String,
    to_id: String,
}

pub async fn post_associations(
    req: HttpRequest,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    require_user(&req, &state).await?;
    let payload: CreateAssociation = parse_json(body.as_ref())?;
    let kind = parse_kind(&payload.kind)?;

    let from_id = parse_note_id(&payload.from_id)?;
    let to_id = parse_note_id(&payload.to_id)?;
    if kind == "version" {
        let locked = state
            .storage
            .is_account_note_id(from_id)
            .await
            .map_err(|_| ApiError::internal())?;
        if locked {
            return Err(ApiError::unprocessable(
                "account_note_locked",
                "Account bootstrap notes cannot be versioned",
                None,
            ));
        }
    }

    let association = state
        .storage
        .create_association(&kind, from_id, to_id)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(HttpResponse::Created().json(association))
}

pub async fn get_associations(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let params = parse_query(&req);
    let note_id = parse_query_param(&params, "note")
        .ok_or_else(|| ApiError::bad_request("missing_note", "Missing note parameter", None))?;
    let note_id = parse_note_id(note_id)?;

    let associations = state
        .storage
        .list_associations(note_id)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(HttpResponse::Ok().json(associations))
}

fn parse_kind(value: &str) -> Result<String, ApiError<serde_json::Value>> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ApiError::bad_request(
            "invalid_kind",
            "Association kind required",
            None,
        ));
    }
    if trimmed.chars().any(|c| c.is_whitespace()) {
        return Err(ApiError::bad_request(
            "invalid_kind",
            "Association kind must be a single token",
            None,
        ));
    }
    Ok(trimmed.to_string())
}
