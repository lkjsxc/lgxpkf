use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::api::helpers::{parse_json, require_user};
use crate::errors::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
struct AccountNoteRequest {
    value: String,
}

pub async fn post_account_note(
    req: HttpRequest,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let user = require_user(&req, &state).await?;
    let payload: AccountNoteRequest = parse_json(body.as_ref())?;
    let bytes = payload.value.as_bytes();
    if bytes.len() > 1024 {
        return Err(ApiError::unprocessable(
            "value_too_large",
            "Account note exceeds 1024 bytes",
            None,
        ));
    }

    let note = state
        .storage
        .create_account_note(user.user_id, bytes)
        .await
        .map_err(|_| ApiError::internal())?;

    Ok(HttpResponse::Created().json(note))
}
