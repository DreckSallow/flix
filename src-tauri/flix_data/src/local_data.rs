use std::{fs, path::PathBuf};

use directories::ProjectDirs;

pub fn local_data_dir() -> Option<PathBuf> {
    let local_path = ProjectDirs::from("dev", "flix", "Flix-App")?;

    let dir = local_path.data_local_dir();
    if !dir.exists() {
        if let Err(e) = fs::create_dir_all(dir) {
            eprintln!("{}", e);
        }
    }
    Some(dir.to_path_buf())
}

pub fn get_folder_path(folder: &str) -> Option<PathBuf> {
    let mut local_folder = local_data_dir()?;

    local_folder.push(folder);

    if !local_folder.exists() {
        if let Err(e) = fs::create_dir_all(&local_folder) {
            eprintln!("{}", e);
        }
    }

    Some(local_folder)
}

#[cfg(test)]
mod dir_test {
    use crate::local_data::get_folder_path;

    use super::local_data_dir;

    #[test]
    fn test_local_folder() {
        let folder_res = local_data_dir();

        assert!(folder_res.is_some());
    }
    
    #[test]
    fn test_local_create_folder() {
        let folder_res = get_folder_path("decks");

        assert!(folder_res.is_some());
    }
}
