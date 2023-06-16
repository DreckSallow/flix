use std::{cell::RefCell, collections::HashMap, path::Path, rc::Rc};

use rusqlite::{Connection, Result};

use crate::note::Note;

pub struct NoteModel {
    conn: Rc<RefCell<Connection>>,
}

impl NoteModel {
    pub fn open_connection<P: AsRef<Path>>(p: P) -> Result<Self> {
        let conn = Connection::open(p)?;

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                text TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            conn: Rc::new(RefCell::new(conn)),
        })
    }

    pub fn create(&self, title: &str, text: &str) -> Result<Note> {
        self.conn.borrow().execute(
            "INSERT INTO notes (title, text) VALUES (?1, ?2)",
            [title, text],
        )?;
        let id = self.conn.borrow().last_insert_rowid() as u32;
        Ok(Note::new(id, title.to_string(), text.to_string()))
    }

    pub fn find_by_id(&self, id: u32) -> Result<Note> {
        self.conn
            .borrow()
            .query_row("SELECT * from notes WHERE id = ?1", [id], |r| {
                let id: u32 = r.get(0)?;
                let title: String = r.get(1)?;
                let text: String = r.get(2)?;
                Ok(Note::new(id, title, text))
            })
    }

    pub fn get_notes_info(&self) -> Result<HashMap<u32, String>> {
        let connection = self.conn.borrow();
        let mut stmt = connection.prepare("SELECT id, title FROM notes")?;

        let mut notes_info = HashMap::new();

        stmt.query_map([], |r| {
            let info: (u32, String) = (r.get(0)?, r.get(1)?);
            Ok(info)
        })?
        .for_each(|info| {
            if let Ok((id, title)) = info {
                notes_info.insert(id, title);
            }
        });

        Ok(notes_info)
    }

    pub fn delete_one(&self, id: u32) -> Result<Note> {
        let note = self.find_by_id(id)?;
        self.conn
            .borrow()
            .execute("DELETE FROM notes WHERE id = ?1", [id])?;
        Ok(note)
    }

    pub fn update_one(&self, id: u32, text: Option<&str>, title: Option<&str>) -> Result<Note> {
        let mut sql = String::from("UPDATE notes SET");
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

        if let Some(title) = &title {
            sql.push_str(" title = ?,");

            params.push(title);
        }

        if let Some(content) = &text {
            sql.push_str(" text = ?,");
            params.push(content);
        }

        //Remove the final comma
        sql.pop();

        sql.push_str(" WHERE id = ?;");
        params.push(&id);

        self.conn.borrow().execute(&sql, params.as_slice())?;

        self.find_by_id(id)
    }
}
