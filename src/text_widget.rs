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
#[derive(Debug, Default)]
pub struct HistoryDisplay;

impl StatefulWidget for GameDisplay {
    type State = Game;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        render_single_line(state, area, buf);
    }
}

impl StatefulWidget for HistoryDisplay {
    type State = Vec<Game>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let line_constraints = state
            .iter()
            .map(|_| Constraint::Length(1))
            .collect::<Vec<Constraint>>();
        let l = Layout::default()
            .direction(Direction::Vertical)
            .constraints(line_constraints)
            .split(area);
        for (i, line) in state.iter_mut().enumerate() {
            render_single_line(line, l[i], buf);
        }
    }
}

fn render_single_line(line: &mut Game, area: Rect, buf: &mut Buffer) {
    let spans: Vec<Span> = (&*line).into();
    let text: Text = vec![Line::from(spans)].into();
    let l = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length(3), Constraint::Min(1)])
        .split(area);
    Paragraph::new(Line::from(vec![line.score().to_string().into()])).render(l[0], buf);
    Paragraph::new(text).render(l[1], buf);
}

impl From<&Game> for Vec<Span<'_>> {
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
