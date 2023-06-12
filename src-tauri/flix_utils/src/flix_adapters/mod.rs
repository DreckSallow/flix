use std::path::{Path, PathBuf};

use anyhow::Result;
use flix_data::{local_data, models::card_model};

mod anki;
mod card_format;

#[derive(Debug)]
pub enum ImportType {
    Anki,
}

pub fn use_import_deck<P: AsRef<Path>>(
    import_type: ImportType,
    path: P,
    workspace: &str,
) -> Result<PathBuf> {
    let mut directory = local_data::get_folder_path("workspaces").unwrap();

    directory.push(workspace);

    let (path, cards) = match import_type {
        ImportType::Anki => anki::read_anki_files(path, &directory)?,
    };
    directory.push(path);

    let conn = card_model::CardModel::open_connection(directory.join("cards.db"))
        .unwrap()
        .get_conn();

    let mut connection = conn.borrow_mut();

    let transaction = connection.transaction()?;

    for card in cards {
        transaction.execute(
            "INSERT INTO cards (front, back, text_items, text_format) VALUES (?1, ?2, ?3,?4)",
            [
                card.front_format,
                card.back_format,
                card.text_items,
                card.items_format,
            ],
        )?;
    }
    transaction.commit()?;

    Ok(directory)
}

#[cfg(test)]
mod import_test {

    use super::{use_import_deck, ImportType};

    #[test]
    fn read_anki_files() {
        let path = "../.hidden/japanese.apkg";
        use_import_deck(ImportType::Anki, path, "japanese");
        assert!(false); //FIXME: change this;
                        // assert!(true);
    }
}
