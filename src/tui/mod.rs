use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyEvent};
use ratatui::{
    buffer::Buffer, 
    layout::Rect, 
    style::Stylize,
    symbols::border,
    text::{Line,Text},
    widgets::{Block,Paragraph,Widget},
    DefaultTerminal, 
    Frame,
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,

}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
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
        let title = Line::from(" SimpleZettel ".bold());
        let block = Block::bordered()
            .title(title)
            .border_set(border::THICK);
        let text = Text::from(Line::from("hello there!"));

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
