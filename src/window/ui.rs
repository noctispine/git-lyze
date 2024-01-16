use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Frame,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, TableState, Tabs},
};

use crate::window::components::Component;

use crate::window::{
    app::App,
    components::{chart::Chart, files::FilesTable, header::Header},
};

pub fn render_app(app: &mut App, f: &mut Frame) {
    let area = f.size();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(4),
            Constraint::Min(10),
        ])
        .split(area);

    let block = Block::default().fg(Color::Black);

    f.render_widget(block, area);

    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![first.light_red(), rest.light_blue()])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Tabs")
                .style(Style::default().fg(Color::Yellow)),
        )
        .select(app.active_tab)
        .style(Style::default())
        .highlight_style(Style::default().bold().on_black());

    f.render_widget(tabs, layout[0]);
    f.render_widget(Header::new(app), layout[1]);

    match app.active_tab {
        0 => f.render_widget(Chart::new(app), layout[2]),
        _ => {}
    };

    match app.active_tab {
        1 => f.render_stateful_widget(
            FilesTable::new(&mut app.clone()),
            layout[2],
            &mut app.file_table_state,
        ),
        _ => {}
    }
}
