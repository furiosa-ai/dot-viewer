use std::io::Write;
use crate::repl::context::Context;
use crate::graph::graph::Node;

// print current CenterGraph to console
pub fn show(context: &mut Context) -> String {
    let graph = &context.graph;
    let center = &context.center;
    let depth_limit = context.depth_limit;

    let center_graph = graph.center_graph(center, depth_limit); 
    format!(
        "{}\n{}",
        center_graph.to_console(),
        context.to_string()
    )
}

// export current CenterGraph to dot to providned filename
pub fn export(filename: &str, context: &mut Context) -> String {
    let graph = &context.graph;
    let center = &context.center;
    let depth_limit = context.depth_limit;

    let file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(filename.clone());
    match file {
        Ok(mut file) => {
            let center_graph = graph.center_graph(center, depth_limit); 
            match file.write_all(center_graph.graph.to_dot().as_bytes()) {
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
    let center = &context.center;
    let depth_limit = context.depth_limit;

    let file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open("tmp.dot");
    match file {
        Ok(mut file) => {
            let center_graph = graph.center_graph(center, depth_limit); 
            match file.write_all(center_graph.graph.to_dot().as_bytes()) {
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
        context.center = Node::new(&node);
        show(context)
    } else {
        panic!("No such node");
    }
}

// change depth limit
pub fn depth(depth_limit: u8, context: &mut Context) -> String {
    context.depth_limit = depth_limit;
    show(context)
}
