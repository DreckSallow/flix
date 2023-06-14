pub mod deck;
pub mod note;
pub mod workspace;

use anyhow::{anyhow, Result};
use flix_data::local_data::get_folder_path;
use std::path::PathBuf;

fn get_workspaces_path() -> Result<PathBuf> {
    let workspaces_path =
        get_folder_path("workspaces").ok_or_else(|| anyhow!("Folder workspace not found"))?;
    Ok(workspaces_path)
}

fn check_workspace_path(workspace_name: &str) -> Result<PathBuf> {
    let mut workspaces_path = get_workspaces_path()?;

    workspaces_path.push(workspace_name);

    if !workspaces_path.exists() || !workspaces_path.is_dir() {
        return Err(anyhow!("Workspace not exist"));
    }
    Ok(workspaces_path)
}
