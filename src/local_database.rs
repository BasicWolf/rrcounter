use std::error::Error;

use rusqlite::Connection;

use crate::storage::Storage;

pub struct LocalDatabase {
    connection: Connection,
}

impl LocalDatabase {
    pub fn new(db_path: &str) -> LocalDatabase {
        let connection = Connection::open(&db_path).expect("Cannot open database file");

        let db = LocalDatabase {
            connection: connection,
        };
        db.init();
        db
    }

    pub fn init(&self) {
        self.connection
            .execute(
                "CREATE TABLE IF NOT EXISTS counter (
                 id INTEGER PRIMARY KEY CHECK (id = 1),
                 value INTEGER NOT NULL
                )",
                [],
            )
            .expect("Cannot create counter table");
    }
}

impl Storage for LocalDatabase {
    fn get_initial_value(&self) -> i64 {
        self.connection
            .query_row("SELECT value FROM counter WHERE id = 1", [], |row| {
                row.get(0)
            })
            .unwrap_or(0)
    }

    fn persist_counter(&self, counter: i64) -> Result<(), Box<dyn Error>> {
        self.connection
            .execute(
                r#"UPDATE counter
                   SET value = ?1
                   WHERE id=1"#,
                (counter,),
            )
            .map(|_| ())?;
        Ok(())
    }
}
