use std::path::PathBuf;

use flix_data::deck::Deck;
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

///TODO: return only the deck name
#[tauri::command]
pub async fn import_deck_handler(workspace_name: &str, file_path: &str) -> Result<Deck, String> {
    //FIXME: change the static import type
    handlers::deck::import_deck(ImportType::Anki, PathBuf::from(file_path), workspace_name)
        .or_else(|e| Err(e.to_string()))
}

// #[tauri::command]
// pub fn create_card(deck_path: &str) -> Result<Deck, String> {
//     create_deck(workspace_name, deck_name).or_else(|e| Err(e.to_string()))
// }
