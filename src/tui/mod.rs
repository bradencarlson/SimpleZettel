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
use crate::config::{ZkCmd, ZkConfig};

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
    EditCurrentCard,
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
    partial_cmd: String,
    config: ZkConfig,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let msg = self.get_message()?;
            self.handle_messages(msg, terminal);
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

    pub fn set_config(&mut self, config: ZkConfig) {
        self.config = config;
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

    pub fn get_selected_card(&self) -> Option<&ZkCard> {
        match self.card_list {
            Some(ref list) => {
                Some(&list[self.selected_index])
            },
            None => None
        }
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

    fn edit_card(&mut self) {
        match self.card_list {
            Some(ref v) => {v[self.selected_index].edit(&self.config.get_editor()).unwrap();},
            None => {}
        };
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

    fn get_message(&mut self) -> io::Result<Vec::<Message>> {
        let mut v = Vec::<Message>::new();
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match self.state {
                    State::DisplayAllCards => {
                        match key_event.code {
                            KeyCode::Char('j') => v.push(Message::IncrementSelected),
                            KeyCode::Char('q') => v.push(Message::Exit),
                            KeyCode::Char('k') => v.push(Message::DecrementSelected),
                            KeyCode::Char(':') => v.push(Message::CmdMode),
                            KeyCode::Enter => v.push(Message::ShowCard),
                            _ => v.push(Message::NoMessage)
                        };
                    },
                    State::ShowCard => {
                        match key_event.code {
                            KeyCode::Esc => v.push(Message::DisplayAllCards),
                            KeyCode::Char('q') => v.push(Message::Exit),
                            KeyCode::Char('e') => v.push(Message::EditCurrentCard),
                            _ => v.push(Message::NoMessage)
                        };
                    },
                    State::EnteringCmd => {
                        match key_event.code {
                            KeyCode::Esc => v.push(Message::DisplayAllCards),
                            KeyCode::Backspace => v.push(Message::DeleteChar),
                            KeyCode::Char(a) => v.push(Message::AppendToCmd(a)),
                            KeyCode::Enter => {
                                v.push(Message::ExecuteCmd);
                                v.push(Message::DisplayAllCards);
                            },
                            _ => v.push(Message::NoMessage),
                        }
                    }
                }
            },
            _ => {}
        };
        Ok(v)
    }

    fn handle_messages(&mut self, messages: Vec::<Message>, terminal: &mut DefaultTerminal) {
        for msg in messages.iter() {
            match msg {
                Message::AppendToCmd(a) => self.append_to_cmd(*a),
                Message::CmdMode => self.change_state(State::EnteringCmd),
                Message::DecrementSelected => self.decrement_selected(),
                Message::DeleteChar => self.delete_char(),
                Message::DisplayAllCards => self.change_state(State::DisplayAllCards),
                Message::EditCurrentCard => {
                    self.edit_card();
                    terminal.clear();
                },
                Message::ExecuteCmd => self.execute_cmd(),
                Message::Exit => {
                    self.exit();
                    terminal.clear();
                },
                Message::IncrementSelected => self.increment_selected(),
                Message::NoMessage => {},
                Message::ShowCard => self.change_state(State::ShowCard),
            };
        }
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

        let refs = match self.get_selected_card() {
            Some(c) => {
                match c.get_references() {
                    Ok(v) => v.iter().map(|r| r.to_string()).collect::<String>().to_string(),
                    Err(e) => String::from("error getting refs")
                }
            },
            None => {
                String::from("no refs")
            }
        };

        Paragraph::new(card_content)
            .block(Block::bordered()
                .title(box_name))
            .wrap(Wrap { trim: true })
            .render(main[0], buf);

        Paragraph::new(refs)
            .block(Block::bordered()
                .title(" References ")
            )
            .render(main[1], buf);
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
            .title(Line::from(box_name.bold()));

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
                    line = line.blue();
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

