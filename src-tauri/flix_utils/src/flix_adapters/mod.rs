use std::path::Path;

use flix_data::{local_data, models::card_model};

mod anki;
mod card_format;

#[derive(Debug)]
pub enum ImportType {
    Anki,
}

pub fn use_import_decks<P: AsRef<Path>>(import_type: ImportType, path: P, workspace: &str) {
    let mut directory = local_data::get_folder_path("workspaces").unwrap();

    directory.push(workspace);

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
            let model = card_model::CardModel::open_connection(directory.join("cards.db")).unwrap();

            for card in cards {
                model
                    .create(
                        &card.front_format,
                        &card.back_format,
                        &card.text_items,
                        &card.items_format,
                    )
                    .unwrap();
            }
        }
    }
}

#[cfg(test)]
mod import_test {

    use super::{use_import_decks, ImportType};

    #[test]
    fn read_anki_files() {
        let path = "../.hidden/japanese.apkg";
        use_import_decks(ImportType::Anki, path, "japanese");
        assert!(false); //FIXME: change this;
                        // assert!(true);
    }
}
