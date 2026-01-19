use crate::domain::NoteId;
use crate::errors::ApiError;
use crate::http::parser::Request;
use crate::http::response::Response;
use crate::related::fetch_related;
use crate::state::AppState;
use crate::urls::base32::{decode_id, is_base32_url};

pub async fn get_related(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let id_str = req
        .path
        .strip_prefix("/notes/")
        .and_then(|path| path.strip_suffix("/related"))
        .ok_or_else(|| ApiError::bad_request("invalid_id", "Invalid note id", None))?;
    if id_str.is_empty() || !is_base32_url(id_str) {
        return Err(ApiError::bad_request("invalid_id", "Invalid note id", None));
    }
    let id_bytes = decode_id(id_str)
        .ok_or_else(|| ApiError::bad_request("invalid_id", "Invalid note id", None))?;

    let response = fetch_related(&state, NoteId::from_bytes(id_bytes)).await?;
    let json = serde_json::to_vec(&response).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(200, json))
}
