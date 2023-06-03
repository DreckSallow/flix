use std::{
    collections::HashMap,
    fs,
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use rusqlite::Connection;
use zip::{result::ZipError, ZipArchive};

#[derive(Debug)]
pub struct CardInfo {
    text: String,
    audio_path: Option<PathBuf>,
    image_path: Option<PathBuf>,
}

impl Default for CardInfo {
    fn default() -> Self {
        CardInfo {
            text: String::new(),
            audio_path: None,
            image_path: None,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AnkiAdapterError {
    #[error("File error IO {0}")]
    IoError(#[from] io::Error),
    #[error("Zip error {0}")]
    Zip(#[from] ZipError),
    #[error("Db error {0}")]
    Db(#[from] rusqlite::Error),
}

type AnkiResult<T> = Result<T, AnkiAdapterError>;

/// Read the anki file [file-name].apkg
pub fn read_anki_files<P: AsRef<Path>>(
    path: P,
    directory: P,
) -> AnkiResult<(PathBuf, Vec<CardInfo>)> {
    let deck_name = path.as_ref().file_stem().unwrap();
    let mut output_dir = directory.as_ref().to_path_buf();
    output_dir.push(deck_name);

    let media_dir = output_dir.join("media");

    {
        let apkg_file = fs::File::open(path).or_else(|e| Err(AnkiAdapterError::IoError(e)))?;

        let mut archive = ZipArchive::new(BufReader::new(apkg_file))
            .or_else(|e| Err(AnkiAdapterError::Zip(e)))?;

        // println!("file anki path: {:?}", deck_name.clone());
        archive
            .extract(&media_dir)
            .or_else(|e| Err(AnkiAdapterError::Zip(e)))?;
    }

    let conn = Connection::open(media_dir.join("collection.anki2"))
        .or_else(|e| Err(AnkiAdapterError::Db(e)))?;

    let notes_ids = get_note_ids(&conn)?;

    let media_data = media_files_store(&media_dir.join("media"))
        .or_else(|e| Err(AnkiAdapterError::IoError(e)))?;
    let mut cards_info = vec![];

    for note_id in notes_ids {
        let res_prepare = conn.prepare("SELECT flds FROM notes WHERE id = ?");
        match res_prepare {
            Ok(mut stmt) => {
                let query_res = stmt.query_row([note_id], |r| {
                    let desc: String = r.get(0)?;
                    Ok(desc)
                });

                if let Ok(desc) = query_res {
                    cards_info.push(desc);
                }
            }
            Err(e) => {
                println!("ERROR ANKI NOTE_INFO: {}", e);
            }
        }
    }

    println!("cards info: {:?}", cards_info);
    let mut cards_data = vec![];

    for data in cards_info {
        let split_info = data.split("\u{001F}").collect::<Vec<&str>>();

        let mut card_info = CardInfo::default();

        for s in split_info {
            if s.contains("[sound") {
                // So then is a audio
                let audio_str = s.trim_end_matches("]").split(":").last().unwrap();
                card_info.audio_path = media_data
                    .get(audio_str)
                    .and_then(|file_name| Some(media_dir.join(file_name.as_str())));
            } else if s.contains("<img") {
                // So then is a image
                let image_path = s.split("\"").nth(1).unwrap();
                card_info.image_path = media_data
                    .get(image_path)
                    .and_then(|file_name| Some(media_dir.join(file_name.as_str())));
            } else {
                // So is text
                card_info.text.push_str(&format!("{} ", &s));
            }
        }

        cards_data.push(card_info);
    }

    let _ = conn.close();
    let _ = fs::remove_file(media_dir.join("collection.anki2"));
    let _ = fs::remove_file(media_dir.join("media"));

    // TODO: create the database

    Ok((output_dir, cards_data))
}

fn get_note_ids(conn: &Connection) -> AnkiResult<Vec<u64>> {
    let mut smt = conn
        .prepare("SELECT nid FROM cards")
        .or_else(|e| Err(AnkiAdapterError::Db(e)))?;

    let collect = smt
        .query_map([], |r| {
            let t2: u64 = r.get(0)?;
            Ok(t2)
        })
        .or_else(|e| Err(AnkiAdapterError::Db(e)))?
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .collect();
    Ok(collect)
}

fn media_files_store<P: AsRef<Path>>(path: P) -> io::Result<HashMap<String, String>> {
    println!("path to read data: {:?}", path.as_ref());
    let file_content = fs::read_to_string(path)?;
    println!("file content: {}", file_content);

    let data: HashMap<String, String> = serde_json::from_str(&file_content).unwrap();

    let mut swap_entry = HashMap::new();

    for (file_number, file_name) in data.into_iter() {
        swap_entry.insert(file_name, file_number);
    }

    Ok(swap_entry)
}

#[cfg(test)]
mod test_anki_adapter {

    use flix_data::local_data;
    use std::fs;

    use super::read_anki_files;

    #[test]
    fn print_files() {
        let deck_folder = local_data::get_folder_path("decks").unwrap();
        if deck_folder.exists() {
            fs::remove_dir_all(&deck_folder.join("japanese")).unwrap();
        }
        let path = "../.hidden/japanese.apkg";
        let res = read_anki_files(path, deck_folder.to_str().unwrap());
        assert!(res.is_ok());
        let (path, _) = res.unwrap();
        assert_eq!(path.file_name().unwrap(), "japanese");
    }
}
