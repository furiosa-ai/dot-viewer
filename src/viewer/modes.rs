#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// `Mode` represents the context that the application, `dot-viewer` is in.
pub(crate) enum Mode {
    Normal,
    Command,
    Search(SearchMode),
    Popup(PopupMode),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// In `PopupMode`, users can
/// - navigate the subgraphs, or
/// - see help message.
pub(crate) enum PopupMode {
    Tree,
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// In `SearchMode`, users can search for a node with,
/// - fuzzy search against node ids, or
/// - regex search against raw node representation in dot format.
pub(crate) enum SearchMode {
    Fuzzy,
    Regex,
}
