use std::marker::PhantomData;

use super::Component;
use crate::{config::SortType, window::app::App};
use ratatui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Cell, Row, Table},
};

pub struct FilesTable<'a> {
    marker: PhantomData<&'a ()>,
}

impl<'a> Component<'a> for FilesTable<'a> {
    type Output = Table<'a>;
    fn new(app: &mut App) -> Self::Output {
        app.file_summs.sort_by(|a, b| match app.sort_file_summs {
            SortType::Asc => a.total_changes.abs().cmp(&b.total_changes.abs()),
            SortType::Desc => b.total_changes.abs().cmp(&a.total_changes.abs()),
        });

        let (first, rest) = "file diff summary".split_at(1);
        let title = Line::from(vec![first.light_red(), rest.light_blue()]);

        let rows: Vec<Row<'_>> = app
            .file_summs
            .iter()
            .map(|f| {
                Row::new(vec![
                    Cell::new(f.path.to_string()).style(Style::default().fg(Color::White)),
                    Cell::new(f.total_changes.to_string())
                        .style(Style::default().add_modifier(Modifier::BOLD)),
                    Cell::new(format!("{}+", f.inserted.to_string()))
                        .style(Style::default().fg(Color::Green)),
                    Cell::new(format!("{}-", f.deleted.to_string()))
                        .style(Style::default().fg(Color::Red)),
                ])
            })
            .collect();

        Table::new(
            rows,
            [
                Constraint::Percentage(70),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ],
        )
        .column_spacing(1)
        .style(Style::new().fg(Color::Yellow))
        .header(
            Row::new(vec!["path", "total change", "insertion", "deletion"])
                .style(Style::new().bold())
                .bottom_margin(1),
        )
        .highlight_style(Style::new().reversed())
        // .highlight_symbol(">>")
        .block(
            Block::default()
                .fg(Color::Yellow)
                .borders(Borders::ALL)
                .title(title)
                .title_alignment(Alignment::Center),
        )
    }
}
