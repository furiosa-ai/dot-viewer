use clap::{ Arg, Command };
use crate::app::utils::list::StatefulList;
use dot_graph::{
    parser::parse,
    structs::Graph,
};

pub enum Mode {
    Normal,
    Command,
}

pub struct App {
    pub quit: bool,
    pub mode: Mode,

    pub commands: Command,
    pub input: String, 
    pub errormsg: Option<String>,
    pub history: Vec<String>,

    pub graph: Graph,
    pub nodes: StatefulList<String>,
}

impl App {
    pub fn new(path: &str) -> App{
        let graph = parse(path); 
        let nodes: Vec<String> = graph.nodes.iter().map(|n| n.id.clone()).collect(); 

        App {
            quit: false,
            mode: Mode::Normal,
            commands: Self::commands(),
            input: String::from(""),
            history: Vec::new(),
            errormsg: None,
            graph,
            nodes: StatefulList::with_items(nodes),
        }
    }

    // command definition (in clap)
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
}
