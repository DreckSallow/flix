use flix_data::deck::Deck;
use flix_utils::{get_workspace_data, get_workspaces};

#[tauri::command]
pub fn workspaces_handler() -> Result<Vec<String>, String> {
    get_workspaces().or_else(|e| Err(e.to_string()))
}

#[tauri::command]
pub fn get_decks_handler(workspace_name: String) -> Result<Vec<Deck>, String> {
    get_workspace_data(&workspace_name).or_else(|e| Err(e.to_string()))
}
