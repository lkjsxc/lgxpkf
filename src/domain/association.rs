use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AssociationKind {
    Version,
    Reply,
    Aggregate,
    Other,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Association {
    pub kind: String,
    pub from_id: String,
    pub to_id: String,
    pub created_at: String,
}

impl AssociationKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Version => "version",
            Self::Reply => "reply",
            Self::Aggregate => "aggregate",
            Self::Other => "other",
        }
    }
}
