use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::game::Game;

#[derive(Debug, Default)]
pub struct GameDisplay;

impl StatefulWidget for GameDisplay {
    type State = Game;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let l = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Min(1)])
            .split(area);
        Paragraph::new(state.target_text.clone())
            .centered()
            .render(l[0], buf);
        Paragraph::new(state.typed_text.clone())
            .centered()
            .render(l[1], buf);
    }
}
