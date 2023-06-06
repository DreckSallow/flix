use std::{path::Path, rc::Rc};

use rusqlite::{params, Connection, Result};

use crate::card::Card;

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
    pub fn find_all(&self) -> Result<Vec<Card>> {
        let mut smt = self.conn.prepare("SELECT * from cards")?;

        let cards: Vec<Card> = smt
            .query_map((), |r| {
                let id: u32 = r.get(0)?;
                let (front, back): (String, String) = (r.get(1)?, r.get(2)?);
                let (audio, image): (Option<String>, Option<String>) = (r.get(3)?, r.get(4)?);

                Ok(Card::new(
                    id,
                    &front,
                    &back,
                    audio.map(|p| p.into()),
                    image.map(|p| p.into()),
                ))
            })?
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        Ok(cards)
    }
}
