pub(crate) struct Help {
    pub(crate) header: Vec<String>,
    pub(crate) rows: Vec<Vec<String>>,
}

impl Help {
    pub(super) fn new() -> Help {
        let header = vec!["Category", "Key", "Mode", "Actions"];
        let header = header.iter().map(|s| s.to_string()).collect();

        let rows = vec![
            vec!["General", "q", "All", "quit dot-viewer"],
            vec!["", "esc", "All", "go back to Nav mode"],
            vec!["", "c", "Nav", "close the current tab"],
            vec!["", "left/right", "Nav", "move focus between current, prevs, nexts list"],
            vec!["", "tab/backtab", "Nav", "move between tabs"],
            vec!["", "tab", "Match", "autocomplete"],
            vec!["Mode Switch", "s", "Nav", "go to subgraph tree Popup mode"],
            vec!["", "/", "Nav", "go to fuzzy Search mode"],
            vec!["", "r", "Nav", "go to regex Search mode"],
            vec!["", "f", "Nav", "go to prefix Filter mode"],
            vec!["Application", "enter", "Nav/Search", "go to selected node"],
            vec!["", "", "Filter", "apply filter with entered prefix"],
            vec!["", "", "Subgraph", "extract currently selected subgraph"],
            vec!["Exports", "e", "Nav", "export the current tab to dot"],
            vec!["", "0-9", "Nav", "export up to n neighbors of the current node to dot"],
            vec!["", "x", "Nav", "launch xdot on exports/current.dot"],
        ];
        let rows = rows.iter().map(|row| row.iter().map(|s| s.to_string()).collect()).collect();

        Help { header, rows }
    }
}
