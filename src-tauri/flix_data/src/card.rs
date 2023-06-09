use std::collections::HashMap;

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Card {
    id: u32,
    items: HashMap<String, String>,
    front: String,
    back: String,
}

impl Card {
    pub fn new(id: u32, items: HashMap<String, String>, front: String, back: String) -> Self {
        Self {
            id,
            items,
            front,
            back,
        }
    }
}
