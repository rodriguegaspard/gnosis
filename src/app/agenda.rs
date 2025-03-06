// Make a custom widget, list calendar, per week/month/ add event, specify date, duration, allow
// for multi-date events.
// Inspiration from lazyorg (https://github.com/HubertBel/lazyorg)
use chrono::{DateTime, Local};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, List, ListItem, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use crate::utils::{parser, init};

pub enum Priority {
    Low,
    Normal,
    Important,
}

pub struct Activity{
    _title: String,
    _start: DateTime<Local>,
    _end: Option<DateTime<Local>>,
    _description: String,
    _priority: Priority,
}

impl Activity{
    pub fn new (&self, title: String, start: DateTime<Local>, end:Option<DateTime<Local>>, description: String, priority: Priority) -> Self {
        Activity{
            _title: title,
            _start: start,
            _end: end,
            _description: description,
            _priority: priority,
        }
    }
}

pub struct Agenda{
    _activities: Vec<Activity>,
}

impl Widget for Agenda {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
            let layout = Layout::horizontal([Constraint::Max(100); 7]).split(area);
            Block::bordered()
                .gray()
                .title("MONDAY".bold().into_centered_line())
                .render(layout[0], buf);
            Block::bordered()
                .gray()
                .title("TUESDAY".bold().into_centered_line())
                .render(layout[1], buf);
            Block::bordered()
                .gray()
                .title("WEDNESDAY".bold().into_centered_line())
                .render(layout[2], buf);
            Block::bordered()
                .gray()
                .title("THURSDAY".bold().into_centered_line())
                .render(layout[3], buf);
            Block::bordered()
                .gray()
                .title("FRIDAY".bold().into_centered_line())
                .render(layout[4], buf);
            Block::bordered()
                .gray()
                .title("SATURDAY".bold().into_centered_line())
                .render(layout[5], buf);
            Block::bordered()
                .gray()
                .title("SUNDAY".bold().into_centered_line())
                .render(layout[6], buf);
    }
}

impl Agenda {
    pub fn load(activities: Vec<Activity>, filepath: &str) -> Self{
        let file = init::ensure_file_exists(filepath);
        Agenda{
            _activities: activities,
        }
    }
}
