use serde::Deserialize;
use uuid::Uuid;

use crate::api::helpers::{parse_json, parse_query_param, require_user};
use crate::domain::Follow;
use crate::domain::note::format_timestamp;
use crate::errors::ApiError;
use crate::http::parser::Request;
use crate::http::response::Response;
use crate::http::router::parse_query;
use crate::state::AppState;

#[derive(Deserialize)]
struct FollowRequest {
    followee_id: String,
}

pub async fn post_follows(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let follower = require_user(&req, &state).await?;
    let body: FollowRequest = parse_json(&req.body)?;
    let followee_id = parse_uuid(&body.followee_id)?;
    if followee_id == follower.user_id {
        return Err(ApiError::unprocessable(
            "self_follow",
            "Cannot follow self",
            None,
        ));
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

    let created_at = created_at.ok_or_else(|| {
        ApiError::conflict("already_following", "Already following")
    })?;

    let follow = Follow {
        follower: follower.profile(),
        followee: followee.profile(),
        created_at: format_timestamp(created_at),
    };
    let json = serde_json::to_vec(&follow).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(201, json))
}

pub async fn delete_follows(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let follower = require_user(&req, &state).await?;
    let body: FollowRequest = parse_json(&req.body)?;
    let followee_id = parse_uuid(&body.followee_id)?;
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

    let payload = serde_json::json!({"status": "deleted"});
    let json = serde_json::to_vec(&payload).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(200, json))
}

pub async fn get_follows(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let params = parse_query(req.query.as_deref());
    let user_id = parse_query_param(&params, "user")
        .ok_or_else(|| ApiError::bad_request("missing_user", "Missing user parameter", None))?;
    let direction = parse_query_param(&params, "direction").ok_or_else(|| {
        ApiError::bad_request("missing_direction", "Missing direction parameter", None)
    })?;

    let user_id = parse_uuid(user_id)?;

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

    let payload = serde_json::json!({"edges": edges});
    let json = serde_json::to_vec(&payload).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(200, json))
}

fn parse_uuid(value: &str) -> Result<Uuid, ApiError<serde_json::Value>> {
    Uuid::parse_str(value).map_err(|_| {
        ApiError::bad_request("invalid_user_id", "Invalid user id", None)
    })
}
