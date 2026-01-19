use serde::Deserialize;

use crate::api::helpers::{parse_json, require_user};
use crate::errors::ApiError;
use crate::http::parser::Request;
use crate::http::response::Response;
use crate::state::AppState;

#[derive(Deserialize)]
struct AccountNoteRequest {
    value: String,
}

pub async fn post_account_note(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let user = require_user(&req, &state).await?;
    let payload: AccountNoteRequest = parse_json(&req.body)?;
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

    let json = serde_json::to_vec(&note).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(201, json))
}
