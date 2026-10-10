
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

pub fn card_layout(area: Rect) -> Vec::<Rect> {
    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(65),
            Constraint::Percentage(35)
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
