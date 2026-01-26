use super::traits::Database;
use rusqlite::{Connection, Error};

pub struct ActivityDatabase {
    path: String,
}

impl ActivityDatabase {
    pub fn new(database_path: &str) -> Self {
        Self {
            path: database_path.to_string(),
        }
    }
}

impl Database for ActivityDatabase {
    fn init(&self) -> Result<(), Error>{
        let conn = self.connect()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS activities (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                start_unix INTEGER NOT NULL,
                end_unix INTEGER NOT NULL,
                description TEXT,
                priority TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_activities_start_unix ON activities(start_unix)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_activities_end_unix ON activities(end_unix)",
            [],
        )?;

        Ok(())
    }

    fn connect(&self) -> Result<Connection, Error>{
        let conn = Connection::open(&self.path)?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activity_database_init(){
        let db = ActivityDatabase::new("./tests/data.sqlite");
        db.init();
        let conn = db.connect();
        assert!(conn.as_ref().expect("Could not access the database").table_exists(None, "activities").unwrap());
        assert!(conn.as_ref().expect("Could not access the database").column_exists(None, "activities", "title").unwrap());
        assert!(conn.as_ref().expect("Could not access the database").column_exists(None, "activities", "start_unix").unwrap());
        assert!(conn.as_ref().expect("Could not access the database").column_exists(None, "activities", "end_unix").unwrap());
        assert!(conn.as_ref().expect("Could not access the database").column_exists(None, "activities", "priority").unwrap());
    }
}
