use std::{fs, rc::Rc};

use anyhow::{anyhow, Result};
use flix_data::{
    db::deck_connect, deck::Deck, local_data::get_folder_path, models::card_model::CardModel,
};

pub mod flix_adapters;

pub fn get_workspaces() -> Result<Vec<String>> {
    let workspaces_path =
        get_folder_path("workspaces").ok_or_else(|| anyhow!("Folder workspace not found"))?;

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

pub fn get_workspace_data(workspace_name: &str) -> Result<Vec<Deck>> {
    let mut workspaces_path =
        get_folder_path("workspaces").ok_or_else(|| anyhow!("Folder workspace not found"))?;

    workspaces_path.push(workspace_name);

    if !workspaces_path.exists() {
        return Err(anyhow!("Workspace not exist"));
    }

    let mut decks_paths = vec![];

    fs::read_dir(workspaces_path)?.for_each(|entry| {
        if let Ok(f) = entry {
            decks_paths.push(f.path())
        }
    });

    let mut decks = vec![];

    for deck_path in decks_paths {
        let conn = Rc::new(deck_connect(deck_path.join("cards.db"))?);
        let model = CardModel::new(Rc::clone(&conn));

        if let Ok(cards) = model.find_all() {
            let deck = Deck::new(deck_path).with_cards(cards);
            decks.push(deck)
        }
    }

    Ok(decks)
}

#[cfg(test)]
mod test_flix_utils {
    use crate::{get_workspace_data, get_workspaces};

    #[test]
    fn test_workspaces() {
        let res = get_workspaces();
        println!("res: {:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn test_workspace_data() {
        let res = get_workspace_data("japanese");
        assert!(false);
        assert!(res.is_ok());
    }
}
