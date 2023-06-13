use std::{collections::HashMap, path::Path};

use anyhow::Result as AnyResult;
use rusqlite::{Connection, Error, Result};

#[derive(Debug)]
pub struct CardRaw {
    pub text: String,
    pub template_id: u8,
    pub model_id: u64,
}

impl CardRaw {
    pub fn new(text: String, temp_id: u8, model_id: u64) -> Self {
        Self {
            text,
            template_id: temp_id,
            model_id,
        }
    }
}

pub struct AnkiDbExtractor {
    conn: Connection,
}

impl AnkiDbExtractor {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            conn: Connection::open(path)?,
        })
    }

    pub fn get_templates_json(&self) -> Result<String> {
        let model_str = self.conn.query_row("SELECT models from col", (), |row| {
            let json_model: String = row.get(0)?;
            Ok(json_model)
        })?;
        Ok(model_str)
    }

    pub fn get_template_parsed(&self) -> AnyResult<HashMap<u64, ModelTemplateCard>> {
        let model_str = self.get_templates_json()?;
        Ok(serde_json::from_str(&model_str)?)
    }

    pub fn get_cards(&self) -> Result<Vec<CardRaw>> {
        let mut smt_card_query = self.conn.prepare("SELECT nid, ord FROM cards")?;

        let card_data: Vec<(u64, u8)> = smt_card_query
            .query_map([], |r| {
                let nid: u64 = r.get(0)?;
                let ord: u8 = r.get(1)?;
                Ok((nid, ord))
            })?
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .collect();

        let mut cards = vec![];

        for (note_id, n_template) in card_data {
            let mut smt_note_query = self
                .conn
                .prepare("SELECT flds, mid FROM notes WHERE id = ?")?;

            let query_res = smt_note_query.query_row([note_id], |r| {
                let desc: String = r.get(0)?;
                let model_id: u64 = r.get(1)?;
                Ok((desc, model_id))
            });

            match query_res {
                Ok((desc, model_id)) => cards.push(CardRaw::new(desc, n_template, model_id)),
                Err(e) => println!("ERROR NOTE INFO ANKI: {:?}", e),
            }
        }

        Ok(cards)
    }

    pub fn close(self) -> Result<(), (Connection, Error)> {
        self.conn.close()
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldCard {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelTemplateCard {
    pub flds: Vec<FieldCard>,
    pub tmpls: Vec<CardTemplate>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CardTemplate {
    pub name: String,
    pub qfmt: String,
    pub afmt: String,
    pub ord: u8,
}
