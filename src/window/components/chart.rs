use std::marker::PhantomData;

use ratatui::{
    layout::Direction,
    style::{Color, Style},
    widgets::{BarChart, Block, Borders},
};

use crate::window::app::App;

use super::Component;

pub struct Chart<'a> {
    marker: PhantomData<&'a ()>,
}
impl<'a> Component<'a> for Chart<'a> {
    type Output = BarChart<'a>;
    fn new(app: &'a mut App) -> Self::Output {
        BarChart::default()
            .block(
                Block::default()
                    .title("types")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Yellow)),
            )
            .data(
                &app.types
                    .iter()
                    .map(|t| (t.0.as_str(), t.1))
                    .collect::<Vec<(&str, u64)>>(),
            )
            .bar_width(2)
            .bar_style(Style::default().fg(Color::Yellow))
            .value_style(Style::default().fg(Color::White))
            .direction(Direction::Horizontal)
    }
}
