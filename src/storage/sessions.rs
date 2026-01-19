use tokio_postgres::Client;
use uuid::Uuid;

use crate::domain::User;
use crate::storage::StorageError;

pub async fn create_session(
    client: &Client,
    user_id: Uuid,
    token: &str,
    expires_at: time::OffsetDateTime,
) -> Result<(), StorageError> {
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
) -> Result<Option<User>, StorageError> {
    let row = client
        .query_opt(
            "SELECT u.user_id, u.google_sub, u.email, u.account_note_id \
             FROM sessions s JOIN users u ON u.user_id = s.user_id \
             WHERE s.token = $1 AND s.expires_at > NOW()",
            &[&token],
        )
        .await?;
    Ok(row.map(|r| User {
        user_id: r.get(0),
        google_sub: r.get(1),
        email: r.get(2),
        account_note_id: map_account_note_id(r.get(3)),
    }))
}

fn map_account_note_id(value: Option<Vec<u8>>) -> Option<String> {
    let bytes = value?;
    if bytes.len() != 32 {
        return None;
    }
    let mut id = [0u8; 32];
    id.copy_from_slice(&bytes[..32]);
    Some(crate::urls::base32::encode_id(id))
}
