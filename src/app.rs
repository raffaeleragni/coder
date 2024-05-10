use std::io;

use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;

use crate::{game::Game, text_widget::GameDisplay, tui};

#[derive(Debug)]
pub struct App {
    exit: bool,
    game: Game,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            game: Game::new("println!(\"Hello World\");"),
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        frame.render_stateful_widget(GameDisplay, frame.size(), &mut self.game);
    }

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
        } else if let KeyCode::Char(c) = key.code {
            self.game.key_pressed(c);
        }
    }
}
