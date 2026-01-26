use rusqlite::{Connection, Error};

pub trait Database {
    fn init(&self) -> Result<(), Error>;
    fn connect(&self) -> Result<Connection, Error>;
}
