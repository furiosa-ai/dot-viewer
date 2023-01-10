use std::fs;
use std::io::Write;
use crate::repl::context::Context;
use crate::graph::parser;
use crate::graph::graph::Node;

// open and parse a given dot file
pub fn open(filename: &str, _context: Option<Context>) -> (String, Option<Context>) {
    let dot = fs::read_to_string(filename).expect("No such file");
    let graph = parser::parse(&dot);

    let context = Some(
        Context {
            filename: String::from(filename),
            graph: graph.clone(),
            centergraph: graph.centergraph(graph.nodes.first().unwrap(), 1)
        }
    );

    (String::from("Opened file"), context)
}

// print current CenterGraph to console
pub fn show(context: Option<Context>) -> (String, Option<Context>) {
    let result = if let Some(ctxt) = &context {
        let centergraph = &ctxt.centergraph; 
        format!(
            "{}\n{}",
            centergraph.to_console(),
            ctxt.to_string()
        )
    } else {
        String::from("Please open a graph")
    };
   
    (result, context)
}

// export current CenterGraph to dot to providned filename
pub fn export(filename: &str, context: Option<Context>) -> (String, Option<Context>) {
    let result = if let Some(ctxt) = &context {
        let file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(filename.clone());
        match file {
            Ok(mut file) => {
                let centergraph = &ctxt.centergraph; 
                match file.write_all(centergraph.graph.to_dot().as_bytes()) {
                    Ok(_) => format!("CenterGraph written to {}", filename),
                    Err(_) => format!("Cannot write to file {}", filename)
                }
            },
            Err(_) => format!("Cannot open file {}", filename)
        }
    } else {
        String::from("Please open a graph")
    };

    (result, context)
}

// render current CenterGraph by xdot
// TODO prevent launching multiple processes of xdot
pub fn render(context: Option<Context>) -> (String, Option<Context>) { 
    let result = if let Some(ctxt) = &context {
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
                        String::from("Launched xdot")
                    },
                    Err(_) => String::from("Cannot write to file tmp.dot")
                }
            },
            Err(_) => String::from("Cannot open file tmp.dot")
        }
    } else {
        String::from("Please open a graph")
    };

    (result, context)
}

// change center node
pub fn goto(node: &str, context: Option<Context>) -> (String, Option<Context>) {
    if let Some(ctxt) = &context {
        let graph = &ctxt.graph;

        if graph.contains(&node) {
            show(Some(ctxt.center(&Node::new(node))))
        } else {
            (String::from("No such node"), context)
        }
    } else {
        (String::from("Please open a graph"), context)
    }

}

// change depth limit
pub fn depth(depth_limit: u8, context: Option<Context>) -> (String, Option<Context>) {
    if let Some(ctxt) = &context {
        show(Some(ctxt.depth_limit(depth_limit)))
    } else {
        (String::from("Please open a graph"), context)
    }

}
