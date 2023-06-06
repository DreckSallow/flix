use std::{fs, path::Path, rc::Rc};

use flix_data::{local_data, models::{open_db, card_model}};
use rusqlite::Connection;

mod anki;

#[derive(Debug)]
pub enum ImportType {
    Anki,
}

pub fn use_import_decks<P: AsRef<Path>>(import_type: ImportType, path: P) {
    let mut directory = local_data::get_folder_path("decks").unwrap();

    let adapter_import_res = match import_type {
        ImportType::Anki => anki::read_anki_files(path, &directory),
    };

    match adapter_import_res {
        Err(e) => {
            eprintln!("Error adapter: {:?}", e);
            return;
        }
        Ok((path, cards)) => {
            directory.push(path);
            let conn = open_db(directory.join("cards.db")).unwrap();

            let model=card_model::CardModel::new(Rc::new(conn));

            for card in cards {
                model.create(&card.text, "", card.audio_path, card.image_path).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod import_test {

    use super::{use_import_decks, ImportType};

    #[test]
    fn print_files() {
        let path = "../.hidden/japanese.apkg";
        let _res= use_import_decks(ImportType::Anki, path);
        assert!(true); //FIXME: change this;
    }
}