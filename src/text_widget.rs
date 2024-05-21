use ratatui::{
    buffer::Buffer,
    layout::Rect,
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
        Paragraph::new(text).centered().render(area, buf);
    }
}
