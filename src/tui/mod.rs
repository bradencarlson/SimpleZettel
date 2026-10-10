use std::io;
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

use crate::boxes::{self, ZkBox};

pub mod widgets;

use regex::Regex;

#[derive(Default,Debug)]
pub enum State {
    #[default]
    DisplayAllCards,
}

#[derive(Debug,Default)]
pub struct App {
    exit: bool,
    current_box: Option<ZkBox>,
    state: State
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn set_box(&mut self, card: Option<ZkBox>) {
        self.current_box = card;
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            },
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        };
    }

    pub fn exit(&mut self) {
        self.exit = true
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            State::DisplayAllCards => self.render_display_cards(area,buf)
        }
    }

}

impl App {

    fn render_display_cards(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" SimpleZettel ".bold());
        let box_name = match self.current_box {
                    Some(ref bx) => {
                        let mut s = bx.name();
                        s.push(' ');
                        s.insert(0, ' ');
                        s
                    },
                    None => String::from(" no current box "),
                };

        let block = Block::bordered()
            .title(Line::from(box_name.bold()))
            .border_set(border::THICK);

        let text = Text::from(Line::from("text here"));

        let side = Text::from(Line::from("side panel"));

        let main = widgets::main_layout(area);

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(main[0], buf);

        Paragraph::new(side)
            .block(Block::bordered()
                .title(Line::from(" References ".bold()))
                .border_set(border::THICK))
            .render(main[1], buf);

        Paragraph::new(Line::from("children cards"))
            .block(Block::bordered()
                .title(Line::from(" Cards ".bold()))
                .border_set(border::THICK))
            .render(main[2], buf);
    }
}
