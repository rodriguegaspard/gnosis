// Utilities to parse text files and returns them in useful objects.
use chrono::{DateTime, Datelike, Local, TimeZone};
use std::io::{self, BufRead, Error, ErrorKind};
use crate::app::agenda::{Activity, Priority};
use crate::utils::init;
use regex::{Regex, RegexSet};

pub trait Parser<T> {
    fn parse(filepath: &str) -> Result<Vec<T>, io::Error>;
}

pub struct AgendaParser;

impl AgendaParser{
    fn weeks(start: DateTime<Local>, end: DateTime<Local>) -> (i64, i64){
        let current: i64 = Local::now().iso_week().week().into();
        let start_week: i64 = start.iso_week().week().into();
        let end_week: i64 = end.iso_week().week().into();
        (start_week - current, end_week - current)
    }
}

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
            let activity = Activity::new(caps["title"].to_string(), start, end, caps["description"].to_string(), priority, AgendaParser::weeks(start, end));
            result.push(activity);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn same_week_offsets() {
        let now = Local::now();
        let dt1 = now - Duration::days(now.weekday().num_days_from_monday() as i64);
        let dt2 = dt1 + Duration::days(6);
        let result = AgendaParser::weeks(dt1, dt2);
        assert_eq!((0, 0), result);
    }

    #[test]
    fn different_week_offsets() {
        let now = Local::now();
        let dt1 = now - Duration::days(now.weekday().num_days_from_monday() as i64) - Duration::days(14);
        let dt2 = now + Duration::days(14);
        let result = AgendaParser::weeks(dt1, dt2);
        assert_eq!((-2, 2), result);
    }
}
