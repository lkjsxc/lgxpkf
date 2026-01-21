use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::api::helpers::{
    parse_json, parse_note_id, parse_note_reference, parse_query, parse_query_param, require_user,
};
use crate::domain::{Note, User};
use crate::errors::ApiError;
use crate::state::AppState;
use crate::storage::AssociationInsertError;

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
    let user = require_user(&req, &state).await?;
    let payload: CreateAssociation = parse_json(body.as_ref())?;
    let kind = parse_kind(&payload.kind)?.to_ascii_lowercase();
    if !is_allowed_kind(&kind) {
        return Err(ApiError::unprocessable(
            "invalid_kind",
            "Association kind is not allowed",
            None,
        ));
    }

    let from_id = parse_note_id(&payload.from_id)?;
    let to_id = parse_note_reference(&payload.to_id)?;
    if from_id == to_id {
        return Err(ApiError::unprocessable(
            "invalid_target",
            "Source and target must differ",
            None,
        ));
    }

    let from_note = state
        .storage
        .find_note(from_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(|| ApiError::not_found("note_not_found", "Note not found"))?;

    let to_note = state
        .storage
        .find_note(to_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(|| ApiError::not_found("note_not_found", "Note not found"))?;

    ensure_association_allowed(&kind, &from_note, &to_note, &user)?;

    let association = state
        .storage
        .create_association(&kind, from_id, to_id)
        .await
        .map_err(|err| {
            if let Some(AssociationInsertError::VersionExists) =
                err.downcast_ref::<AssociationInsertError>()
            {
                return ApiError::conflict(
                    "version_exists",
                    "Newer version already exists for this note",
                );
            }
            ApiError::internal()
        })?;
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

fn is_allowed_kind(kind: &str) -> bool {
    matches!(
        kind,
        "link" | "reply" | "quote" | "parent" | "child" | "next" | "prev" | "version"
    )
}

fn allows_cross_author(kind: &str) -> bool {
    matches!(kind, "link" | "reply" | "quote")
}

fn is_account_note(user: &User, note: &Note) -> bool {
    user.account_note_id
        .as_deref()
        .is_some_and(|id| id == note.id)
}

fn ensure_association_allowed(
    kind: &str,
    from_note: &Note,
    to_note: &Note,
    user: &User,
) -> Result<(), ApiError<serde_json::Value>> {
    if from_note.author.user_id != user.user_id {
        return Err(ApiError::forbidden(
            "association_forbidden",
            "Cannot create associations for this note",
        ));
    }
    if is_account_note(user, from_note) {
        return Err(ApiError::unprocessable(
            "account_note_locked",
            "Account notes cannot create associations",
            None,
        ));
    }
    if kind == "version" && is_account_note(user, to_note) {
        return Err(ApiError::unprocessable(
            "account_note_locked",
            "Account notes cannot be versioned",
            None,
        ));
    }
    if !allows_cross_author(kind) && from_note.author.user_id != to_note.author.user_id {
        return Err(ApiError::forbidden(
            "association_forbidden",
            "Cannot link notes owned by different authors",
        ));
    }
    Ok(())
}
