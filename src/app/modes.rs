#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Navigate(NavMode),
    Input(InputMode),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NavMode {
    Current,
    Prevs,
    Nexts,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputMode {
    Search(SearchMode),
    Filter,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchMode {
    Fuzzy,
    Regex,
}
