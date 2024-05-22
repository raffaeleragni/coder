#[derive(Debug)]
pub struct Game {
    pub target_text: String,
    pub typed_text: String,
    done: bool,
    score: i32,
    position: usize,
}

impl Game {
    pub fn new(target_text: &str) -> Self {
        Self {
            target_text: target_text.to_string(),
            typed_text: String::new(),
            done: false,
            score: 0,
            position: 0,
        }
    }

    pub fn key_pressed(&mut self, k: char) {
        self.typed_text.push(k);
        if let Some(c) = self.target_text.chars().nth(self.position) {
            if c == k {
                self.score += 1;
            }
        }
        self.position += 1;
        self.done = self.typed_text.eq(&self.target_text);
    }

    pub fn undo(&mut self) {
        self.score -= 1;
        self.position -= 1;
        self.typed_text.pop();
        self.done = self.typed_text.eq(&self.target_text);
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn done(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod test {
    use super::Game;

    #[test]
    fn test_initial() {
        let game = Game::new("test");
        assert_eq!(game.score(), 0);
        assert!(!game.done());
    }

    #[test]
    fn test_advancement() {
        let mut game = Game::new("test");
        game.key_pressed('t');
        assert_eq!(game.score(), 1);
        assert!(!game.done());
    }

    #[test]
    fn test_undo() {
        let mut game = Game::new("test");
        game.key_pressed('t');
        game.undo();
        assert_eq!(game.score(), 0);
        assert!(!game.done());
    }

    #[test]
    fn test_wrong_key() {
        let mut game = Game::new("test");
        game.key_pressed('a');
        game.undo();
        assert_eq!(game.score(), -1);
        assert!(!game.done());
    }

    #[test]
    fn test_full_word() {
        let mut game = Game::new("test");
        game.key_pressed('t');
        game.key_pressed('e');
        game.key_pressed('s');
        game.key_pressed('t');
        assert_eq!(game.score(), 4);
        assert!(game.done());
    }
}
