
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyEvent};
use ratatui::{
    buffer::Buffer, 
    layout::{Constraint, Direction, Layout,Rect},
    style::Stylize,
    symbols::border,
    text::{Line,Text},
    widgets::{Block,Paragraph,Widget},
    DefaultTerminal, 
    Frame,
};

pub fn main_layout(area: Rect) -> Vec::<Rect> {
    let main = Layout::default()
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3)
        ])
        .split(area);
    vec![main[0], main[1]]
}

pub fn card_layout(area: Rect) -> Vec::<Rect> {
    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(85),
            Constraint::Percentage(15)
        ])
        .split(area);

    let side_panel = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(main[1]);

    vec![main[0], side_panel[0], side_panel[1]]
}
