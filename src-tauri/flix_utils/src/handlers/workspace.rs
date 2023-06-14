use std::{fs, path::PathBuf};

use anyhow::Result;

use super::get_workspaces_path;

pub fn create_workspace(workspace_name: &str) -> Result<PathBuf> {
    let mut workspaces_path = get_workspaces_path()?;
    workspaces_path.push(workspace_name);
    fs::create_dir(&workspaces_path)?;

    Ok(workspaces_path)
}

pub fn rename_workspace(old_name: &str, new_name: &str) -> Result<PathBuf> {
    let workspaces_path = get_workspaces_path()?;

    fs::rename(
        workspaces_path.join(old_name),
        workspaces_path.join(new_name),
    )?;

    Ok(workspaces_path.join(new_name))
}

pub fn remove_workspace(workspace_name: &str) -> Result<PathBuf> {
    let mut workspaces_path = get_workspaces_path()?;

    workspaces_path.push(workspace_name);

    fs::remove_dir_all(&workspaces_path)?;

    Ok(workspaces_path)
}

pub fn get_workspaces() -> Result<Vec<String>> {
    let workspaces_path = get_workspaces_path()?;

    let mut workspaces = vec![];

    fs::read_dir(workspaces_path)?.for_each(|entry| {
        if let Ok(f) = entry {
            if f.path().is_dir() {
                workspaces.push(f.file_name().to_string_lossy().to_string());
            }
        }
    });

    Ok(workspaces)
}
