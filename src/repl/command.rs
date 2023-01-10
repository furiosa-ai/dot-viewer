use std::fs;
use std::io::Write;
use crate::repl::context::Context;
use crate::repl::error::ReplError;
use crate::graph::parser;
use crate::graph::graph::Node;

// open and parse a given dot file
pub fn open(filename: &str, _context: &Option<Context>) -> Result<(String, Option<Context>), ReplError> {
    let dot = fs::read_to_string(filename).expect("No such file");
    let graph = parser::parse(&dot);

    let context = Some(
        Context {
            filename: String::from(filename),
            graph: graph.clone(),
            centergraph: graph.centergraph(graph.nodes.first().unwrap(), 1)
        }
    );

    Ok((String::from("Opened file"), context))
}

// print current CenterGraph to console
pub fn show(context: &Option<Context>) -> Result<(String, Option<Context>), ReplError> {
    if let Some(ctxt) = &context {
        let centergraph = &ctxt.centergraph; 
        Ok((format!("{}\n{}", centergraph.to_console(), ctxt.to_string()), context.clone()))
    } else {
        Err(ReplError::NoGraphError)
    }
}

// export current CenterGraph to dot to providned filename
pub fn export(filename: &str, context: &Option<Context>) -> Result<(String, Option<Context>), ReplError> {
    if let Some(ctxt) = &context {
        let file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(filename.clone());
        match file {
            Ok(mut file) => {
                let centergraph = &ctxt.centergraph; 
                match file.write_all(centergraph.graph.to_dot().as_bytes()) {
                    Ok(_) => Ok((format!("CenterGraph written to {}", filename), context.clone())),
                    Err(_) => Err(ReplError::FileError(String::from(filename)))
                }
            },
            Err(_) => Err(ReplError::FileError(String::from(filename)))
        }
    } else {
        Err(ReplError::NoGraphError)
    }
}

// render current CenterGraph by xdot
// TODO prevent launching multiple processes of xdot
pub fn render(context: &Option<Context>) -> Result<(String, Option<Context>), ReplError> { 
    if let Some(ctxt) = &context {
        let file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open("tmp.dot");
        match file {
            Ok(mut file) => {
                let centergraph = &ctxt.centergraph; 
                match file.write_all(centergraph.graph.to_dot().as_bytes()) {
                    Ok(_) => {
                        std::process::Command::new("xdot")
                            .arg("./tmp.dot")
                            .spawn()
                            .expect("failed to execute process");
                        Ok((String::from("Launched xdot"), context.clone()))
                    },
                    Err(_) => Err(ReplError::FileError(String::from("tmp.dot")))
                }
            },
            Err(_) => Err(ReplError::FileError(String::from("tmp.dot")))
        }
    } else {
        Err(ReplError::NoGraphError)
    }
}

// change center node
pub fn goto(target: &str, context: &Option<Context>) -> Result<(String, Option<Context>), ReplError> {
    if let Some(ctxt) = &context {
        match target.parse::<u8>() {
            Ok(idx) => {
                let centergraph = &ctxt.centergraph;

                match centergraph.fwd.get(&idx) {
                    Some(node) => show(&Some(ctxt.center(node))),
                    None => Err(ReplError::NoNodeError)
                }
            },
            Err(_) => {
                let graph = &ctxt.graph;

                if graph.contains(&target) {
                    show(&Some(ctxt.center(&Node::new(target))))
                } else {
                   Err(ReplError::NoNodeError) 
                }
            }
        }
    } else {
       Err(ReplError::NoGraphError) 
    }

}

// change depth limit
pub fn depth(depth_limit: u8, context: &Option<Context>) -> Result<(String, Option<Context>), ReplError> {
    if let Some(ctxt) = &context {
        show(&Some(ctxt.depth_limit(depth_limit)))
    } else {
        Err(ReplError::NoGraphError)
    }
}
