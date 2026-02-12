use super::traits::Database;
use crate::core::agenda::{Activity, Priority};
use chrono::{DateTime, Local, TimeZone};
use rusqlite::{Connection, Error};
use std::path::Path;

pub struct ActivityDatabase {
    conn: Connection,
}

impl ActivityDatabase {
    pub fn new(database_path: &str) -> Self {
        ActivityDatabase {
            conn: ActivityDatabase::connect(database_path)
                .expect("Failure to open the activity database."),
        }
    }
    pub fn insert(&self, activity: &Activity) -> Result<(), Error> {
        self.conn.execute(
            "INSERT INTO activities (title, start, end, description, priority) VALUES (?1, ?2, ?3, ?4, ?5)",
            (activity.title(), activity.start().timestamp(), activity.end().timestamp(), activity.description(), activity.priority().to_string()),
        )?;
        Ok(())
    }
    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}

impl Database for ActivityDatabase {
    fn init(&self) -> Result<(), Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS activities (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                start INTEGER NOT NULL,
                end INTEGER NOT NULL,
                description TEXT,
                priority TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_activities_start ON activities(start)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_activities_end ON activities(end)",
            [],
        )?;
        Ok(())
    }

    fn connect(database_path: &str) -> Result<Connection, Error> {
        let conn = Connection::open(database_path)?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn activity_database_init() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let db = ActivityDatabase::new(temp_file.path().to_str().unwrap());
        db.init().unwrap();
        assert!(db.conn().table_exists(None, "activities").unwrap());
        assert!(db
            .conn()
            .column_exists(None, "activities", "title")
            .unwrap());
        assert!(db
            .conn()
            .column_exists(None, "activities", "start")
            .unwrap());
        assert!(db.conn().column_exists(None, "activities", "end").unwrap());
        assert!(db
            .conn()
            .column_exists(None, "activities", "priority")
            .unwrap());
    }

    #[test]
    fn activity_database_insert() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let db = ActivityDatabase::new(temp_file.path().to_str().unwrap());
        db.init().unwrap();
        let event = Activity::new(
            "dummy event".to_string(),
            Local::now(),
            Local::now(),
            "optional desc".to_string(),
            Priority::Low,
            (0, 0),
        );
        db.insert(&event).unwrap();
    }
}
