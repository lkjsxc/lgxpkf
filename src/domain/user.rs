use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub user_id: Uuid,
    pub email: String,
}

#[derive(Clone)]
pub struct User {
    pub user_id: Uuid,
    pub google_sub: String,
    pub email: String,
}

impl User {
    pub fn profile(&self) -> UserProfile {
        UserProfile {
            user_id: self.user_id,
            email: self.email.clone(),
        }
    }
}
