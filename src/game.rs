#[derive(Debug)]
pub struct Game {
    pub target_text: String,
    pub typed_text: String,
}

impl Game {
    pub fn new(target_text: &str) -> Self {
        Self {
            target_text: target_text.to_string(),
            typed_text: String::new(),
        }
    }

    pub fn key_pressed(&mut self, k: char) {
        self.typed_text.push(k);
    }
}
