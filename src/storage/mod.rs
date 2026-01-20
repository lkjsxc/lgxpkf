mod associations; mod follows; mod migrations; mod notes; mod sessions; mod users;
pub use crate::storage::associations::AssociationInsertError;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
use crate::config::Config;
use crate::domain::{generate_note_id, Association, FollowEdge, Note, NoteId, User};
use crate::storage::associations::{create_association, list_associations};
use crate::storage::follows::{create_follow, delete_follow, list_followers, list_following};
use crate::storage::notes::{create_note, find_note, find_notes_by_ids, insert_note, list_feed_notes, list_notes, list_random_notes};
use crate::storage::sessions::{create_session, get_session_user};
use crate::storage::users::{create_account_note, find_or_create_user, find_user_by_id, is_account_note_id};
use crate::urls::base32::encode_id;

pub type StorageError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone)]
pub struct Storage { pool: Pool }

impl Storage {
    pub async fn connect(config: &Config) -> Result<Self, StorageError> {
        let manager = Manager::from_config(config.database_url.parse()?, NoTls, ManagerConfig { recycling_method: RecyclingMethod::Fast });
        let pool = Pool::builder(manager).max_size(16).build().unwrap();
        Ok(Self { pool })
    }
    pub async fn run_migrations(&self, path: &str) -> Result<(), StorageError> { migrations::run(&self.pool, path).await }
    pub async fn healthcheck(&self) -> Result<(), StorageError> { let client = self.pool.get().await?; client.query_one("SELECT 1", &[]).await?; Ok(()) }
    pub async fn find_or_create_user(&self, google_sub: &str, email: &str) -> Result<User, StorageError> { let client = self.pool.get().await?; find_or_create_user(&client, google_sub, email).await }
    pub async fn create_session(&self, user_id: uuid::Uuid, token: &str, expires_at: time::OffsetDateTime) -> Result<(), StorageError> { let client = self.pool.get().await?; create_session(&client, user_id, token, expires_at).await }
    pub async fn get_session_user(&self, token: &str) -> Result<Option<User>, StorageError> { let client = self.pool.get().await?; get_session_user(&client, token).await }
    pub async fn create_note(&self, note_id: NoteId, value: &[u8], author_id: uuid::Uuid) -> Result<Note, StorageError> {
        let mut client = self.pool.get().await?; let client_ref = &mut **client; create_note(client_ref, note_id, value, author_id).await
    }
    pub async fn create_note_chain(&self, segments: &[Vec<u8>], author_id: uuid::Uuid, account_note_id: NoteId) -> Result<(Note, Vec<String>), StorageError> {
        let mut client = self.pool.get().await?; let client_ref = &mut **client; let transaction = client_ref.transaction().await?;
        let mut ids = Vec::with_capacity(segments.len()); let mut root_note = None; let mut prev_id: Option<NoteId> = None;
        for (index, segment) in segments.iter().enumerate() {
            let note_id = generate_note_id();
            if index == 0 {
                let note = create_note(&transaction, note_id, segment, author_id).await?;
                create_association(&transaction, "author", account_note_id, note_id).await?; root_note = Some(note);
            } else { insert_note(&transaction, note_id, segment, author_id).await?; }
            if let Some(prev) = prev_id { create_association(&transaction, "next", prev, note_id).await?; }
            prev_id = Some(note_id); ids.push(note_id);
        }
        transaction.commit().await?;
        let root = root_note.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "missing_root_note"))?;
        let segments = ids.iter().map(|id| encode_id(id.to_bytes())).collect();
        Ok((root, segments))
    }
    pub async fn find_note(&self, note_id: NoteId) -> Result<Option<Note>, StorageError> { let client = self.pool.get().await?; find_note(&client, note_id).await }
    pub async fn find_notes_by_ids(&self, note_ids: &[NoteId]) -> Result<Vec<Note>, StorageError> { let client = self.pool.get().await?; find_notes_by_ids(&client, note_ids).await }
    pub async fn list_notes(&self, author: Option<uuid::Uuid>, from: Option<time::OffsetDateTime>, to: Option<time::OffsetDateTime>) -> Result<Vec<Note>, StorageError> { let client = self.pool.get().await?; list_notes(&client, author, from, to).await }
    pub async fn list_feed_notes(&self, user_id: uuid::Uuid, from: Option<time::OffsetDateTime>, to: Option<time::OffsetDateTime>, limit: i64) -> Result<Vec<Note>, StorageError> { let client = self.pool.get().await?; list_feed_notes(&client, user_id, from, to, limit).await }
    pub async fn list_random_notes(&self, limit: i64) -> Result<Vec<Note>, StorageError> { let client = self.pool.get().await?; list_random_notes(&client, limit).await }
    pub async fn create_association(&self, kind: &str, from_id: NoteId, to_id: NoteId) -> Result<Association, StorageError> { let client = self.pool.get().await?; let client_ref = &**client; create_association(client_ref, kind, from_id, to_id).await }
    pub async fn list_associations(&self, note_id: NoteId) -> Result<Vec<Association>, StorageError> { let client = self.pool.get().await?; list_associations(&client, note_id).await }
    pub async fn create_follow(&self, follower_id: uuid::Uuid, followee_id: uuid::Uuid) -> Result<Option<time::OffsetDateTime>, StorageError> { let client = self.pool.get().await?; create_follow(&client, follower_id, followee_id).await }
    pub async fn delete_follow(&self, follower_id: uuid::Uuid, followee_id: uuid::Uuid) -> Result<bool, StorageError> { let client = self.pool.get().await?; delete_follow(&client, follower_id, followee_id).await }
    pub async fn list_followers(&self, user_id: uuid::Uuid) -> Result<Vec<FollowEdge>, StorageError> { let client = self.pool.get().await?; list_followers(&client, user_id).await }
    pub async fn list_following(&self, user_id: uuid::Uuid) -> Result<Vec<FollowEdge>, StorageError> { let client = self.pool.get().await?; list_following(&client, user_id).await }
    pub async fn find_user_by_id(&self, user_id: uuid::Uuid) -> Result<Option<User>, StorageError> { let client = self.pool.get().await?; find_user_by_id(&client, user_id).await }
    pub async fn create_account_note(&self, user_id: uuid::Uuid, value: &[u8]) -> Result<Note, StorageError> { let client = self.pool.get().await?; create_account_note(&client, user_id, value).await }
    pub async fn is_account_note_id(&self, note_id: NoteId) -> Result<bool, StorageError> { let client = self.pool.get().await?; is_account_note_id(&client, note_id).await }
}
