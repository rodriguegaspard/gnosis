// Utilities to parse text files and returns them in useful objects.
use chrono::{DateTime, Local, TimeZone};
use std::io::{self, BufRead, Error, ErrorKind};
use crate::app::agenda::{Activity, Priority};
use crate::utils::init;
use regex::{Regex, RegexSet};

pub trait Parser<T> {
    fn parse(filepath: &str) -> Result<Vec<T>, io::Error>;
}

pub struct AgendaParser;

impl Parser<Activity> for AgendaParser {
    fn parse(filepath: &str) -> Result<Vec<Activity>, io::Error> {
        let content = init::read_file(filepath)?;
        let re = Regex::new(r"(?<title>.+);(?<start>[0-9]+);(?<end>[0-9]+);(?<description>.*);(?<priority>.+)").unwrap();
        let mut result = Vec::new();
        for line in content.lines(){
            let Some(caps) = re.captures(line) else { return Err(Error::new(ErrorKind::Other, "Failed to parse the agenda"));};
            let start = Local.timestamp_opt(caps["start"].parse::<i64>().unwrap() ,0).unwrap();
            let end = Local.timestamp_opt(caps["end"].parse::<i64>().unwrap() ,0).unwrap();
            let priority: Priority = caps["priority"].parse().unwrap();  
            let activity = Activity::new(caps["title"].to_string(), start, end, caps["description"].to_string(), priority);
            result.push(activity);
        }
        Ok(result)
    }
}
