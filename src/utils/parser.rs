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
            match re.captures(line){
                Some(caps) => {
                    let start = Local.timestamp_opt(caps["start"].parse::<i64>().unwrap() ,0).unwrap();
                    let end = Local.timestamp_opt(caps["end"].parse::<i64>().unwrap() ,0).unwrap();
                    let priority: Priority = caps["priority"].parse().unwrap();  
                    let activity = Activity::new(caps["title"].to_string(), start, end, caps["description"].to_string(), priority, AgendaParser::weeks(start, end));
                    result.push(activity);
                },
                None => continue,
            }
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

    #[test]
    fn parse_agenda(){
        let activities: Vec<Activity> = AgendaParser::parse("src/utils/tests/agenda.txt").expect("Failed to open the test agenda");
        assert_eq!(2, activities.len());
        assert_eq!((
                "testevent1".to_string(), 
                Local.timestamp_opt("1577808000".parse::<i64>().unwrap(), 0).unwrap(),
                Local.timestamp_opt("1578585600".parse::<i64>().unwrap(), 0).unwrap(),
                "this event has a description".to_string(),
                Priority::Low,
        ),
        (
            activities[0].title(),
            activities[0].start(),
            activities[0].end(),
            activities[0].description(),
            activities[0].priority()
        ));
        assert_eq!((
                "testevent2".to_string(), 
                Local.timestamp_opt("1577808000".parse::<i64>().unwrap(), 0).unwrap(),
                Local.timestamp_opt("1578585600".parse::<i64>().unwrap(), 0).unwrap(),
                "".to_string(),
                Priority::Low,
        ),
        (
            activities[1].title(),
            activities[1].start(),
            activities[1].end(),
            activities[1].description(),
            activities[1].priority()
        ));
    }
}
