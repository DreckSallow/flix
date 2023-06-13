use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::card::Card;

#[derive(Clone, Debug, Serialize)]
pub struct Deck {
    path: PathBuf,
    name: String,
    cards: Vec<Card>,
}

impl Deck {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            name: path
                .as_ref()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            cards: vec![],
        }
    }
    pub fn with_cards(self, cards: Vec<Card>) -> Self {
        Self { cards, ..self }
    }
}
