use deadpool_postgres::Pool;
use tokio::fs;
use tokio_postgres::Client;

use crate::storage::StorageError;

pub async fn run(pool: &Pool, path: &str) -> Result<(), StorageError> {
    let mut entries = fs::read_dir(path).await?;
    let mut files = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".sql") {
            files.push((name, entry.path()));
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let client = pool.get().await?;
    ensure_table(&client).await?;
    for (name, path) in files {
        let applied = is_applied(&client, &name).await?;
        if applied {
            continue;
        }
        let sql = fs::read_to_string(path).await?;
        client.batch_execute("BEGIN").await?;
        if let Err(err) = client.batch_execute(&sql).await {
            client.batch_execute("ROLLBACK").await.ok();
            return Err(Box::new(err));
        }
        mark_applied(&client, &name).await?;
        client.batch_execute("COMMIT").await?;
    }
    Ok(())
}

async fn ensure_table(client: &Client) -> Result<(), StorageError> {
    client
        .batch_execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (name TEXT PRIMARY KEY, applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW())",
        )
        .await?;
    Ok(())
}

async fn is_applied(client: &Client, name: &str) -> Result<bool, StorageError> {
    let row = client
        .query_opt("SELECT name FROM schema_migrations WHERE name = $1", &[&name])
        .await?;
    Ok(row.is_some())
}

async fn mark_applied(client: &Client, name: &str) -> Result<(), StorageError> {
    client
        .execute(
            "INSERT INTO schema_migrations (name) VALUES ($1)",
            &[&name],
        )
        .await?;
    Ok(())
}
