use crate::api::helpers::{parse_limit_param, parse_time_param, require_user};
use crate::errors::ApiError;
use crate::http::parser::Request;
use crate::http::response::Response;
use crate::http::router::parse_query;
use crate::state::AppState;

pub async fn get_feed(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let user = require_user(&req, &state).await?;
    let params = parse_query(req.query.as_deref());
    let from = parse_time_param(&params, "from")?;
    let to = parse_time_param(&params, "to")?;
    let limit = parse_limit_param(&params, "limit", 50, 200)?;

    let notes = state
        .storage
        .list_feed_notes(user.user_id, from, to, limit as i64)
        .await
        .map_err(|_| ApiError::internal())?;
    let json = serde_json::to_vec(&notes).unwrap_or_else(|_| b"[]".to_vec());
    Ok(Response::json(200, json))
}
