use crate::window::app::App;
use std::marker::PhantomData;

use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use super::Component;

pub struct Header<'a> {
    marker: PhantomData<&'a ()>,
}
impl<'a> Component<'a> for Header<'a> {
    type Output = Paragraph<'a>;
    fn new(_app: &mut App) -> Self::Output {
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C`, or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
    ",
        ))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::LightYellow))
        .alignment(Alignment::Center)
    }
}
