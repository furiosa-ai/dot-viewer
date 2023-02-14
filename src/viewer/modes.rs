#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// `Mode` represents the context that the application, `dot-viewer` is in.
pub enum Mode {
    Main(MainMode),
    Popup(PopupMode),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// In `MainMode`, users can
/// - navigate the current, prev, next lists, or
/// - type in inputs to the input form.
pub enum MainMode {
    Navigate(NavMode),
    Input(InputMode),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// In `PopupMode`, users can
/// - navigate the subgraphs, or
/// - see help message.
pub enum PopupMode {
    Tree,
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// In `NavMode`, users can navigate
/// - current nodes list,
/// - previous nodes list, and
/// - next nodes list.
pub enum NavMode {
    Current,
    Prevs,
    Nexts,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// In `InputMode`, users can type in inputs for,
/// - node search, or
/// - filter application with node id prefix
pub enum InputMode {
    Search(SearchMode),
    Filter,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// In `SearchMode`, users can search for a node with,
/// - fuzzy search against node ids, or
/// - regex search against raw node representation in dot format.
pub enum SearchMode {
    Fuzzy,
    Regex,
}
