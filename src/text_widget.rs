use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
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
        let spans: Vec<Span> = (&*state).into();
        let text: Text = vec![Line::from(spans)].into();
        let l = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Min(1)])
            .split(area);
        Paragraph::new(text).render(l[0], buf);
        Paragraph::new(Line::from(vec![
            "score: ".into(),
            state.score().to_string().into(),
        ]))
        .render(l[1], buf);
    }
}

impl<'a> From<&Game> for Vec<Span<'a>> {
    fn from(game: &Game) -> Self {
        if game.done() {
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
