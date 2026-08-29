use rusqlite::Connection;

pub struct Storage {
    connection: Connection,
}

impl Storage {
    pub fn new(db_path: &str) -> Storage {
        let connection = Connection::open(&db_path).expect("Cannot open database file");

        let db = Storage {
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

    pub fn get_initial_value(&self) -> i64 {
        self.connection
            .query_row("SELECT value FROM counter WHERE id = 1", [], |row| {
                row.get(0)
            })
            .unwrap_or(0)
    }

    pub fn persist_counter(&self, counter: i64) -> rusqlite::Result<()> {
        self.connection.execute(
            "UPDATE counter
            SET value = ?1
            WHERE id=1",
            (counter,),
        )?;
        Ok(())
    }
}
