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

fn commands() -> ClapCommand {
    ClapCommand::new("dot-viewer")
        .multicall(true)
        .disable_help_subcommand(true)
        .subcommand_required(true)
        .subcommand(
            ClapCommand::new("neighbors")
                .arg(Arg::new("depth").value_parser(clap::value_parser!(usize))),
        )
        .subcommand(ClapCommand::new("export").arg(Arg::new("filename")))
        .subcommand(ClapCommand::new("xdot").arg(Arg::new("filename")))
        .subcommand(ClapCommand::new("filter"))
        .subcommand(ClapCommand::new("help"))
        .subcommand(ClapCommand::new("subgraph"))
}

impl Command {
    pub(crate) fn parse(input: &String) -> Command {
        let inputs: Vec<&str> = input.split_whitespace().collect();

        match commands().try_get_matches_from(inputs) {
            Ok(matches) => match matches.subcommand() {
                Some(("neighbors", matches)) => {
                    let depth = matches.get_one::<usize>("depth").copied();
                    let neigbors = Neighbors { depth };

                    Command::Neighbors(neigbors)
                }
                Some(("export", matches)) => {
                    let filename = matches.get_one::<String>("filename").cloned();
                    let export = Export { filename };

                    Command::Export(export)
                }
                Some(("xdot", matches)) => {
                    let filename = matches.get_one::<String>("filename").cloned();
                    let xdot = Xdot { filename };

                    Command::Xdot(xdot)
                }
                Some(("filter", _)) => Command::Filter,
                Some(("help", _)) => Command::Help,
                Some(("subgraph", _)) => Command::Subgraph,
                _ => unreachable!(),
            },
            Err(_) => Command::NoMatch,
        }
    }
}

impl CommandTrie {
    pub(crate) fn new() -> CommandTrie {
        let cmds = ["neighbors", "export", "xdot", "filter", "help", "subgraph"]
            .map(String::from)
            .to_vec();
        let trie_cmd = Trie::new(&cmds);

        let empty = Vec::new();
        let _trie_arg = Trie::new(&empty);

        CommandTrie { trie_cmd, _trie_arg }
    }
}
