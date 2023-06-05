use std::{path::Path, rc::Rc};

use rusqlite::{params, Connection, Result};

pub struct CardModel {
    conn: Rc<Connection>,
}

impl CardModel {
    pub fn new(conn: Rc<Connection>) -> Self {
        Self {
            conn: Rc::clone(&conn),
        }
    }

    pub fn create<P1: AsRef<Path>, P2: AsRef<Path>>(
        &self,
        front: &str,
        back: &str,
        audio: Option<P1>,
        image: Option<P2>,
    ) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO cards (front_text, back_text, audio, image) VALUES (?1, ?2, ?3, ?4)",
            params![
                front,
                back,
                audio.map(|p| p.as_ref().to_string_lossy().to_string()),
                image.map(|p| p.as_ref().to_string_lossy().to_string()),
            ],
        )
    }
}
