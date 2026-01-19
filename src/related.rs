use serde::Serialize;
use std::collections::{HashMap, HashSet};

use crate::domain::{Association, Note, NoteId};
use crate::errors::ApiError;
use crate::state::AppState;
use crate::urls::base32::decode_id;

#[derive(Serialize, Clone)]
pub struct RelatedEntry {
    pub association: Association,
    pub note: Note,
}

#[derive(Serialize, Clone)]
pub struct RelatedResponse {
    pub center: Note,
    pub related: Vec<RelatedEntry>,
}

pub async fn fetch_related(
    state: &AppState,
    note_id: NoteId,
) -> Result<RelatedResponse, ApiError<serde_json::Value>> {
    let center = state
        .storage
        .find_note(note_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(|| ApiError::not_found("note_not_found", "Note not found"))?;

    let associations = state
        .storage
        .list_associations(note_id)
        .await
        .map_err(|_| ApiError::internal())?;

    let mut seen = HashSet::new();
    let mut related_ids = Vec::new();
    for assoc in &associations {
        let other = if assoc.from_id == center.id {
            assoc.to_id.as_str()
        } else {
            assoc.from_id.as_str()
        };
        if seen.insert(other.to_string()) {
            if let Some(bytes) = decode_id(other) {
                related_ids.push(NoteId::from_bytes(bytes));
            }
        }
    }

    let notes = if related_ids.is_empty() {
        Vec::new()
    } else {
        state
            .storage
            .find_notes_by_ids(&related_ids)
            .await
            .map_err(|_| ApiError::internal())?
    };
    let note_map: HashMap<String, Note> =
        notes.into_iter().map(|note| (note.id.clone(), note)).collect();

    let related = associations
        .into_iter()
        .filter_map(|association| {
            let other_id = if association.from_id == center.id {
                association.to_id.clone()
            } else {
                association.from_id.clone()
            };
            note_map
                .get(&other_id)
                .cloned()
                .map(|note| RelatedEntry { association, note })
        })
        .collect();

    Ok(RelatedResponse { center, related })
}
