// Make a custom widget, list calendar, per week/month/ add event, specify date, duration, allow
// for multi-date events.
// Inspiration from lazyorg (https://github.com/HubertBel/lazyorg)
use crate::utils::parser::{AgendaParser, Parser};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, TimeZone, Timelike};
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Widget},
};
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

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

#[derive(Debug, Clone)]
pub struct Activity {
    //_id: &str,
    _title: String,
    _start: DateTime<Local>,
    _end: DateTime<Local>,
    _description: String,
    _priority: Priority,
    _weeks: (i64, i64),
}

impl Activity {
    pub fn new(
        title: String,
        start: DateTime<Local>,
        end: DateTime<Local>,
        description: String,
        priority: Priority,
        weeks: (i64, i64),
    ) -> Self {
        Activity {
            _title: title,
            _start: start,
            _end: end,
            _description: description,
            _priority: priority,
            _weeks: weeks,
        }
    }

    pub fn title(&self) -> &String {
        &self._title
    }

    pub fn start(&self) -> &DateTime<Local> {
        &self._start
    }

    pub fn end(&self) -> &DateTime<Local> {
        &self._end
    }

    pub fn description(&self) -> &String {
        &self._description
    }

    pub fn priority(&self) -> &Priority {
        &self._priority
    }
}

#[derive(Debug)]
pub struct Agenda {
    _activities: Vec<Activity>,
}

impl Agenda {
    pub fn activities(&self) -> &Vec<Activity> {
        &self._activities
    }

    pub fn from_file(filepath: &str) -> Self {
        Agenda {
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
        week_map
    }

    pub fn render_week(&self, area: Rect, buf: &mut Buffer) {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 7); 7])
            .split(area);

        let target_week: chrono::DateTime<Local> =
            Local.with_ymd_and_hms(2025, 1, 13, 0, 0, 0).unwrap();
        let week_activities: BTreeMap<NaiveDate, Vec<&Activity>> =
            self.get_week_activities(target_week);

        for (col_area, (date, activities)) in columns.iter().zip(week_activities.iter()) {
            let block = Block::default().borders(Borders::ALL).title(format!(
                "{} ~ {} activities",
                date.format("%A"),
                activities.len()
            ));
            block.render(*col_area, buf);
        }
    }
}

impl Default for Agenda {
    fn default() -> Self {
        Self::from_file(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/agenda_test.txt"
        ))
    }
}

#[test]
fn get_correct_week() {
    let agenda = Agenda::from_file("tests/agenda_test.txt");
    let target_week: chrono::DateTime<Local> =
        Local.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
    let week_activities: BTreeMap<NaiveDate, Vec<&Activity>> =
        agenda.get_week_activities(target_week);

    let monday = NaiveDate::from_ymd_opt(2025, 1, 13).unwrap();
    let sunday = NaiveDate::from_ymd_opt(2025, 1, 19).unwrap();

    let first_key = *week_activities.keys().next().unwrap();
    assert_eq!(first_key, monday);

    let last_key = *week_activities.keys().next_back().unwrap();
    assert_eq!(last_key, sunday);
}

#[test]
fn check_event_in_week() {
    let agenda = Agenda::from_file("tests/agenda_test.txt");
    let target_week: chrono::DateTime<Local> =
        Local.with_ymd_and_hms(2025, 1, 13, 0, 0, 0).unwrap();
    let week_activities: BTreeMap<NaiveDate, Vec<&Activity>> =
        agenda.get_week_activities(target_week);

    // We should have 3 activities on tuesday : Meet, Call and Span1, in that order in the .csv
    let tuesday = NaiveDate::from_ymd_opt(2025, 1, 14).unwrap();
    let tuesday_activities = week_activities.get(&tuesday).unwrap();
    assert_eq!(tuesday_activities.len(), 3);
    assert_eq!(tuesday_activities[0].title(), "Meet");
    assert_eq!(tuesday_activities[1].title(), "Call");
    assert_eq!(tuesday_activities[2].title(), "Span1");
}

#[test]
fn check_spanning_event() {
    let agenda = Agenda::from_file("tests/agenda_test.txt");
    let target_week: chrono::DateTime<Local> =
        Local.with_ymd_and_hms(2025, 1, 13, 0, 0, 0).unwrap();
    let week_activities: BTreeMap<NaiveDate, Vec<&Activity>> =
        agenda.get_week_activities(target_week);

    // Span1 should have 7 entries, one for each day of the week
    for (date, activities) in &week_activities {
        let contains_span1 = activities.iter().any(|a| a.title() == "Span1");

        assert!(
            contains_span1,
            "Date {} does not contain an activity named Span1.",
            date
        );
    }
}
