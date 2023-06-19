// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod handlers;

use handlers::{
    create_deck_handler, create_note, create_workspace_handler, delete_one_note, get_deck_handler,
    get_decks_handler, get_notes_info, get_one_note, get_workspaces_handler, import_deck_handler,
    remove_deck_handler, remove_workspace_handler, rename_workspace_handler, update_deck_handler,
    update_note,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_workspaces_handler,
            get_decks_handler,
            create_deck_handler,
            get_deck_handler,
            create_workspace_handler,
            import_deck_handler,
            remove_deck_handler,
            update_deck_handler,
            remove_workspace_handler,
            rename_workspace_handler,
            create_note,
            update_note,
            get_notes_info,
            delete_one_note,
            get_one_note,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
