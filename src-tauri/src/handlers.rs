use flix_data::deck::Deck;
use flix_utils::{create_deck, create_workspace, get_workspace_data, get_workspaces};

#[tauri::command]
pub fn workspaces_handler() -> Result<Vec<String>, String> {
    get_workspaces().or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub fn get_decks_handler(workspace_name: String) -> Result<Vec<Deck>, String> {
    get_workspace_data(&workspace_name).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub fn create_deck_handler(workspace_name: String, deck_name: String) -> Result<Deck, String> {
    create_deck(&workspace_name, &deck_name).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub fn create_workspace_handler(workspace_name: &str) -> Result<(), String> {
    create_workspace(workspace_name).or_else(|e| Err(e.to_string()))
}

// #[tauri::command]
// pub fn create_card(deck_path: &str) -> Result<Deck, String> {
//     create_deck(workspace_name, deck_name).or_else(|e| Err(e.to_string()))
// }
