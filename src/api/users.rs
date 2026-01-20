use actix_web::{web, HttpResponse};

use crate::api::helpers::parse_uuid;
use crate::errors::ApiError;
use crate::state::AppState;

pub async fn get_user_by_id(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let id_str = path.into_inner();
    if id_str.is_empty() {
        return Err(ApiError::bad_request(
            "missing_user_id",
            "Missing user id",
            None,
        ));
    }
    let user_id = parse_uuid(&id_str, "invalid_user_id", "Invalid user id")?;

    let user = state
        .storage
        .find_user_by_id(user_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(|| ApiError::not_found("user_not_found", "User not found"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"user": user.profile()})))
}
