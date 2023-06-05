use std::path::PathBuf;

use crate::card::Card;

#[derive(Clone, Debug)]
pub struct Deck<'a> {
    name: &'a str,
    cards: Vec<Card<'a>>,
    image: Option<PathBuf>,
}

impl<'a> Deck<'a> {
    pub fn new(name: &'a str, image: Option<PathBuf>) -> Self {
        Self {
            name,
            cards: vec![],
            image,
        }
    }
    pub fn with_cards(self, cards: Vec<Card<'a>>) -> Self {
        Self { cards, ..self }
    }
}
