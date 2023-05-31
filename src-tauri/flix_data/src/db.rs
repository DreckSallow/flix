use std::path::Path;

use rusqlite::{Connection, Result};

pub fn deck_connect<P: AsRef<Path>>(path: P) -> Result<Connection> {
    Connection::open(path)
}
