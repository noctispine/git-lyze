use crate::window::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Char('h') => app.prev_tab(),
        KeyCode::Char('l') => app.next_tab(),
        KeyCode::Char('f') => app.toggle_sort_files(),
        KeyCode::Char('j') | KeyCode::Down => app.table_next_item(),
        KeyCode::Char('k') | KeyCode::Up => app.table_prev_item(),
        _ => {
            if let Some(new_tab_index) = app.titles.iter().position(|t| {
                key_event.code
                    == KeyCode::Char(t.chars().next().unwrap().to_uppercase().next().unwrap())
            }) {
                app.active_tab = new_tab_index;
            };
        }
    };
}
