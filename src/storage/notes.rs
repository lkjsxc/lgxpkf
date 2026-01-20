use tokio_postgres::{Client, GenericClient};
use uuid::Uuid;
use crate::domain::{Note, NoteId, UserProfile};
use crate::domain::note::format_timestamp;
use crate::storage::StorageError;
use crate::urls::base32::encode_id;
pub async fn create_note<C>(
    client: &C,
    note_id: NoteId,
    value: &[u8],
    author_id: Uuid,
) -> Result<Note, StorageError>
where
    C: GenericClient + Sync,
{
    insert_note(client, note_id, value, author_id).await?;
    let id_bytes = note_id.to_bytes();
    let row = client
        .query_one(
            "SELECT n.id, n.value, n.created_at, u.user_id, u.email, u.account_note_id \
             FROM notes n JOIN users u ON u.user_id = n.author_id \
             WHERE n.id = $1",
            &[&id_bytes.to_vec()],
        )
        .await?;
    Ok(map_note(&row))
}
pub async fn insert_note<C>(
    client: &C,
    note_id: NoteId,
    value: &[u8],
    author_id: Uuid,
) -> Result<(), StorageError>
where
    C: GenericClient + Sync,
{
    let id_bytes = note_id.to_bytes();
    client
        .execute(
            "INSERT INTO notes (id, value, created_at, author_id) VALUES ($1, $2, NOW(), $3)",
            &[&id_bytes.to_vec(), &value, &author_id],
        )
        .await?;
    Ok(())
}
pub async fn find_note(
    client: &Client,
    note_id: NoteId,
) -> Result<Option<Note>, StorageError> {
    let id_bytes = note_id.to_bytes();
    let row = client
        .query_opt(
            "SELECT n.id, n.value, n.created_at, u.user_id, u.email, u.account_note_id \
             FROM notes n JOIN users u ON u.user_id = n.author_id \
             WHERE n.id = $1",
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
) -> Result<Vec<Note>, StorageError> {
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
        "SELECT n.id, n.value, n.created_at, u.user_id, u.email, u.account_note_id \
         FROM notes n JOIN users u ON u.user_id = n.author_id {} \
         ORDER BY n.created_at DESC",
        where_clause
    );
    let rows = client.query(&query, &params).await?;
    Ok(rows.iter().map(map_note).collect())
}
pub async fn list_random_notes(
    client: &Client,
    limit: i64,
) -> Result<Vec<Note>, StorageError> {
    let rows = client
        .query(
            "SELECT n.id, n.value, n.created_at, u.user_id, u.email, u.account_note_id \
             FROM notes n JOIN users u ON u.user_id = n.author_id \
             WHERE NOT EXISTS (SELECT 1 FROM associations a WHERE (a.kind = 'next' AND a.to_id = n.id) OR (a.kind = 'prev' AND a.from_id = n.id)) \
             AND NOT EXISTS (SELECT 1 FROM associations a WHERE a.kind = 'version' AND a.from_id = n.id) \
             ORDER BY RANDOM() \
             LIMIT $1",
            &[&limit],
        )
        .await?;
    Ok(rows.iter().map(map_note).collect())
}
pub async fn find_notes_by_ids(
    client: &Client,
    note_ids: &[NoteId],
) -> Result<Vec<Note>, StorageError> {
    if note_ids.is_empty() {
        return Ok(Vec::new());
    }
    let ids: Vec<Vec<u8>> = note_ids
        .iter()
        .map(|note_id| note_id.to_bytes().to_vec())
        .collect();
    let rows = client
        .query(
            "SELECT n.id, n.value, n.created_at, u.user_id, u.email, u.account_note_id \
             FROM notes n \
             JOIN users u ON u.user_id = n.author_id \
             WHERE n.id = ANY($1)",
            &[&ids],
        )
        .await?;
    Ok(rows.iter().map(map_note).collect())
}
pub async fn list_feed_notes(
    client: &Client,
    user_id: Uuid,
    from: Option<time::OffsetDateTime>,
    to: Option<time::OffsetDateTime>,
    limit: i64,
) -> Result<Vec<Note>, StorageError> {
    let mut clauses = Vec::new();
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
    params.push(&user_id);
    clauses.push(
        "(n.author_id = $1 OR EXISTS (SELECT 1 FROM follows f WHERE f.follower_id = $1 AND f.followee_id = n.author_id))"
            .to_string(),
    );
    clauses.push(
        "NOT EXISTS (SELECT 1 FROM associations a WHERE (a.kind = 'next' AND a.to_id = n.id) OR (a.kind = 'prev' AND a.from_id = n.id))"
            .to_string(),
    );
    clauses.push(
        "NOT EXISTS (SELECT 1 FROM associations a WHERE a.kind = 'version' AND a.from_id = n.id)"
            .to_string(),
    );
    if let Some(from_ts) = from.as_ref() {
        clauses.push(format!("n.created_at >= ${}", params.len() + 1));
        params.push(from_ts);
    }
    if let Some(to_ts) = to.as_ref() {
        clauses.push(format!("n.created_at <= ${}", params.len() + 1));
        params.push(to_ts);
    }
    let limit_idx = params.len() + 1;
    params.push(&limit);
    let query = format!(
        "SELECT n.id, n.value, n.created_at, u.user_id, u.email, u.account_note_id \
         FROM notes n \
         JOIN users u ON u.user_id = n.author_id \
         WHERE {} \
         ORDER BY n.created_at DESC \
         LIMIT ${}",
        clauses.join(" AND "),
        limit_idx
    );
    let rows = client.query(&query, &params).await?;
    Ok(rows.iter().map(map_note).collect())
}
pub(crate) fn map_note(row: &tokio_postgres::Row) -> Note {
    let id_bytes: Vec<u8> = row.get(0);
    let value_bytes: Vec<u8> = row.get(1);
    let created_at: time::OffsetDateTime = row.get(2);
    let author_id: Uuid = row.get(3);
    let email: String = row.get(4);
    let account_note_id = map_account_note_id(row.get(5));
    let mut id = [0u8; 32];
    id.copy_from_slice(&id_bytes[..32]);
    Note {
        id: encode_id(id),
        value: String::from_utf8_lossy(&value_bytes).to_string(),
        created_at: format_timestamp(created_at),
        author: UserProfile {
            user_id: author_id,
            email,
            account_note_id,
        },
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
