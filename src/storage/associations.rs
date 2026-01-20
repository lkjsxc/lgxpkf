use std::fmt;
use tokio_postgres::{Client, GenericClient};
use uuid::Uuid;

use crate::domain::{Association, NoteId};
use crate::domain::note::format_timestamp;
use crate::storage::StorageError;
use crate::urls::base32::encode_id;

#[derive(Debug)]
pub enum AssociationInsertError {
    VersionExists,
}

impl fmt::Display for AssociationInsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssociationInsertError::VersionExists => write!(f, "version association already exists"),
        }
    }
}

impl std::error::Error for AssociationInsertError {}

pub async fn create_association<C>(
    client: &C,
    kind: &str,
    from_id: NoteId,
    to_id: NoteId,
) -> Result<Association, StorageError>
where
    C: GenericClient + Sync,
{
    let from_bytes = from_id.to_bytes();
    let to_bytes = to_id.to_bytes();
    let from_vec = from_bytes.to_vec();
    let to_vec = to_bytes.to_vec();
    if kind == "version" {
        let row = client
            .query_opt(
                "INSERT INTO associations (id, kind, from_id, to_id, created_at) SELECT $1, $2, $3, $4, NOW() WHERE NOT EXISTS (SELECT 1 FROM associations WHERE kind = 'version' AND from_id = $3) RETURNING kind, from_id, to_id, created_at",
                &[&Uuid::new_v4(), &kind, &from_vec, &to_vec],
            )
            .await?;
        if let Some(row) = row {
            return Ok(map_association(&row));
        }
        return Err(Box::new(AssociationInsertError::VersionExists));
    }
    client
        .execute(
            "INSERT INTO associations (id, kind, from_id, to_id, created_at) VALUES ($1, $2, $3, $4, NOW())",
            &[&Uuid::new_v4(), &kind, &from_vec, &to_vec],
        )
        .await?;

    let row = client
        .query_one(
            "SELECT kind, from_id, to_id, created_at FROM associations WHERE from_id = $1 AND to_id = $2 ORDER BY created_at DESC LIMIT 1",
            &[&from_vec, &to_vec],
        )
        .await?;
    Ok(map_association(&row))
}

pub async fn list_associations(
    client: &Client,
    note_id: NoteId,
) -> Result<Vec<Association>, StorageError> {
    let id_bytes = note_id.to_bytes();
    let rows = client
        .query(
            "SELECT kind, from_id, to_id, created_at FROM associations WHERE from_id = $1 OR to_id = $1 ORDER BY created_at DESC",
            &[&id_bytes.to_vec()],
        )
        .await?;
    Ok(rows.iter().map(map_association).collect())
}

fn map_association(row: &tokio_postgres::Row) -> Association {
    let kind: String = row.get(0);
    let from_bytes: Vec<u8> = row.get(1);
    let to_bytes: Vec<u8> = row.get(2);
    let created_at: time::OffsetDateTime = row.get(3);

    let mut from_id = [0u8; 32];
    let mut to_id = [0u8; 32];
    from_id.copy_from_slice(&from_bytes[..32]);
    to_id.copy_from_slice(&to_bytes[..32]);

    Association {
        kind,
        from_id: encode_id(from_id),
        to_id: encode_id(to_id),
        created_at: format_timestamp(created_at),
    }
}
