use crate::window::app::App;
use ratatui::widgets::Widget;
pub mod chart;
pub mod counter_paragraph;
pub mod files;
pub mod header;

pub trait Component<'a> {
    type Output: Widget;
    fn new(_app: &'a mut App) -> Self::Output;
}
