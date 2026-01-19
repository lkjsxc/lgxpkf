use uuid::Uuid;

use crate::errors::ApiError;
use crate::http::parser::Request;
use crate::http::response::Response;
use crate::http::router::AppState;

pub async fn get_user_by_id(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let id_str = req.path.trim_start_matches("/users/");
    if id_str.is_empty() {
        return Err(ApiError::bad_request(
            "missing_user_id",
            "Missing user id",
            None,
        ));
    }
    let user_id = Uuid::parse_str(id_str).map_err(|_| {
        ApiError::bad_request("invalid_user_id", "Invalid user id", None)
    })?;

    let user = state
        .storage
        .find_user_by_id(user_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(|| ApiError::not_found("user_not_found", "User not found"))?;

    let payload = serde_json::json!({"user": user.profile()});
    let json = serde_json::to_vec(&payload).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(200, json))
}
