use tokio_postgres::Client;
use uuid::Uuid;

use crate::domain::{Note, NoteId, UserProfile};
use crate::domain::note::format_timestamp;
use crate::urls::base32::encode_id;

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

fn map_note(row: &tokio_postgres::Row) -> Note {
    let id_bytes: Vec<u8> = row.get(0);
    let value_bytes: Vec<u8> = row.get(1);
    let created_at: time::OffsetDateTime = row.get(2);
    let author_id: Uuid = row.get(3);
    let email: String = row.get(4);

    let mut id = [0u8; 32];
    id.copy_from_slice(&id_bytes[..32]);

    Note {
        id: encode_id(id),
        value: String::from_utf8_lossy(&value_bytes).to_string(),
        created_at: format_timestamp(created_at),
        author: UserProfile { user_id: author_id, email },
    }
}
