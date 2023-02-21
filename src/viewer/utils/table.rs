use tui::widgets::TableState;

pub(crate) struct Table {
    pub state: TableState,
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub(crate) fn new(header: Vec<String>, rows: Vec<Vec<String>>) -> Table {
        let mut state = TableState::default();

        if !rows.is_empty() {
            state.select(Some(0));
        }

        Table { state, header, rows }
    }

    pub(crate) fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.rows.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub(crate) fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.rows.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
