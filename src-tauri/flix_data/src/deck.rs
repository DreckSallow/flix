use serde::Serialize;

use crate::card::Card;

#[derive(Clone, Debug, Serialize)]
pub struct Deck {
    name: String,
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            cards: vec![],
        }
    }
    pub fn with_cards(self, cards: Vec<Card>) -> Self {
        Self { cards, ..self }
    }
}
