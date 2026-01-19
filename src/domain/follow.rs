use serde::{Deserialize, Serialize};

use crate::domain::user::UserProfile;

#[derive(Serialize, Deserialize, Clone)]
pub struct Follow {
    pub follower: UserProfile,
    pub followee: UserProfile,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FollowEdge {
    pub user: UserProfile,
    pub created_at: String,
}
