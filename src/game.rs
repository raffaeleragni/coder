use ratatui::{style::Stylize, text::Span};

#[derive(Debug)]
pub struct Game {
    pub target_text: String,
    pub typed_text: String,
    pub done: bool,
}

impl Game {
    pub fn new(target_text: &str) -> Self {
        Self {
            target_text: target_text.to_string(),
            typed_text: String::new(),
            done: false,
        }
    }

    pub fn key_pressed(&mut self, k: char) {
        self.typed_text.push(k);

        self.done = self.typed_text.eq(&self.target_text);
    }

    pub fn undo(&mut self) {
        self.typed_text.pop();
    }
}

impl<'a> From<&Game> for Vec<Span<'a>> {
    fn from(game: &Game) -> Self {
        if game.done {
            return vec![Span::from(game.target_text.clone()).bold().green()];
        }
        let check = game.typed_text.as_str().chars().collect::<Vec<char>>();
        let target = game.target_text.as_str();
        let mut result = Vec::<Span>::new();
        for (i, c) in target.chars().enumerate() {
            if i >= check.len() {
                result.push(Span::from(c.to_string()).dark_gray());
            } else if check[i] == c {
                result.push(Span::from(c.to_string()).bold().white());
            } else {
                result.push(Span::from(c.to_string()).bold().red().underlined());
            }
        }
        result
    }
}
