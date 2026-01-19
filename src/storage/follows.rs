use tokio_postgres::Client;
use uuid::Uuid;

use crate::domain::{FollowEdge, UserProfile};
use crate::domain::note::format_timestamp;

pub async fn create_follow(
    client: &Client,
    follower_id: Uuid,
    followee_id: Uuid,
) -> Result<Option<time::OffsetDateTime>, Box<dyn std::error::Error>> {
    let row = client
        .query_opt(
            "INSERT INTO follows (follower_id, followee_id, created_at) VALUES ($1, $2, NOW())\n             ON CONFLICT (follower_id, followee_id) DO NOTHING\n             RETURNING created_at",
            &[&follower_id, &followee_id],
        )
        .await?;
    Ok(row.map(|r| r.get(0)))
}

pub async fn delete_follow(
    client: &Client,
    follower_id: Uuid,
    followee_id: Uuid,
) -> Result<bool, Box<dyn std::error::Error>> {
    let count = client
        .execute(
            "DELETE FROM follows WHERE follower_id = $1 AND followee_id = $2",
            &[&follower_id, &followee_id],
        )
        .await?;
    Ok(count > 0)
}

pub async fn list_followers(
    client: &Client,
    user_id: Uuid,
) -> Result<Vec<FollowEdge>, Box<dyn std::error::Error>> {
    let rows = client
        .query(
            "SELECT u.user_id, u.email, f.created_at\n             FROM follows f\n             JOIN users u ON u.user_id = f.follower_id\n             WHERE f.followee_id = $1\n             ORDER BY f.created_at DESC",
            &[&user_id],
        )
        .await?;
    Ok(rows.iter().map(map_edge).collect())
}

pub async fn list_following(
    client: &Client,
    user_id: Uuid,
) -> Result<Vec<FollowEdge>, Box<dyn std::error::Error>> {
    let rows = client
        .query(
            "SELECT u.user_id, u.email, f.created_at\n             FROM follows f\n             JOIN users u ON u.user_id = f.followee_id\n             WHERE f.follower_id = $1\n             ORDER BY f.created_at DESC",
            &[&user_id],
        )
        .await?;
    Ok(rows.iter().map(map_edge).collect())
}

fn map_edge(row: &tokio_postgres::Row) -> FollowEdge {
    let user_id: Uuid = row.get(0);
    let email: String = row.get(1);
    let created_at: time::OffsetDateTime = row.get(2);

    FollowEdge {
        user: UserProfile { user_id, email },
        created_at: format_timestamp(created_at),
    }
}
