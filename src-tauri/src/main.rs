// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod handlers;

use handlers::{get_decks_handler, workspaces_handler};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            workspaces_handler,
            get_decks_handler
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
