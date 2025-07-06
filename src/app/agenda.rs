// Make a custom widget, list calendar, per week/month/ add event, specify date, duration, allow
// for multi-date events.
// Inspiration from lazyorg (https://github.com/HubertBel/lazyorg)
use chrono::{DateTime, Datelike, Duration, Local, Weekday};
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

impl Widget for Agenda {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
            let layout = Layout::horizontal([Constraint::Max(100); 7]).split(area);
            self.render_week(layout.to_vec(), buf, self.get_week_activities(Local::now()));
    }
}

impl Agenda{
    pub fn activities(&self) -> &Vec<Activity> {
        &self._activities
    }

    pub fn get_week_activities(&self, day: DateTime<Local>) -> Vec<&Activity> {
        let days_from_monday = day.weekday().num_days_from_monday() as i64;
        let week_start = day.date_naive() - Duration::days(days_from_monday);
        let week_end = week_start + Duration::days(6);
        self._activities
            .iter()
            .filter(|a| {
                let activity_start = a.start().date_naive();
                let activity_end = a.end().date_naive();
                activity_end >= week_start && activity_start <= week_end
            })
        .collect()
    }

    pub fn render_week(&self, area: Vec<Rect>, buf: &mut ratatui::prelude::Buffer, activities: Vec<&Activity>){
        let day_names = ["MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY", "SUNDAY"];
        // Extract activities per day and create renderable objects accordingly 
        for day in 0..7{
            Paragraph::new("testing")
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                    .gray()
                    .title(day_names[day]).bold(),
                )
                .render(area[day], buf)
            }
    }
}

impl Default for Agenda {
    fn default() -> Self {
        Agenda{
            _activities: AgendaParser::parse("/home/rosco/.local/share/gnosis/agenda/agenda.txt").expect("Failed to load the agenda"),
        }
    }
}
