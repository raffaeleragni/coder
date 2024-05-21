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
