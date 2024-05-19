use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Span, Text},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::game::Game;

#[derive(Debug, Default)]
pub struct GameDisplay;

impl StatefulWidget for GameDisplay {
    type State = Game;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let text: Text = vec![Line::from(game_into_spans(state))].into();
        Paragraph::new(text).centered().render(area, buf);
    }
}

fn game_into_spans(game: &Game) -> Vec<Span> {
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
