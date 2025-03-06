// Utilities to parse .md files and returns them in useful objects.
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::app::agenda::{Activity, Agenda};

pub trait Parser<T> {
    fn parse(filepath: &str) -> Result<Vec<T>, io::Error>;
}

pub struct AgendaParser;

// Agenda file structure:
// title: string
// start: date with timezone
// end: date with timezone (optional)
// description: string (optional)
// priority: has to match Priority enum
impl Parser<Activity> for AgendaParser {
    fn parse(filepath: &str) -> Result<Vec<Activity>, io::Error> {
        let content = Vec::new();
        let path = Path::new(filepath);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines(){
            println!("{}", line?);
        }
        Ok(content)
    }
}
