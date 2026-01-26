use super::traits::Database;
use chrono::{DateTime, Local, TimeZone};
use rusqlite::{params, Connection, Error};
use rusqlite::types::{ToSql, FromSql};
use crate::core::agenda::{Priority, Activity};

pub struct ActivityDatabase {
    path: String,
}

impl ActivityDatabase {
    pub fn new(database_path: &str) -> Self {
        Self {
            path: database_path.to_string(),
        }
    }

    pub fn insert(&self, activity: &Activity) -> Result<(), Error>{
        let conn = self.connect()?;
        conn.execute(
            "INSERT INTO activities (title, start, end, description, priority) VALUES (?1, ?2, ?3, ?4, ?5)",
            (activity.title(), activity.start().timestamp(), activity.end().timestamp(), activity.description(), activity.priority().to_string()),
        )?;
        Ok(())
    }
}

impl Database for ActivityDatabase {
    fn init(&self) -> Result<(), Error>{
        let conn = self.connect()?;
        conn.execute(
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
        assert!(conn.as_ref().expect("Could not access the database").column_exists(None, "activities", "start").unwrap());
        assert!(conn.as_ref().expect("Could not access the database").column_exists(None, "activities", "end").unwrap());
        assert!(conn.as_ref().expect("Could not access the database").column_exists(None, "activities", "priority").unwrap());
    }

    #[test]
    fn activity_database_insert(){
        let db = ActivityDatabase::new("./tests/data.sqlite");
        db.init();
        let conn = db.connect();
        let event = Activity::new("dummy event".to_string(), Local::now(), Local::now(), "optional desc".to_string(), Priority::Low, (0, 0));
        db.insert(&event);
    }
}
