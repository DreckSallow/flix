use std::{fs, path::Path};

use anyhow::{anyhow, Result};
use flix_data::{deck::Deck, models::card_model::CardModel};

use crate::flix_adapters::{use_import_deck, ImportType};

use super::check_workspace_path;

pub fn get_decks(workspace_name: &str) -> Result<Vec<String>> {
    let workspace_path = check_workspace_path(workspace_name)?;

    let mut decks_paths = vec![];

    fs::read_dir(workspace_path)?.for_each(|entry| {
        if let Ok(f) = entry {
            if let Ok(file_type) = f.file_type() {
                if file_type.is_file() {
                    return;
                }
            }
            if let Ok(name) = f.file_name().into_string() {
                decks_paths.push(name);
            }
        }
    });

    Ok(decks_paths)
}

pub fn get_deck(workspace_name: &str, deck_name: &str) -> Result<Deck> {
    let deck_path = check_workspace_path(workspace_name)?.join(deck_name);
    let cards = CardModel::open_connection(deck_path.join("cards.db"))?.find_all()?;

    Ok(Deck::new(deck_path).with_cards(cards))
}

pub fn import_deck<P: AsRef<Path>>(
    type_file: ImportType,
    file_path: P,
    workspace_name: &str,
) -> Result<Deck> {
    let deck_path = use_import_deck(type_file, file_path, workspace_name)?;

    let deck_name = if let Some(p) = deck_path.file_name() {
        p.to_str().unwrap()
    } else {
        return Err(anyhow!("The deck path not exist"));
    };
    let deck = get_deck(workspace_name, deck_name)?;
    Ok(deck)
}

pub fn create_deck(workspace_name: &str, deck_name: &str) -> Result<Deck> {
    let deck_path = check_workspace_path(workspace_name)?.join(deck_name);
    fs::create_dir(&deck_path)?;
    //CREATE THE CARDS TABLE
    let _ = CardModel::open_connection(&deck_path.join("cards.db"));

    Ok(Deck::new(deck_path))
}

pub fn remove_deck(workspace_name: &str, deck_name: &str) -> Result<()> {
    let deck_path = check_workspace_path(workspace_name)?.join(deck_name);
    fs::remove_dir_all(deck_path)?;
    Ok(())
}

pub fn update_deck_title(
    workspace_name: &str,
    deck_name: &str,
    new_deck_name: &str,
) -> Result<String> {
    let workspace_path = check_workspace_path(workspace_name)?;
    fs::rename(
        workspace_path.join(deck_name),
        workspace_path.join(new_deck_name),
    )?;
    Ok(new_deck_name.into())
}
