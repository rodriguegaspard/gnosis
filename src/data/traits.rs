use rusqlite::{Connection, Error};

pub trait Database {
    fn init(&self) -> Result<(), Error>;
    fn connect(database_path: &str) -> Result<Connection, Error>
    where
        Self: Sized;
}
