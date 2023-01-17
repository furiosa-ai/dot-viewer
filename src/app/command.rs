use clap::{ Arg, Command };
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

    pub fn exec(&mut self, command: String) {
        // create clap commands and parse given command
        let commands: Vec<&str> = command.split_whitespace().collect();
        
        // parse command using clap
        let parser = Self::commands();
        let cmd = parser.try_get_matches_from(commands);

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
                // TODO merge below three lines into a function
                self.all.state.select(Some(*idx));
                self.prevs();
                self.nexts();
                None
            },
            None => Some(format!("Err: no such node {:?}", id))
        }
    }

    fn render(&mut self) -> Option<String> {
        Some(format!("render"))
    }
}
