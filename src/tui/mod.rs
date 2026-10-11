use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyEvent};
use ratatui::{
    buffer::Buffer, 
    layout::{Constraint, Direction, Layout,Rect},
    style::{Style,Stylize},
    symbols::border,
    text::{Line,Text},
    widgets::{Block,Paragraph,Widget,Wrap},
    DefaultTerminal, 
    Frame,
};

use crate::boxes::{self, ZkBox, notes::ZkCard};

pub mod widgets;

use regex::Regex;

#[derive(Default,Debug)]
pub enum State {
    #[default]
    DisplayAllCards,
    ShowCard,
    EnteringCmd,
}

pub enum Message {
    AppendToCmd(char),
    CmdMode,
    DecrementSelected,
    DeleteChar,
    DisplayAllCards,
    ExecuteCmd,
    Exit,
    IncrementSelected,
    NoMessage,
    ShowCard,
}

#[derive(Debug,Default)]
pub struct App {
    exit: bool,
    current_box: Option<ZkBox>,
    card_list: Option<Vec::<ZkCard>>,
    state: State,
    selected_index: usize,
    pattern: Option<Regex>,
    partial_cmd: String
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

    pub fn set_box(&mut self, bx: Option<ZkBox>) {
        if let Some(ref bx) = bx {
            self.card_list = match bx.get_cards(None) {
                Ok(v) => Some(v),
                Err(_) => None
            };
        }
        self.current_box = bx;
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

    pub fn get_selected(&self) -> usize {
        self.selected_index
    }

    fn append_to_cmd(&mut self, c: char) {
        self.partial_cmd.push(c);
    }

    fn delete_char(&mut self) {
        self.partial_cmd.pop();
    }

    fn change_state(&mut self, state: State) {
        self.state = state;
    }

    fn execute_cmd(&mut self) {
        if let Ok(pat) = Regex::new(self.partial_cmd.as_str()) {
            if let Some(ref bx) = self.current_box {
                self.card_list = match bx.get_cards(Some(&pat)) {
                    Ok(list) => Some(list),
                    Err(_e) => None
                };
                self.selected_index = 0;
            }
        };
    }

    fn get_message(&mut self) -> io::Result<Message> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match self.state {
                    State::DisplayAllCards => {
                        return match key_event.code {
                            KeyCode::Char('j') => Ok(Message::IncrementSelected),
                            KeyCode::Char('q') => Ok(Message::Exit),
                            KeyCode::Char('k') => Ok(Message::DecrementSelected),
                            KeyCode::Char(':') => Ok(Message::CmdMode),
                            KeyCode::Enter => Ok(Message::ShowCard),
                            _ => Ok(Message::NoMessage)
                        };
                    },
                    State::ShowCard => {
                        return match key_event.code {
                            KeyCode::Esc => Ok(Message::DisplayAllCards),
                            KeyCode::Char('q') => Ok(Message::Exit),
                            _ => Ok(Message::NoMessage)
                        };
                    },
                    State::EnteringCmd => {
                        return match key_event.code {
                            KeyCode::Esc => Ok(Message::DisplayAllCards),
                            KeyCode::Backspace => Ok(Message::DeleteChar),
                            KeyCode::Char(a) => Ok(Message::AppendToCmd(a)),
                            KeyCode::Enter => Ok(Message::ExecuteCmd),
                            _ => Ok(Message::NoMessage),
                        }
                    }
                }
            },
            _ => {}
        };
        Ok(Message::NoMessage)
    }

    fn handle_message(&mut self, message: Message) {
        match message {
            Message::AppendToCmd(a) => self.append_to_cmd(a),
            Message::CmdMode => self.change_state(State::EnteringCmd),
            Message::DecrementSelected => self.decrement_selected(),
            Message::DeleteChar => self.delete_char(),
            Message::DisplayAllCards => self.change_state(State::DisplayAllCards),
            Message::ExecuteCmd => self.execute_cmd(),
            Message::Exit => self.exit(),
            Message::IncrementSelected => self.increment_selected(),
            Message::NoMessage => {},
            Message::ShowCard => self.change_state(State::ShowCard),
        };
    }

    pub fn exit(&mut self) {
        self.exit = true
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            State::DisplayAllCards => self.render_display_cards(area,buf),
            State::ShowCard => self.render_show_card(area, buf),
            State::EnteringCmd => self.render_display_cards(area,buf),
        }
    }

}

impl App {

    fn render_show_card(&self, area: Rect, buf: &mut Buffer) {
        let box_name = match self.current_box {
            Some(ref bx) => {
                let mut s = bx.name();
                s.push(' ');
                s.insert(0, ' ');
                s
            },
            None => String::from(" no current box "),
        };

        let main = widgets::card_layout(area);

        let card_content = match self.card_list {
            Some(ref list) => {
                list[self.selected_index].get_content()
            },
            None => {
                String::from("Card content unavailable")
            }
        };

        Paragraph::new(card_content)
            .block(Block::bordered()
                .title(box_name))
            .wrap(Wrap { trim: true })
            .render(main[0], buf);
    }

    fn render_display_cards(&self, area: Rect, buf: &mut Buffer) {
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
                match self.get_card_lines() {
                    Some(list) => Text::from(list),
                    None => Text::from("")
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

        let filter_block = match self.state {
            State::EnteringCmd => {
                Block::bordered()
                    .title("Filter")
                    .border_style(Style::new().white().bold())
            },
            _ => {
                Block::bordered()
                    .title("Filter")
            }
        };

        Paragraph::new(Text::from(self.partial_cmd.clone()))
            .block(filter_block)
            .render(main[1], buf);

    }

    fn get_card_lines(&self) -> Option<Vec::<Line>> {
        if let Some(ref cards) = self.card_list { 
            let max = match cards.iter()
                .map(|c| c.get_number_length())
                .max() {
                    Some(m) => m+4,
                    None => 20
            };
            let mut lines = Vec::<Line>::new();
            let mut counter: usize = 0;
            for card in cards.iter() {
                let mut s = card.format_number(max);
                s.push_str(card.get_header().as_str());
                let mut line: Line = s.into();
                if counter == self.selected_index {
                    line = line.bold().blue();
                }
                lines.push(line);
                counter += 1;
            }
            Some(lines)
        } else {
            None
        }
    }
}

#[test]
fn selected_value() {
    let mut app = App::default();
    app.increment_selected();
    assert_eq!(app.get_selected(), 1usize);
    app.increment_selected();
    assert_eq!(app.get_selected(), 2usize);
    app.decrement_selected();
    assert_eq!(app.get_selected(), 1usize);
    app.decrement_selected();
    assert_eq!(app.get_selected(), 0usize);
    app.decrement_selected();
    assert_eq!(app.get_selected(), 0usize);
}

