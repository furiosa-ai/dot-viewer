#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Navigate(Navigate),
    Input(Input),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Navigate {
    Current,
    Prevs,
    Nexts,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    Search,
    Filter,
    Regex,
}
