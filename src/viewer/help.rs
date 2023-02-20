pub(super) fn header() -> Vec<String> {
    let header = vec!["When", "Key", "Command", "Actions"];
    header.iter().map(|s| s.to_string()).collect()
}

pub(super) fn rows() -> Vec<Vec<String>> {
    let rows = vec![
        vec!["Quit", "q", "", "quit dot-viewer"],
        vec!["Help", "", ":help<CR>", "help"],
        vec!["", "", "", ""],
        vec!["All", "esc", "", "go back to Normal mode"],
        vec!["Normal", "/", "", "go to fuzzy search mode"],
        vec!["Normal", "r", "", "go to regex search mode"],
        vec!["Normal", ":", "", "go to command mode"],
        vec!["", "", "", ""],
        vec!["Normal", "c", "", "close the current tab (view)"],
        vec!["", "h/l", "", "move focus between current, prevs, nexts list"],
        vec!["", "j/k", "", "traverse in focused list"],
        vec!["", "n/N", "", "go to next/previous match"],
        vec!["", "tab/backtab", "", "move between tabs"],
        vec!["Search", "tab", "", "autocomplete search keyword"],
        vec!["", "enter", "", "apply search"],
        vec!["Command", "", "filter", "apply filter on current matches"],
        vec!["", "", "neighbors [depth]", "get up to [depth] neighbors of the current node"],
        vec!["", "", "export [(opt) filename]", "export the current tab (view) to dot"],
        vec![
            "",
            "",
            "xdot [(opt) filename]",
            "launch xdot, showing the most current exported file on default",
        ],
        vec!["", "", "subgraph", "go to subgraph Popup mode"],
        vec!["", "tab", "", "autocomplete command"],
        vec!["", "enter", "", "execute command"],
        vec!["Subgraph Popup", "h/j/k/l", "", "traverse tree"],
        vec!["", "enter", "", "change root to the selected subgraph"],
    ];
    rows.iter().map(|row| row.iter().map(|s| s.to_string()).collect()).collect()
}
