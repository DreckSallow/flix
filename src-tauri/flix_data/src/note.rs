use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Note {
    id: u32,
    title: String,
    text: String,
}

impl Note {
    pub fn new(id: u32, title: String, text: String) -> Self {
        Self { id, title, text }
    }
}
