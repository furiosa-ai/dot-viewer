use clap::builder::{Arg, Command as ClapCommand};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Command {
    Filter(Filter),
    NoMatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Filter {
    pub(crate) prefix: Option<String>,
}

fn commands() -> ClapCommand {
    ClapCommand::new("dot-viewer")
        .multicall(true)
        .subcommand_required(true)
        .subcommand(
            ClapCommand::new("filter").arg(Arg::new("prefix"))
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
                _ => unreachable!(),
            }
            Err(_) => Command::NoMatch,
        }
    }
}
