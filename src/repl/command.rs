use std::fs;
use std::io::Write;
use crate::repl::context::Context;
use crate::graph::parser;
use crate::graph::graph::Node;

// open and parse a given dot file
pub fn open(filename: &str, context: &mut Context) -> String {
    let dot = fs::read_to_string(filename).expect("No such file");
    let graph = parser::parse(&dot);

    context.filename = String::from(filename);
    context.graph = graph.clone();
    context.centergraph.center = graph.nodes.first().unwrap().clone();
    context.centergraph.depth_limit = 1;

    String::from("Opened file")
}

// print current CenterGraph to console
pub fn show(context: &mut Context) -> String {
    let graph = &context.graph;
    let center = &context.centergraph.center;
    let depth_limit = context.centergraph.depth_limit;

    let centergraph = graph.centergraph(center, depth_limit); 
    format!(
        "{}\n{}",
        centergraph.to_console(),
        context.to_string()
    )
}

// export current CenterGraph to dot to providned filename
pub fn export(filename: &str, context: &mut Context) -> String {
    let graph = &context.graph;
    let center = &context.centergraph.center;
    let depth_limit = context.centergraph.depth_limit;

    let file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(filename.clone());
    match file {
        Ok(mut file) => {
            let centergraph = graph.centergraph(center, depth_limit); 
            match file.write_all(centergraph.graph.to_dot().as_bytes()) {
                Ok(_) => format!("CenterGraph written to {}", filename),
                Err(_) => panic!("Cannot write to file {}", filename)
            }
        },
        Err(_) => panic!("Cannot open file {}", filename)
    }
}

// render current CenterGraph by xdot
// TODO prevent launching multiple processes of xdot
pub fn render(context: &mut Context) -> String { 
    let graph = &context.graph;
    let center = &context.centergraph.center;
    let depth_limit = context.centergraph.depth_limit;

    let file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open("tmp.dot");
    match file {
        Ok(mut file) => {
            let centergraph = graph.centergraph(center, depth_limit); 
            match file.write_all(centergraph.graph.to_dot().as_bytes()) {
                Ok(_) => {
                    std::process::Command::new("xdot")
                        .arg("./tmp.dot")
                        .spawn()
                        .expect("failed to execute process");
                    String::from("Launched xdot")
                },
                Err(_) => panic!("Cannot write to file tmp.dot")
            }
        },
        Err(_) => panic!("Cannot open file tmp.dot")
    }
}

// change center node
pub fn goto(node: &str, context: &mut Context) -> String {
    let graph = &context.graph;

    if graph.contains(&node) {
        context.centergraph.center = Node::new(&node);
        show(context)
    } else {
        panic!("No such node");
    }
}

// change depth limit
pub fn depth(depth_limit: u8, context: &mut Context) -> String {
    context.centergraph.depth_limit = depth_limit;
    show(context)
}
