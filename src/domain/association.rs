use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Association {
    pub kind: String,
    pub from_id: String,
    pub to_id: String,
    pub created_at: String,
}
