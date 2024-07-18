use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Game {
    pub target_text: String,
    pub typed_text: String,
    pub score_events: Vec<ScoreEvent>,
    done: bool,
    score: i32,
}

#[derive(Debug, Clone)]
pub struct ScoreEvent {
    pub stamp: Instant,
    pub points: i32,
    pub total: i32,
}

impl Game {
    pub fn new(target_text: &str) -> Self {
        Self {
            target_text: target_text.to_string(),
            typed_text: String::new(),
            score_events: vec![],
            done: false,
            score: 0,
        }
    }

    pub fn key_pressed(&mut self, k: char) {
        self.typed_text.push(k);
        let mut points = 0;
        if let Some(same) = self.is_last_matching() {
            if same {
                points = 1;
            } else {
                points = -1;
            }
        }
        self.score += points;
        self.score_events.push(ScoreEvent {
            stamp: Instant::now(),
            points,
            total: self.score,
        });
        self.update_done();
    }

    pub fn undo(&mut self) {
        if self.typed_text.is_empty() {
            return;
        }
        if let Some(true) = self.is_last_matching() {
            self.score -= 1;
        }
        self.typed_text.pop();
        self.update_done();
    }

    fn is_last_matching(&mut self) -> Option<bool> {
        if let Some(c) = self.target_text.chars().nth(self.typed_text.len() - 1) {
            if let Some(k) = self.typed_text.chars().nth(self.typed_text.len() - 1) {
                return Some(c == k);
            }
        }
        None
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn done(&self) -> bool {
        self.done
    }

    fn update_done(&mut self) {
        self.done = self.typed_text.eq(&self.target_text);
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
    fn test_undo_from_initial() {
        let mut game = Game::new("test");
        game.undo();
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
        assert_eq!(game.score(), -1);
        assert!(!game.done());
    }

    #[test]
    fn test_wrong_key_and_undo() {
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
