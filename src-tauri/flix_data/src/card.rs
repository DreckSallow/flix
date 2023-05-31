use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Card<'a> {
    front: &'a str,
    back: &'a str,
    audio: Option<PathBuf>,
    image: Option<PathBuf>,
}

impl<'a> Card<'a> {
    pub fn new(
        front: &'a str,
        back: &'a str,
        audio: Option<PathBuf>,
        image: Option<PathBuf>,
    ) -> Self {
        Self {
            front,
            back,
            audio,
            image,
        }
    }
}
