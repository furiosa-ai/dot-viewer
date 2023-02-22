use rayon::prelude::*;
use tui::widgets::TableState;

pub(crate) struct Table {
    pub state: TableState,
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(header: &[&str], rows: &[&[&str]]) -> Self {
        let mut state = TableState::default();

        if !rows.is_empty() {
            state.select(Some(0));
        }

        let header: Vec<String> = header.par_iter().map(|s| s.to_string()).collect();

        let rows: Vec<Vec<String>> =
            rows.par_iter().map(|row| row.iter().map(|s| s.to_string()).collect()).collect();

        Self { state, header, rows }
    }

    pub fn next(&mut self) {
        let i = (self.state.selected())
            .map(|i| if i >= self.rows.len() - 1 { 0 } else { i + 1 })
            .unwrap_or(0);

        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = (self.state.selected())
            .map(|i| if i == 0 { self.rows.len() - 1 } else { i - 1 })
            .unwrap_or(0);

        self.state.select(Some(i));
    }
}
