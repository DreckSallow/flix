use std::path::PathBuf;

use anyhow::Result;
use flix_data::{models::note_model::NoteModel, note::Note};

use super::check_workspace_path;

fn get_notes_path_db(workspace_name: &str) -> Result<PathBuf> {
    let mut workspaces_path = check_workspace_path(workspace_name)?;
    workspaces_path.push("data.db");
    Ok(workspaces_path)
}

pub fn create_note(workspace_name: &str, title: &str, text: &str) -> Result<Note> {
    let notes_db_path = get_notes_path_db(workspace_name)?;
    let note = NoteModel::open_connection(notes_db_path)?.create(title, text)?;
    Ok(note)
}

pub fn delete_one(workspace_name: &str, id: u32) -> Result<Note> {
    let notes_db_path = get_notes_path_db(workspace_name)?;
    let note = NoteModel::open_connection(notes_db_path)?.delete_one(id)?;
    Ok(note)
}

pub fn find_by_id(workspace_name: &str, id: u32) -> Result<Note> {
    let notes_db_path = get_notes_path_db(workspace_name)?;
    let note = NoteModel::open_connection(notes_db_path)?.find_by_id(id)?;
    Ok(note)
}

pub fn update_one(
    workspace_name: &str,
    id: u32,
    text: Option<&str>,
    title: Option<&str>,
) -> Result<Note> {
    let notes_db_path = get_notes_path_db(workspace_name)?;
    let note = NoteModel::open_connection(notes_db_path)?.update_one(id, text, title)?;
    Ok(note)
}
