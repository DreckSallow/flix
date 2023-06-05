use rusqlite::{Connection, Result};
use std::path::Path;

pub mod card_model;

pub fn open_db<P: AsRef<Path>>(p: P) -> Result<Connection> {
    let conn = Connection::open(p)?;

    conn.execute(
        "
            CREATE TABLE IF NOT EXISTS cards (
                id INTEGER PRIMARY KEY,
                front_text TEXT NOT NULL,
                back_text TEXT NOT NULL,
                audio TEXT,
                image TEXT
        )
        ",
        [],
    )?;
    Ok(conn)
}
