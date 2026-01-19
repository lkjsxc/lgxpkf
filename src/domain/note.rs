use rand::RngCore;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::user::UserProfile;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NoteId(pub [u8; 32]);

#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: String,
    pub value: String,
    pub created_at: String,
    pub author: UserProfile,
}

impl NoteId {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn to_bytes(self) -> [u8; 32] {
        self.0
    }
}

pub fn format_timestamp(ts: OffsetDateTime) -> String {
    ts.format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}

pub fn generate_note_id() -> NoteId {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    NoteId(bytes)
}
