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

#[derive(Clone)]
pub struct NoteChain {
    pub center: Note,
    pub prev: Vec<Note>,
    pub next: Vec<Note>,
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

pub async fn fetch_chain(
    state: &AppState,
    note_id: NoteId,
) -> Result<NoteChain, ApiError<serde_json::Value>> {
    let center = state
        .storage
        .find_note(note_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(|| ApiError::not_found("note_not_found", "Note not found"))?;

    let prev_ids = walk_chain_ids(state, &center.id, Direction::Prev).await?;
    let next_ids = walk_chain_ids(state, &center.id, Direction::Next).await?;

    let mut lookup_ids = Vec::new();
    let mut unique = HashSet::new();
    for id in prev_ids.iter().chain(next_ids.iter()) {
        if unique.insert(id.clone()) {
            if let Some(bytes) = decode_id(id) {
                lookup_ids.push(NoteId::from_bytes(bytes));
            }
        }
    }

    let notes = if lookup_ids.is_empty() {
        Vec::new()
    } else {
        state
            .storage
            .find_notes_by_ids(&lookup_ids)
            .await
            .map_err(|_| ApiError::internal())?
    };
    let note_map: HashMap<String, Note> =
        notes.into_iter().map(|note| (note.id.clone(), note)).collect();

    let prev = prev_ids
        .iter()
        .rev()
        .filter_map(|id| note_map.get(id).cloned())
        .collect();
    let next = next_ids
        .iter()
        .filter_map(|id| note_map.get(id).cloned())
        .collect();

    Ok(NoteChain { center, prev, next })
}

#[derive(Clone, Copy)]
enum Direction {
    Prev,
    Next,
}

async fn walk_chain_ids(
    state: &AppState,
    start_id: &str,
    direction: Direction,
) -> Result<Vec<String>, ApiError<serde_json::Value>> {
    let mut chain = Vec::new();
    let mut seen = HashSet::new();
    let mut current_id = start_id.to_string();
    seen.insert(current_id.clone());

    loop {
        let Some(bytes) = decode_id(&current_id) else { break };
        let associations = state
            .storage
            .list_associations(NoteId::from_bytes(bytes))
            .await
            .map_err(|_| ApiError::internal())?;
        let next_id = associations
            .iter()
            .find_map(|assoc| match direction {
                Direction::Prev => resolve_prev_id(assoc, &current_id),
                Direction::Next => resolve_next_id(assoc, &current_id),
            });
        let Some(next_id) = next_id else { break };
        if !seen.insert(next_id.to_string()) {
            break;
        }
        chain.push(next_id.to_string());
        current_id = next_id.to_string();
    }

    Ok(chain)
}

fn resolve_prev_id<'a>(assoc: &'a Association, current_id: &str) -> Option<&'a str> {
    match assoc.kind.as_str() {
        "prev" if assoc.from_id == current_id => Some(assoc.to_id.as_str()),
        "next" if assoc.to_id == current_id => Some(assoc.from_id.as_str()),
        _ => None,
    }
}

fn resolve_next_id<'a>(assoc: &'a Association, current_id: &str) -> Option<&'a str> {
    match assoc.kind.as_str() {
        "next" if assoc.from_id == current_id => Some(assoc.to_id.as_str()),
        "prev" if assoc.to_id == current_id => Some(assoc.from_id.as_str()),
        _ => None,
    }
}
