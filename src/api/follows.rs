use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::api::helpers::{parse_json, parse_query, parse_query_param, parse_uuid, require_user};
use crate::domain::note::format_timestamp;
use crate::domain::Follow;
use crate::errors::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
struct FollowRequest {
    followee_id: String,
}

pub async fn post_follows(
    req: HttpRequest,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let follower = require_user(&req, &state).await?;
    let payload: FollowRequest = parse_json(body.as_ref())?;
    let followee_id = parse_uuid(&payload.followee_id, "invalid_user_id", "Invalid user id")?;
    if followee_id == follower.user_id {
        return Err(ApiError::unprocessable("self_follow", "Cannot follow self", None));
    }

    let followee = state
        .storage
        .find_user_by_id(followee_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(|| ApiError::not_found("user_not_found", "User not found"))?;

    let created_at = state
        .storage
        .create_follow(follower.user_id, followee_id)
        .await
        .map_err(|_| ApiError::internal())?;

    let created_at = created_at
        .ok_or_else(|| ApiError::conflict("already_following", "Already following"))?;

    let follow = Follow {
        follower: follower.profile(),
        followee: followee.profile(),
        created_at: format_timestamp(created_at),
    };
    Ok(HttpResponse::Created().json(follow))
}

pub async fn delete_follows(
    req: HttpRequest,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let follower = require_user(&req, &state).await?;
    let payload: FollowRequest = parse_json(body.as_ref())?;
    let followee_id = parse_uuid(&payload.followee_id, "invalid_user_id", "Invalid user id")?;
    if followee_id == follower.user_id {
        return Err(ApiError::unprocessable(
            "self_unfollow",
            "Cannot unfollow self",
            None,
        ));
    }

    let deleted = state
        .storage
        .delete_follow(follower.user_id, followee_id)
        .await
        .map_err(|_| ApiError::internal())?;

    if !deleted {
        return Err(ApiError::not_found("follow_not_found", "Follow not found"));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({"status": "deleted"})))
}

pub async fn get_follows(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let params = parse_query(&req);
    let user_id = parse_query_param(&params, "user")
        .ok_or_else(|| ApiError::bad_request("missing_user", "Missing user parameter", None))?;
    let direction = parse_query_param(&params, "direction").ok_or_else(|| {
        ApiError::bad_request("missing_direction", "Missing direction parameter", None)
    })?;

    let user_id = parse_uuid(user_id, "invalid_user_id", "Invalid user id")?;

    let edges = match direction {
        "followers" => state
            .storage
            .list_followers(user_id)
            .await
            .map_err(|_| ApiError::internal())?,
        "following" => state
            .storage
            .list_following(user_id)
            .await
            .map_err(|_| ApiError::internal())?,
        _ => {
            return Err(ApiError::bad_request(
                "invalid_direction",
                "Direction must be followers or following",
                None,
            ))
        }
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({"edges": edges})))
}
