pub(crate) struct Help {
    pub(crate) header: Vec<String>,
    pub(crate) rows: Vec<Vec<String>>,
}

impl Help {
    pub(super) fn new() -> Help {
        let header = vec!["When", "Key", "Command", "Actions"];
        let header = header.iter().map(|s| s.to_string()).collect();

        let rows = vec![
            vec!["All", "q", "",  "quit dot-viewer"],
            vec!["", "esc", "",  "go back to Normal mode"],
            vec!["Normal", "c", "",  "close the current tab (view)"],
            vec!["", "h/l", "",  "move focus between current, prevs, nexts list"],
            vec!["", "j/k", "",  "traverse in focused list"],
            vec!["", "tab/backtab", "",  "move between tabs"],
            vec!["Search", "/", "", "go to fuzzy Search mode"],
            vec!["", "r", "", "go to regex Search mode"],
            vec!["", "tab", "", "autocomplete search keyword"], 
            vec!["", "enter", "", "apply search"],
            vec!["Command", ":", "", "go to Command mode"],
            vec!["", "", "filter", "apply filter on current matches"],
            vec!["", "", "neighbors [depth]", "export [depth] neighbors of the current node to dot"],
            vec!["", "", "export", "export the current tab (view) to dot"],
            vec!["", "", "xdot", "launch xdot on exports/current.dot"],
            vec!["", "", "subgraph", "go to subgraph Popup mode"],
            vec!["", "tab", "", "autocomplete command"],
            vec!["", "enter", "", "execute command"],
            vec!["Subgraph Popup", "h/l", "", "open/close tree"],
            vec!["", "j/k", "", "traverse tree"],
            vec!["", "enter", "", "change root to the selected subgraph"],
        ];
        let rows = rows.iter().map(|row| row.iter().map(|s| s.to_string()).collect()).collect();

        Help { header, rows }
    }
}
