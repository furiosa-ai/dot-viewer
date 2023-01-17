use crate::app::app::App;

impl App {
    pub fn exec(&mut self, command: String) {
        // create clap commands and parse given command
        let commands: Vec<&str> = command.split_whitespace().collect();
        
        // parse command using clap
        let parser = self.commands;
        let cmd = parser.try_get_matches_from(commands);
        let msg = match cmd {
            Ok(cmd) => match cmd.subcommand() {
                Some(("goto", args)) => {
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

    fn goto(&mut self, node: &str) -> Option<String> {
        let idx = self.graph.lookup.get_by_left(node);
        match idx {
            Some(idx) => {
                self.nodes.state.select(Some(*idx));
                None
            },
            None => Some(format!("Err: no such node {:?}", node))
        }
    }

    fn render(&mut self) -> Option<String> {
        Some(format!("render"))
    }
}
