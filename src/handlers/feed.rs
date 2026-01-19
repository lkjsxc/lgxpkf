use actix_web::{web, HttpRequest, HttpResponse};
use std::collections::HashMap;

use crate::errors::ApiError;
use crate::handlers::helpers::{parse_limit_param, parse_time_param, require_user};
use crate::state::AppState;

pub async fn get_feed(
    req: HttpRequest,
    query: web::Query<HashMap<String, String>>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let user = require_user(&req, &state).await?;
    let params = query.into_inner();
    let from = parse_time_param(&params, "from")?;
    let to = parse_time_param(&params, "to")?;
    let limit = parse_limit_param(&params, "limit", 50, 200)?;

    let notes = state
        .storage
        .list_feed_notes(user.user_id, from, to, limit as i64)
        .await
        .map_err(|_| ApiError::internal())?;

    Ok(HttpResponse::Ok().json(notes))
}
