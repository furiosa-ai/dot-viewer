use std::collections::HashMap;
use crate::context::Context;
use crate::error::ViewerError;
use crate::graph::{ Node, EdgeMap };
use repl_rs::{ Value, Convert };

pub fn show(_args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>, ViewerError> {
    let graph = &context.graph;
    let center = &context.center;
    let depth = &context.depth;

    let edgemap = EdgeMap::new(graph);
    Ok(Some(format!(
        "{}\n{}",
        edgemap.render(center, *depth),
        context.to_string()
    )))
}

pub fn goto(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>, ViewerError> {
    let graph = &context.graph;
    let depth = &context.depth;

    let node = format!("{}", args["node"]);

    if graph.contains(&node) {
        let edgemap = EdgeMap::new(graph);
        Ok(Some(edgemap.render(&Node::new(&node), *depth)))
    } else {
        Err(ViewerError::GotoError(node))
    }
}

pub fn depth(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>, ViewerError> {
    context.depth = args["depth"].convert()?;

    Ok(Some(format!("change depth to, {}", args["depth"])))
}
