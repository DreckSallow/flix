use std::path::PathBuf;

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Card {
    id: u32,
    front: String,
    back: String,
    audio: Option<PathBuf>,
    image: Option<PathBuf>,
}

impl Card {
    pub fn new(
        id: u32,
        front: &str,
        back: &str,
        audio: Option<PathBuf>,
        image: Option<PathBuf>,
    ) -> Self {
        Self {
            id,
            front: front.into(),
            back: back.into(),
            audio,
            image,
        }
    }
}
