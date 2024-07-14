use std::io;

use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Padding, Widget},
    Frame,
};

use crate::{game::Game, loader::Loader, text_widget::GameDisplay, tui};

#[derive(Debug)]
pub struct App {
    exit: bool,
    game: Game,
    loader: Loader,
}

impl App {
    pub fn new() -> Self {
        let mut loader = Loader::default();
        Self {
            exit: false,
            game: Game::new(loader.load_new_text()),
            loader
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
        let l = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Min(1)])
            .split(frame.size());
        let block = Block::default()
            .borders(Borders::all())
            .padding(Padding::uniform(1));
        let inner = block.inner(l[0]);
        block.render(l[0], frame.buffer_mut());
        frame.render_stateful_widget(GameDisplay, inner, &mut self.game);
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
        } else if let KeyCode::Backspace = key.code {
            self.game.undo();
        }

        if self.game.done() {
            self.game = Game::new(self.loader.load_new_text());
        }
    }
}
