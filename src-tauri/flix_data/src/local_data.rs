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
