// Make a custom widget, list calendar, per week/month/ add event, specify date, duration, allow
// for multi-date events.
// Inspiration from lazyorg (https://github.com/HubertBel/lazyorg)
use std::{collections::BTreeMap, ffi};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, TimeZone};
use color_eyre::{owo_colors::OwoColorize, Result};
use std::str::FromStr;
use std::fmt;
use ratatui::{
    layout::{Constraint, Layout, Rect, Alignment},
    widgets::{Block, List, ListItem, Paragraph, Widget},
    prelude::Stylize,
};
use crate::utils::parser::{Parser, AgendaParser};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Priority {
    Low,
    Normal,
    Important,
}

impl FromStr for Priority {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Low" => Ok(Priority::Low),
            "Normal" => Ok(Priority::Normal),
            "Important" => Ok(Priority::Important),
            _ => Err(format!("'{}' is not a valid priority", s)),
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Normal => write!(f, "Normal"),
            Priority::Important => write!(f, "Important"),
        }
    }
}

//Add id for deletion
#[derive(Debug, Clone)]
pub struct Activity{
    _title: String,
    _start: DateTime<Local>,
    _end: DateTime<Local>,
    _description: String,
    _priority: Priority,
    _weeks: (i64, i64),
}

impl Activity{
    pub fn new(title: String, start: DateTime<Local>, end:DateTime<Local>, description: String, priority: Priority, weeks: (i64, i64)) -> Self {
        Activity{
            _title: title,
            _start: start,
            _end: end,
            _description: description,
            _priority: priority,
            _weeks: weeks,
        }
    }

    pub fn title(&self) -> &String{
        &self._title
    }

    pub fn start(&self) -> &DateTime<Local>{
        &self._start
    }

    pub fn end(&self) -> &DateTime<Local>{
        &self._end
    }

    pub fn description(&self) -> &String {
        &self._description
    }

    pub fn priority(&self) -> &Priority{
        &self._priority
    }
}

#[derive(Debug)]
pub struct Agenda{
    _activities: Vec<Activity>,
}

impl Agenda{
    pub fn activities(&self) -> &Vec<Activity> {
        &self._activities
    }

    fn from_file(filepath: &str) -> Self {
        Agenda{
            _activities: AgendaParser::parse(filepath).expect("Failed to load the agenda"),
        }
    }

    pub fn get_week_activities(&self, day: DateTime<Local>) -> BTreeMap<NaiveDate, Vec<&Activity>> {
        let days_from_monday = day.weekday().num_days_from_monday() as i64;
        let week_start = day.date_naive() - Duration::days(days_from_monday);
        let week_end = week_start + Duration::days(6);

        let mut week_map: BTreeMap<NaiveDate, Vec<&Activity>> = (0..7)
            .map(|i| (week_start + Duration::days(i), Vec::new()))
            .collect();

        for activity in &self._activities {
            let activity_start = activity.start().date_naive();
            let activity_end = activity.end().date_naive();

            if activity_end < week_start || activity_start > week_end {
                continue;
            }

            let start = activity_start.max(week_start);
            let end = activity_end.min(week_end);

            for offset in 0..=(end - start).num_days() {
                let day = start + Duration::days(offset);
                week_map.entry(day).or_default().push(activity);
            }
        }
        eprintln!("{:#?}", week_map);
        week_map
    }
}

impl Default for Agenda {
    fn default() -> Self {
        Agenda{
            _activities: AgendaParser::parse("/home/rosco/.local/share/gnosis/agenda/agenda.txt").expect("Failed to load the agenda"),
        }
    }
}

#[test]
fn get_week_events() {
    let agenda = Agenda::from_file("tests/agenda_test.txt");
    let monday : chrono::DateTime<Local> = Local.with_ymd_and_hms(2025, 1, 13, 0, 0, 0).unwrap();
    let week_activities : BTreeMap<NaiveDate, Vec<&Activity>> = agenda.get_week_activities(monday);
}
