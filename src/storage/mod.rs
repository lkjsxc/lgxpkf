mod associations;
mod follows;
mod migrations;
mod notes;
mod sessions;
mod users;

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

use crate::config::Config;
use crate::domain::{Association, FollowEdge, Note, NoteId, User};
use crate::storage::associations::{create_association, list_associations};
use crate::storage::follows::{create_follow, delete_follow, list_followers, list_following};
use crate::storage::notes::{create_note, find_note, list_feed_notes, list_notes};
use crate::storage::sessions::{create_session, get_session_user};
use crate::storage::users::{find_or_create_user, find_user_by_id};

#[derive(Clone)]
pub struct Storage {
    pool: Pool,
}

impl Storage {
    pub async fn connect(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = Manager::from_config(
            config.database_url.parse()?,
            NoTls,
            ManagerConfig {
                recycling_method: RecyclingMethod::Fast,
            },
        );
        let pool = Pool::builder(manager).max_size(16).build().unwrap();
        Ok(Self { pool })
    }

    pub async fn run_migrations(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        migrations::run(&self.pool, path).await
    }

    pub async fn find_or_create_user(
        &self,
        google_sub: &str,
        email: &str,
    ) -> Result<User, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        find_or_create_user(&client, google_sub, email).await
    }

    pub async fn create_session(
        &self,
        user_id: uuid::Uuid,
        token: &str,
        expires_at: time::OffsetDateTime,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        create_session(&client, user_id, token, expires_at).await
    }

    pub async fn get_session_user(
        &self,
        token: &str,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        get_session_user(&client, token).await
    }

    pub async fn create_note(
        &self,
        note_id: NoteId,
        value: &[u8],
        author_id: uuid::Uuid,
    ) -> Result<Note, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        create_note(&client, note_id, value, author_id).await
    }

    pub async fn find_note(
        &self,
        note_id: NoteId,
    ) -> Result<Option<Note>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        find_note(&client, note_id).await
    }

    pub async fn list_notes(
        &self,
        author: Option<uuid::Uuid>,
        from: Option<time::OffsetDateTime>,
        to: Option<time::OffsetDateTime>,
    ) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        list_notes(&client, author, from, to).await
    }

    pub async fn list_feed_notes(
        &self,
        user_id: uuid::Uuid,
        from: Option<time::OffsetDateTime>,
        to: Option<time::OffsetDateTime>,
        limit: i64,
    ) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        list_feed_notes(&client, user_id, from, to, limit).await
    }

    pub async fn create_association(
        &self,
        kind: &str,
        from_id: NoteId,
        to_id: NoteId,
    ) -> Result<Association, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        create_association(&client, kind, from_id, to_id).await
    }

    pub async fn list_associations(
        &self,
        note_id: NoteId,
    ) -> Result<Vec<Association>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        list_associations(&client, note_id).await
    }

    pub async fn create_follow(
        &self,
        follower_id: uuid::Uuid,
        followee_id: uuid::Uuid,
    ) -> Result<Option<time::OffsetDateTime>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        create_follow(&client, follower_id, followee_id).await
    }

    pub async fn delete_follow(
        &self,
        follower_id: uuid::Uuid,
        followee_id: uuid::Uuid,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        delete_follow(&client, follower_id, followee_id).await
    }

    pub async fn list_followers(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Vec<FollowEdge>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        list_followers(&client, user_id).await
    }

    pub async fn list_following(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Vec<FollowEdge>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        list_following(&client, user_id).await
    }

    pub async fn find_user_by_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        find_user_by_id(&client, user_id).await
    }
}
