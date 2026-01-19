use tokio_postgres::Client;
use uuid::Uuid;

use crate::domain::User;

pub async fn create_session(
    client: &Client,
    user_id: Uuid,
    token: &str,
    expires_at: time::OffsetDateTime,
) -> Result<(), Box<dyn std::error::Error>> {
    client
        .execute(
            "INSERT INTO sessions (token, user_id, expires_at, created_at) VALUES ($1, $2, $3, NOW())",
            &[&token, &user_id, &expires_at],
        )
        .await?;
    Ok(())
}

pub async fn get_session_user(
    client: &Client,
    token: &str,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let row = client
        .query_opt(
            "SELECT u.user_id, u.google_sub, u.email FROM sessions s JOIN users u ON u.user_id = s.user_id WHERE s.token = $1 AND s.expires_at > NOW()",
            &[&token],
        )
        .await?;
    Ok(row.map(|r| User {
        user_id: r.get(0),
        google_sub: r.get(1),
        email: r.get(2),
    }))
}
