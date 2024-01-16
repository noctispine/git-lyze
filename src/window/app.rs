use ratatui::widgets::TableState;

use crate::{commit::FileStatInfo, config::SortType};

#[derive(Debug, Default, Clone)]
pub struct App<'a> {
    pub should_quit: bool,
    pub counter: u8,
    pub active_tab: usize,
    pub titles: Vec<&'a str>,
    pub file_summs: Vec<&'a FileStatInfo>,
    pub sort_file_summs: SortType,
    pub file_table_state: TableState,
    pub types: Vec<(String, u64)>,
}

impl<'a> App<'a> {
    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn next_tab(&mut self) {
        self.active_tab = (self.active_tab + 1) % self.titles.len();
    }

    pub fn prev_tab(&mut self) {
        if self.active_tab > 0 {
            self.active_tab -= 1;
        } else {
            self.active_tab = self.titles.len() - 1;
        }
    }

    pub fn toggle_sort_files(&mut self) {
        if matches!(self.sort_file_summs, SortType::Asc) {
            self.sort_file_summs = SortType::Desc;
            return;
        }
        self.sort_file_summs = SortType::Asc;
    }

    pub fn table_next_item(&mut self) {
        let i = match self.file_table_state.selected() {
            Some(i) => {
                if i >= self.file_summs.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.file_table_state.select(Some(i));
    }

    pub fn table_prev_item(&mut self) {
        let i = match self.file_table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.file_summs.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.file_table_state.select(Some(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_increment_counter() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_decrement_counter() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(app.counter, 0);
    }
}
