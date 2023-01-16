use clap::{ Arg, Command };
use crate::app::app::App;

impl App {
    pub fn exec(&mut self, command: String) {
        // create clap commands and parse given command
        let commands = Self::commands();       
        let command: Vec<&str> = command.split_whitespace().collect();
        let command = commands.try_get_matches_from(command);

        match command {
            Ok(command) => match command.subcommand() {
                Some(("goto", args)) => {
                    let node = args.get_one::<String>("id").unwrap();
                    self.goto(node);
                }
                Some(("render", _)) => self.render(),
                _ => {},
            },
            // TODO print out error to tui
            Err(msg) => println!("Err: {:?}", msg)
        };
    }

    fn commands() -> Command {
        Command::new("dot-viewer")
            .no_binary_name(true)
            .subcommand_required(true)
            .allow_external_subcommands(true)
            .subcommand(
                Command::new("goto")
                    .arg(Arg::new("id"))
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("render"),
            )
    }

    fn goto(&mut self, node: &str) {
        let idx = self.graph.lookup.get_by_left(node);
        match idx {
            Some(idx) => self.nodes.state.select(Some(*idx)),
            // TODO print out error to tui
            None => println!("Err: No such node {:?}", node),
        }
    }

    fn render(&mut self) {
        println!("render");
    }
}
