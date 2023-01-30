#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Navigate(Navigate),
    Input(Input),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Navigate {
    Current,
    Prevs,
    Nexts,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Input {
    Search(Search),
    Filter,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Search {
    Prefix,
    Regex,
}
