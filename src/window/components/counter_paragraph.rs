use crate::window::app::App;
use crate::window::components::Component;
use crossterm::style::style;
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use std::marker::PhantomData;

pub struct CounterParagraph<'a> {
    marker: PhantomData<&'a ()>,
}

impl<'a> Component<'a> for CounterParagraph<'a> {
    type Output = Paragraph<'a>;
    fn new(app: &mut App) -> Self::Output {
        Paragraph::new(format!("here is the counter: {}", app.counter))
            .style(Style::new().fg(Color::White))
            .block(
                Block::default()
                    .title("Counter")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Yellow)),
            )
            .alignment(Alignment::Center)
    }
}
