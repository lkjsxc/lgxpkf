use tokio_postgres::Client;
use uuid::Uuid;

use crate::domain::{generate_note_id, Note, User};
use crate::storage::notes::map_note;
use crate::urls::base32::encode_id;

pub async fn find_or_create_user(
    client: &Client,
    google_sub: &str,
    email: &str,
) -> Result<User, Box<dyn std::error::Error>> {
    if let Some(mut user) = find_user_by_sub(client, google_sub).await? {
        if user.account_note_id.is_none() {
            let note = create_account_note(client, user.user_id, account_note_value(&user.email).as_bytes()).await?;
            user.account_note_id = Some(note.id);
        }
        return Ok(user);
    }
    let user_id = Uuid::new_v4();
    client
        .execute(
            "INSERT INTO users (user_id, google_sub, email, created_at) VALUES ($1, $2, $3, NOW())",
            &[&user_id, &google_sub, &email],
        )
        .await?;
    let note = create_account_note(client, user_id, account_note_value(email).as_bytes()).await?;
    Ok(User {
        user_id,
        google_sub: google_sub.to_string(),
        email: email.to_string(),
        account_note_id: Some(note.id),
    })
}

pub async fn find_user_by_id(
    client: &Client,
    user_id: Uuid,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let row = client
        .query_opt(
            "SELECT user_id, google_sub, email, account_note_id FROM users WHERE user_id = $1",
            &[&user_id],
        )
        .await?;
    Ok(row.map(map_user))
}

async fn find_user_by_sub(
    client: &Client,
    google_sub: &str,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let row = client
        .query_opt(
            "SELECT user_id, google_sub, email, account_note_id FROM users WHERE google_sub = $1",
            &[&google_sub],
        )
        .await?;
    Ok(row.map(map_user))
}

pub async fn create_account_note(
    client: &Client,
    user_id: Uuid,
    value: &[u8],
) -> Result<Note, Box<dyn std::error::Error>> {
    let note_id = generate_note_id();
    let id_bytes = note_id.to_bytes();
    let tx = client.transaction().await?;
    tx.execute(
        "INSERT INTO notes (id, value, created_at, author_id) VALUES ($1, $2, NOW(), $3)",
        &[&id_bytes.to_vec(), &value, &user_id],
    )
    .await?;
    tx.execute(
        "UPDATE users SET account_note_id = $1 WHERE user_id = $2",
        &[&id_bytes.to_vec(), &user_id],
    )
    .await?;
    let row = tx
        .query_one(
            "SELECT n.id, n.value, n.created_at, u.user_id, u.email, u.account_note_id \
             FROM notes n JOIN users u ON u.user_id = n.author_id \
             WHERE n.id = $1",
            &[&id_bytes.to_vec()],
        )
        .await?;
    tx.commit().await?;
    Ok(map_note(&row))
}

fn map_user(row: tokio_postgres::Row) -> User {
    let account_note_id = map_account_note_id(row.get(3));
    User {
        user_id: row.get(0),
        google_sub: row.get(1),
        email: row.get(2),
        account_note_id,
    }
}

fn map_account_note_id(value: Option<Vec<u8>>) -> Option<String> {
    let bytes = value?;
    if bytes.len() != 32 {
        return None;
    }
    let mut id = [0u8; 32];
    id.copy_from_slice(&bytes[..32]);
    Some(encode_id(id))
}

fn account_note_value(email: &str) -> String {
    format!("account: {email}")
}
