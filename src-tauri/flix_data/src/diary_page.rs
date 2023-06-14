use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct DiaryPage {
    id: u32,
    text: String, //TODO: add date
}

impl DiaryPage {
    pub fn new(id: u32, text: String) -> Self {
        Self { id, text }
    }
}
