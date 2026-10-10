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

pub enum Message {
    IncrementSelected,
    DecrementSelected,
    Exit,
    NoMessage,
}

#[derive(Debug,Default)]
pub struct App {
    exit: bool,
    current_box: Option<ZkBox>,
    state: State,
    selected_index: usize,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let msg = self.get_message()?;
            self.handle_message(msg);
        }
        Ok(())
    }

    pub fn set_box(&mut self, card: Option<ZkBox>) {
        self.current_box = card;
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn increment_selected(&mut self) {
        self.selected_index += 1;
    }
    pub fn decrement_selected(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn get_message(&mut self) -> io::Result<Message> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                return match key_event.code {
                    KeyCode::Char('q') => Ok(Message::Exit),
                    KeyCode::Char('j') => Ok(Message::IncrementSelected),
                    KeyCode::Char('k') => Ok(Message::DecrementSelected),
                    _ => Ok(Message::NoMessage)
                };
            },
            _ => {}
        };
        Ok(Message::NoMessage)
    }

    fn handle_message(&mut self, message: Message) {
        match message {
            Message::IncrementSelected => self.increment_selected(),
            Message::DecrementSelected => self.decrement_selected(),
            Message::Exit => self.exit(),
            Message::NoMessage => {},
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

        let content = match self.current_box {
            Some(ref bx) => {
                if let Ok(cards) = bx.get_cards(None) {
                    match self.get_card_list() {
                        Some(list) => Text::from(list),
                        None => Text::from("")
                    }
                    
                } else {
                    Text::from("")
                }
            },
            None => Text::from("")
        };

        let block = Block::bordered()
            .title(Line::from(box_name.bold()))
            .border_set(border::THICK);

        let side = Text::from(Line::from("side panel"));

        let main = widgets::main_layout(area);

        Paragraph::new(content)
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

    fn get_card_list(&self) -> Option<Vec::<Line>> {
        match self.current_box {
            Some(ref bx) => {
                if let Ok(cards) = bx.get_cards(None) {
                    let max = match cards.iter()
                        .map(|c| c.get_number_length())
                        .max() {
                            Some(m) => m, 
                            None => 20
                    };
                    let mut lines = Vec::<Line>::new();
                    let mut counter: usize = 0;
                    for card in cards.iter() {
                        let mut s = card.format_number(max);
                        s.push_str(card.get_header().as_str());
                        let mut line: Line = s.into();
                        if counter == self.selected_index {
                            line = line.bold();
                        }
                        lines.push(line);
                        counter += 1;
                    }
                    Some(lines)
                } else {
                    None
                }
            },
            None => {
                None
            }
        }
    }
}
