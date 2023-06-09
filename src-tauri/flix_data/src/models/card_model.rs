use std::{collections::HashMap, path::Path, rc::Rc};

use rusqlite::{params, Connection, Result};

use crate::card::Card;

pub struct CardModel {
    conn: Rc<Connection>,
}

impl CardModel {
    pub fn open_connection<P: AsRef<Path>>(p: P) -> Result<Self> {
        let conn = Connection::open(p)?;

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS cards (
                id INTEGER PRIMARY KEY,
                text_items TEXT NOT NULL,
                text_format TEXT NOT NULL,
                front TEXT NOT NULL,
                back TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            conn: Rc::new(conn),
        })
    }
    pub fn new(conn: Rc<Connection>) -> Self {
        Self {
            conn: Rc::clone(&conn),
        }
    }

    pub fn create(
        &self,
        front: &str,
        back: &str,
        text_items: &str,
        text_format: &str,
    ) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO cards (front, back, text_items, text_format) VALUES (?1, ?2, ?3,?4)",
            params![front, back, text_items, text_format],
        )
    }
    pub fn find_all(&self) -> Result<Vec<Card>> {
        let mut smt = self.conn.prepare("SELECT * from cards")?;

        let cards: Vec<Card> = smt
            .query_map((), |r| {
                let id: u32 = r.get(0)?;
                let (text_items, text_format): (String, String) = (r.get(1)?, r.get(2)?);
                let (front, back): (String, String) = (r.get(3)?, r.get(4)?);

                let mut card_items = HashMap::new();

                let items_card: Vec<&str> = text_items.split(0x1f as char).collect();

                for (i, tfmt) in text_format.split(0x1f as char).enumerate() {
                    card_items.insert(
                        tfmt.to_owned(),
                        items_card.get(i).unwrap_or(&"").to_string(),
                    );
                }

                Ok(Card::new(id, card_items, front, back))
            })?
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        // println!("cards: {:?}", &cards[0..5]);

        Ok(cards)
    }
}
