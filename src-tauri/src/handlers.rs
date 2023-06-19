use std::{collections::HashMap, path::PathBuf};

use flix_data::{deck::Deck, note::Note};
use flix_utils::{flix_adapters::ImportType, handlers};

#[tauri::command]
pub fn get_workspaces_handler() -> Result<Vec<String>, String> {
    handlers::workspace::get_workspaces().or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub fn create_workspace_handler(workspace_name: &str) -> Result<PathBuf, String> {
    handlers::workspace::create_workspace(workspace_name).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub fn remove_workspace_handler(workspace_name: &str) -> Result<PathBuf, String> {
    handlers::workspace::remove_workspace(workspace_name).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub fn rename_workspace_handler(workspace_name: &str, new_name: &str) -> Result<PathBuf, String> {
    handlers::workspace::rename_workspace(workspace_name, new_name).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub fn get_decks_handler(workspace_name: &str) -> Result<Vec<String>, String> {
    handlers::deck::get_decks(workspace_name).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub fn create_deck_handler(workspace_name: &str, deck_name: &str) -> Result<Deck, String> {
    handlers::deck::create_deck(workspace_name, deck_name).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub fn get_deck_handler(workspace_name: &str, deck_name: &str) -> Result<Deck, String> {
    handlers::deck::get_deck(workspace_name, deck_name).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub fn remove_deck_handler(workspace_name: &str, deck_name: &str) -> Result<(), String> {
    handlers::deck::remove_deck(workspace_name, deck_name).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub fn update_deck_handler(
    workspace_name: &str,
    deck_name: &str,
    new_deck_name: &str,
) -> Result<String, String> {
    handlers::deck::update_deck_title(workspace_name, deck_name, new_deck_name)
        .or_else(|e| Err(e.to_string()))
}

///TODO: return only the deck name
#[tauri::command]
pub async fn import_deck_handler(workspace_name: &str, file_path: &str) -> Result<Deck, String> {
    //FIXME: change the static import type
    handlers::deck::import_deck(ImportType::Anki, PathBuf::from(file_path), workspace_name)
        .or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub async fn create_note(workspace_name: &str, title: &str, text: &str) -> Result<Note, String> {
    handlers::note::create_note(workspace_name, title, text).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub async fn get_notes_info(workspace_name: &str) -> Result<HashMap<u32, String>, String> {
    handlers::note::get_notes_info(workspace_name).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub async fn delete_one_note(workspace_name: &str, id: u32) -> Result<Note, String> {
    handlers::note::delete_one(workspace_name, id).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub async fn get_one_note(workspace_name: &str, id: u32) -> Result<Note, String> {
    handlers::note::find_by_id(workspace_name, id).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub async fn update_note(
    workspace_name: &str,
    id: u32,
    title: Option<&str>,
    text: Option<&str>,
) -> Result<Note, String> {
    handlers::note::update_one(workspace_name, id, text, title).or_else(|e| Err(e.to_string()))
}

// #[tauri::command]
// pub fn create_card(deck_path: &str) -> Result<Deck, String> {
//     create_deck(workspace_name, deck_name).or_else(|e| Err(e.to_string()))
// }
