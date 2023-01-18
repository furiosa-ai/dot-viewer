use clap::{ Arg, ArgMatches, Command, error::Error };
use crate::app::app::App;

impl App {
    // command definition (in clap)
    fn commands() -> Command {
        Command::new("dot-viewer")
            .no_binary_name(true)
            .subcommand_required(true)
            .allow_external_subcommands(true)
            .subcommand(
                Command::new("gt")
                    .arg(Arg::new("id"))
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("render"),
            )
    }

    fn parse(command: &String) -> Result<ArgMatches, Error> {
        let commands: Vec<&str> = command.split_whitespace().collect();
        
        // parse command using clap
        let parser = Self::commands();
        parser.try_get_matches_from(commands)
    }

    pub fn autocomplete(&mut self, command: String) {
        let cmd = Self::parse(&command);
        match cmd {
            Ok(cmd) => match cmd.subcommand() {
                Some(("gt", args)) => {
                    let node = args.get_one::<String>("id").unwrap();
                    if let Some(node) = self.trie.autocomplete(node) {
                        self.input = format!("gt {}", node);
                    }
                },
                _ => {}
            },
            _ => {},
        }
    }

    pub fn exec(&mut self, command: String) {
        let cmd = Self::parse(&command);
        let msg = match cmd {
            Ok(cmd) => match cmd.subcommand() {
                Some(("gt", args)) => {
                    let node = args.get_one::<String>("id").unwrap();
                    self.goto(node)
                }
                Some(("render", _)) => self.render(),
                _ => Some(format!("Err: no such command {:?}", command)),
            },
            Err(msg) => Some(format!("Err: {:?}", msg))
        };

        self.errormsg = msg;
    }

    pub fn goto(&mut self, id: &str) -> Option<String> {
        let idx = self.graph.lookup.get_by_left(id);
        match idx {
            Some(idx) => {
                self.all.select(*idx);
                self.update_list();
                None
            },
            None => Some(format!("Err: no such node {:?}", id))
        }
    }

    fn render(&mut self) -> Option<String> {
        Some(format!("render"))
    }
}
