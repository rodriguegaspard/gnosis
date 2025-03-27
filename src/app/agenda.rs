// Make a custom widget, list calendar, per week/month/ add event, specify date, duration, allow
// for multi-date events.
// Inspiration from lazyorg (https://github.com/HubertBel/lazyorg)
use chrono::{DateTime, Local};
use color_eyre::{owo_colors::OwoColorize, Result};
use std::str::FromStr;
use std::fmt;
use ratatui::{
    layout::{Constraint, Layout, Rect},
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

#[derive(Debug)]
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
            self.week(layout.to_vec(), buf, vec![]);
    }
}

impl Agenda{
    pub fn day(&self, area: Rect, buf: &mut ratatui::prelude::Buffer, day: String, activities: Vec<Activity>){
        Block::bordered()
            .gray()
            .title(day).bold()
            .render(area, buf);
    }

    pub fn week(&self, area: Vec<Rect>, buf: &mut ratatui::prelude::Buffer, activities: Vec<Activity>){
        self.day(area[0], buf, String::from("MONDAY"), vec![]);
        self.day(area[1], buf, String::from("MONDAY"), vec![]);
        self.day(area[2], buf, String::from("MONDAY"), vec![]);
        self.day(area[3], buf, String::from("MONDAY"), vec![]);
        self.day(area[4], buf, String::from("MONDAY"), vec![]);
        self.day(area[5], buf, String::from("MONDAY"), vec![]);
        self.day(area[6], buf, String::from("MONDAY"), vec![]);

    }
}

impl Default for Agenda {
    fn default() -> Self {
        Agenda{
            _activities: AgendaParser::parse("/home/rosco/.local/share/gnosis/agenda/agenda.txt").expect("Failed to load the agenda"),
        }
    }
}
