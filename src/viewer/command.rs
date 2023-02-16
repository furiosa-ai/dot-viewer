use clap::builder::{Arg, Command as ClapCommand};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Command {
    Filter(Filter),
    Neighbors(Neighbors),
    Help,
    Subgraph,
    Export,
    Xdot,
    NoMatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Filter {
    pub(crate) prefix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Neighbors {
    pub(crate) depth: Option<usize>,
}

fn commands() -> ClapCommand {
    ClapCommand::new("dot-viewer")
        .multicall(true)
        .subcommand_required(true)
        .subcommand(
            ClapCommand::new("filter").arg(Arg::new("prefix"))
        )
        .subcommand(
            ClapCommand::new("neighbors").arg(Arg::new("depth"))
        )
        .subcommand(
            ClapCommand::new("help")
        )
        .subcommand(
            ClapCommand::new("subgraph")
        )
        .subcommand(
            ClapCommand::new("export")
        )
        .subcommand(
            ClapCommand::new("xdot")
        )
}

impl Command {
    pub(crate) fn parse(input: &String) -> Command {
        let inputs: Vec<&str> = input.split_whitespace().collect();

        match commands().try_get_matches_from(inputs) {
            Ok(matches) => match matches.subcommand() {
                Some(("filter", matches)) => {
                    let prefix = matches.get_one::<String>("prefix").map(|s| s.clone());
                    let filter = Filter { prefix };

                    Command::Filter(filter)
                }
                Some(("neighbors", matches)) => {
                    let depth = matches.get_one::<usize>("depth").map(|d| d.clone());
                    let neigbors = Neighbors { depth };

                    Command::Neighbors(neigbors)
                }
                Some(("help", _)) => Command::Help,
                Some(("subgraph", _)) => Command::Subgraph,
                Some(("export", _)) => Command::Export,
                Some(("xdot", _)) => Command::Xdot,
                _ => unreachable!(),
            }
            Err(_) => Command::NoMatch,
        }
    }
}
