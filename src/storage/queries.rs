use tokio_postgres::Client;
use uuid::Uuid;

use crate::domain::{Association, Note, NoteId, User, UserProfile};
use crate::domain::note::format_timestamp;
use crate::urls::base32::encode_id;

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

pub async fn create_note(
    client: &Client,
    note_id: NoteId,
    value: &[u8],
    author_id: Uuid,
) -> Result<Note, Box<dyn std::error::Error>> {
    let id_bytes = note_id.to_bytes();
    client
        .execute(
            "INSERT INTO notes (id, value, created_at, author_id) VALUES ($1, $2, NOW(), $3)",
            &[&id_bytes.to_vec(), &value, &author_id],
        )
        .await?;
    let row = client
        .query_one(
            "SELECT n.id, n.value, n.created_at, u.user_id, u.email FROM notes n JOIN users u ON u.user_id = n.author_id WHERE n.id = $1",
            &[&id_bytes.to_vec()],
        )
        .await?;
    Ok(map_note(&row))
}

pub async fn find_note(
    client: &Client,
    note_id: NoteId,
) -> Result<Option<Note>, Box<dyn std::error::Error>> {
    let id_bytes = note_id.to_bytes();
    let row = client
        .query_opt(
            "SELECT n.id, n.value, n.created_at, u.user_id, u.email FROM notes n JOIN users u ON u.user_id = n.author_id WHERE n.id = $1",
            &[&id_bytes.to_vec()],
        )
        .await?;
    Ok(row.map(|r| map_note(&r)))
}

pub async fn list_notes(
    client: &Client,
    author: Option<Uuid>,
    from: Option<time::OffsetDateTime>,
    to: Option<time::OffsetDateTime>,
) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let mut clauses = Vec::new();
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

    if let Some(author_id) = author.as_ref() {
        clauses.push(format!("author_id = ${}", params.len() + 1));
        params.push(author_id);
    }
    if let Some(from_ts) = from.as_ref() {
        clauses.push(format!("created_at >= ${}", params.len() + 1));
        params.push(from_ts);
    }
    if let Some(to_ts) = to.as_ref() {
        clauses.push(format!("created_at <= ${}", params.len() + 1));
        params.push(to_ts);
    }

    let where_clause = if clauses.is_empty() {
        "".to_string()
    } else {
        format!("WHERE {}", clauses.join(" AND "))
    };
    let query = format!(
        "SELECT n.id, n.value, n.created_at, u.user_id, u.email FROM notes n JOIN users u ON u.user_id = n.author_id {} ORDER BY n.created_at DESC",
        where_clause
    );

    let rows = client.query(&query, &params).await?;
    Ok(rows.iter().map(map_note).collect())
}

pub async fn create_association(
    client: &Client,
    kind: &str,
    from_id: NoteId,
    to_id: NoteId,
) -> Result<Association, Box<dyn std::error::Error>> {
    let from_bytes = from_id.to_bytes();
    let to_bytes = to_id.to_bytes();
    client
        .execute(
            "INSERT INTO associations (id, kind, from_id, to_id, created_at) VALUES ($1, $2, $3, $4, NOW())",
            &[&Uuid::new_v4(), &kind, &from_bytes.to_vec(), &to_bytes.to_vec()],
        )
        .await?;

    let row = client
        .query_one(
            "SELECT kind, from_id, to_id, created_at FROM associations WHERE from_id = $1 AND to_id = $2 ORDER BY created_at DESC LIMIT 1",
            &[&from_bytes.to_vec(), &to_bytes.to_vec()],
        )
        .await?;
    Ok(map_association(&row))
}

pub async fn list_associations(
    client: &Client,
    note_id: NoteId,
) -> Result<Vec<Association>, Box<dyn std::error::Error>> {
    let id_bytes = note_id.to_bytes();
    let rows = client
        .query(
            "SELECT kind, from_id, to_id, created_at FROM associations WHERE from_id = $1 OR to_id = $1 ORDER BY created_at DESC",
            &[&id_bytes.to_vec()],
        )
        .await?;
    Ok(rows.iter().map(map_association).collect())
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
// Deprecated: module superseded by split storage modules.
