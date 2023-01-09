mod parser;
mod graph;
mod command;
mod context;
mod error;

use std::fs;
use crate::context::Context;
use repl_rs::{ Command, Parameter, Result, Repl };

fn main() -> Result<()> {
    let dot = fs::read_to_string("graph.dot").expect("no such file");
    let graph = parser::parse(&dot);
    //println!("graph: {:?}", graph);

    let mut repl = Repl::new(Context { graph: graph.clone(), center: graph.nodes.first().unwrap().clone() ,depth: 1 })
        .with_name("dot-viewer")
        .with_version("dev")
        .add_command(
            Command::new("show", command::show)
                .with_help("Show graph centered at current node"),
        )
        .add_command(
            Command::new("export", command::export)
                .with_help("Export graph centered at current node to dot"),
        )
        .add_command(
            Command::new("goto", command::goto)
                .with_parameter(Parameter::new("node").set_required(true)?)?
                .with_help("Go to a node in graph"),
        )
        .add_command(
            Command::new("depth", command::depth)
                .with_parameter(Parameter::new("depth").set_required(true)?)?
                .with_help("Set visualization depth"),
        );

    repl.run()
}
