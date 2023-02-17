use crate::viewer::utils::Trie;
use clap::builder::{Arg, Command as ClapCommand};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Command {
    Neighbors(Neighbors),
    Filter,
    Help,
    Subgraph,
    Export,
    Xdot,
    NoMatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Neighbors {
    pub(crate) depth: Option<usize>,
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
        .subcommand(ClapCommand::new("filter"))
        .subcommand(
            ClapCommand::new("neighbors")
                .arg(Arg::new("depth").value_parser(clap::value_parser!(usize))),
        )
        .subcommand(ClapCommand::new("help"))
        .subcommand(ClapCommand::new("subgraph"))
        .subcommand(ClapCommand::new("export"))
        .subcommand(ClapCommand::new("xdot"))
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
                Some(("filter", _)) => Command::Filter,
                Some(("help", _)) => Command::Help,
                Some(("subgraph", _)) => Command::Subgraph,
                Some(("export", _)) => Command::Export,
                Some(("xdot", _)) => Command::Xdot,
                _ => unreachable!(),
            },
            Err(_) => Command::NoMatch,
        }
    }
}

impl CommandTrie {
    pub(crate) fn new() -> CommandTrie {
        let cmds = ["neighbors", "filter", "help", "subgraph", "export", "xdot"]
            .map(String::from)
            .to_vec();
        let trie_cmd = Trie::new(&cmds);

        let empty = Vec::new();
        let _trie_arg = Trie::new(&empty);

        CommandTrie { trie_cmd, _trie_arg }
    }
}
