use std::io;

use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;

use crate::tui;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { exit: false }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.events()?;
        }
        Ok(())
    }

    fn render(&self, _: &mut Frame) {}

    fn events(&mut self) -> io::Result<()> {
        match event::read()? {
            event::Event::Key(key) if key.kind == KeyEventKind::Press => self.key_event(key),
            _ => {}
        }
        Ok(())
    }
    fn key_event(&mut self, key: KeyEvent) {
        if let KeyCode::Esc = key.code {
            self.exit = true
        }
    }
}
