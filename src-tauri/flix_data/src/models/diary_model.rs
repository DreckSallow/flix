use std::{cell::RefCell, path::Path, rc::Rc};

use rusqlite::{params, Connection, Result};

use crate::diary_page::DiaryPage;

pub struct DiaryModel {
    conn: Rc<RefCell<Connection>>,
}

impl DiaryModel {
    pub fn open_connection<P: AsRef<Path>>(p: P) -> Result<Self> {
        let conn = Connection::open(p)?;

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS diary_pages (
                id INTEGER PRIMARY KEY,
                text TEXT NOT NULL,
            )",
            [],
        )?;

        Ok(Self {
            conn: Rc::new(RefCell::new(conn)),
        })
    }

    pub fn create(&self, text: &str) -> Result<DiaryPage> {
        self.conn
            .borrow()
            .execute("INSERT INTO diary_pages ( text ) VALUES (?1)", [text])?;
        let id = self.conn.borrow().last_insert_rowid() as u32;
        Ok(DiaryPage::new(id, text.to_string()))
    }

    pub fn find_by_id(&self, id: u32) -> Result<DiaryPage> {
        self.conn
            .borrow()
            .query_row("SELECT * from diary_pages WHERE id = ?1", [id], |r| {
                let id: u32 = r.get(0)?;
                let text: String = r.get(1)?;
                Ok(DiaryPage::new(id, text))
            })
    }

    pub fn delete_one(&self, id: u32) -> Result<DiaryPage> {
        let diary_page = self.find_by_id(id)?;
        self.conn
            .borrow()
            .execute("DELETE FROM diary WHERE id = ?1", [id])?;
        Ok(diary_page)
    }

    pub fn update_one(&self, id: u32, text: &str) -> Result<DiaryPage> {
        self.conn.borrow().execute(
            "UPDATE diary_pages SET text = ?1 WHERE id = ?2",
            params![text, id],
        )?;

        self.find_by_id(id)
    }
}
