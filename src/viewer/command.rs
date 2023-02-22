use crate::viewer::utils::Trie;
use clap::builder::{Arg, Command as ClapCommand};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Command {
    Neighbors(Neighbors),
    Export(Export),
    Xdot(Xdot),
    Filter,
    Help,
    Subgraph,
    NoMatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Neighbors {
    pub(crate) depth: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Export {
    pub(crate) filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Xdot {
    pub(crate) filename: Option<String>,
}

pub(crate) struct CommandTrie {
    pub(crate) trie_cmd: Trie,
    pub(crate) _trie_arg: Trie,
}

fn subcommands() -> [ClapCommand; 6] {
    [
        ClapCommand::new("neighbors")
            .arg(Arg::new("depth").value_parser(clap::value_parser!(usize))),
        ClapCommand::new("export").arg(Arg::new("filename")),
        ClapCommand::new("xdot").arg(Arg::new("filename")),
        ClapCommand::new("filter"),
        ClapCommand::new("help"),
        ClapCommand::new("subgraph"),
    ]
}

fn commands() -> ClapCommand {
    ClapCommand::new("dot-viewer")
        .multicall(true)
        .disable_help_subcommand(true)
        .subcommand_required(true)
        .subcommands(subcommands())
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let inputs: Vec<&str> = input.split_whitespace().collect();

        match commands().try_get_matches_from(inputs) {
            Ok(matches) => match matches.subcommand() {
                Some(("neighbors", matches)) => {
                    let depth = matches.get_one::<usize>("depth").copied();
                    let neigbors = Neighbors { depth };

                    Self::Neighbors(neigbors)
                }
                Some(("export", matches)) => {
                    let filename = matches.get_one::<String>("filename").cloned();
                    let export = Export { filename };

                    Self::Export(export)
                }
                Some(("xdot", matches)) => {
                    let filename = matches.get_one::<String>("filename").cloned();
                    let xdot = Xdot { filename };

                    Self::Xdot(xdot)
                }
                Some(("filter", _)) => Self::Filter,
                Some(("help", _)) => Self::Help,
                Some(("subgraph", _)) => Self::Subgraph,
                _ => unreachable!(),
            },
            Err(_) => Self::NoMatch,
        }
    }
}

impl CommandTrie {
    pub fn new() -> CommandTrie {
        let trie_cmd = Trie::from_iter(subcommands().iter().map(|c| c.get_name().to_string()));

        let _trie_arg = Trie::from_iter([]);

        Self { trie_cmd, _trie_arg }
    }
}
