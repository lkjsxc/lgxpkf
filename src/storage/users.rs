use tokio_postgres::Client;
use uuid::Uuid;

use crate::domain::User;

pub async fn find_or_create_user(
    client: &Client,
    google_sub: &str,
    email: &str,
) -> Result<User, Box<dyn std::error::Error>> {
    if let Some(user) = find_user_by_sub(client, google_sub).await? {
        return Ok(user);
    }
    let user_id = Uuid::new_v4();
    client
        .execute(
            "INSERT INTO users (user_id, google_sub, email, created_at) VALUES ($1, $2, $3, NOW())",
            &[&user_id, &google_sub, &email],
        )
        .await?;
    Ok(User {
        user_id,
        google_sub: google_sub.to_string(),
        email: email.to_string(),
    })
}

pub async fn find_user_by_id(
    client: &Client,
    user_id: Uuid,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let row = client
        .query_opt(
            "SELECT user_id, google_sub, email FROM users WHERE user_id = $1",
            &[&user_id],
        )
        .await?;
    Ok(row.map(|r| User {
        user_id: r.get(0),
        google_sub: r.get(1),
        email: r.get(2),
    }))
}

async fn find_user_by_sub(
    client: &Client,
    google_sub: &str,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let row = client
        .query_opt(
            "SELECT user_id, google_sub, email FROM users WHERE google_sub = $1",
            &[&google_sub],
        )
        .await?;
    Ok(row.map(|r| User {
        user_id: r.get(0),
        google_sub: r.get(1),
        email: r.get(2),
    }))
}
