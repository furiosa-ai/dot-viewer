use std::collections::HashMap;
use crate::context::Context;
use crate::error::ViewerError;
use crate::graph::Node;
use repl_rs::{ Value, Convert };

pub fn show(_args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>, ViewerError> {
    let graph = &context.graph;
    let center = &context.center;
    let depth_limit = context.depth;

    let center_graph = graph.center_graph(center, depth_limit); 
    Ok(Some(format!(
        "{}\n{}",
        center_graph.to_console(),
        context.to_string()
    )))
}

pub fn export(_args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>, ViewerError> {
    let graph = &context.graph;
    let center = &context.center;
    let depth_limit = context.depth;

    let center_graph = graph.center_graph(center, depth_limit); 
    Ok(Some(center_graph.graph.to_dot()))
}

pub fn goto(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>, ViewerError> {
    let graph = &context.graph;
    let node = format!("{}", args["node"]);

    if graph.contains(&node) {
        context.center = Node::new(&node);
        show(HashMap::new(), context)
    } else {
        Err(ViewerError::GotoError(node))
    }
}

pub fn depth(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>, ViewerError> {
    context.depth = args["depth"].convert()?;
    show(HashMap::new(), context)
}
