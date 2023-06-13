use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use flix_data::{deck::Deck, local_data::get_folder_path, models::card_model::CardModel};

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

pub fn get_deck(deck_path: PathBuf) -> Result<Deck> {
    let model = CardModel::open_connection(deck_path.join("cards.db"))?;

    let cards = model.find_all()?;

    Ok(Deck::new(deck_path).with_cards(cards))
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
        match get_deck(deck_path.clone()) {
            Ok(deck) => decks.push(deck),
            Err(e) => {
                println!("ERROR {:?}", e);
            }
        }
    }

    Ok(decks)
}

pub fn create_deck(workspace_name: &str, deck_name: &str) -> Result<Deck> {
    let mut workspaces_path =
        get_folder_path("workspaces").ok_or_else(|| anyhow!("Folder workspace not found"))?;

    workspaces_path.push(workspace_name);

    if !workspaces_path.exists() {
        return Err(anyhow!("Workspace not exist"));
    }
    let deck_path = workspaces_path.join(deck_name);
    fs::create_dir(&deck_path)?;
    //CREATE THE CARDS TABLE
    let _ = CardModel::open_connection(&deck_path.join("cards.db"));

    Ok(Deck::new(deck_path))
}

pub fn remove_workspace(workspace_name: &str) -> Result<PathBuf> {
    let mut workspaces_path =
        get_folder_path("workspaces").ok_or_else(|| anyhow!("Folder workspace not found"))?;

    workspaces_path.push(workspace_name);

    fs::remove_dir_all(&workspaces_path)?;

    Ok(workspaces_path)
}

pub fn rename_workspace(old_name: &str, new_name: &str) -> Result<String> {
    let workspaces_path =
        get_folder_path("workspaces").ok_or_else(|| anyhow!("Folder workspace not found"))?;

    fs::rename(
        workspaces_path.join(old_name),
        workspaces_path.join(new_name),
    )?;

    Ok(new_name.into())
}

// pub fn create_card(deck_path: &str, front: &str, back: &str, text_items: &str) -> Result<Card> {
//     let card_model = CardModel::open_connection(PathBuf::from(deck_path).join("cards.db"))?;
//     // texto
//     // audio
//     // imagen
//     //texto
//     // audio
//     // imagen

//     card_model.create(front, back, text_items, "")?;
//     //FIXME: the card id
//     // Ok(Card::new(0, , front.to_string(), back.to_string()))
// }

pub fn create_workspace(workspace_name: &str) -> Result<()> {
    let mut workspaces_path =
        get_folder_path("workspaces").ok_or_else(|| anyhow!("Folder workspace not found"))?;

    workspaces_path.push(workspace_name);
    fs::create_dir(workspaces_path)?;

    Ok(())
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
