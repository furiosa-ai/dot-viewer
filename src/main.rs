extern crate dot_viewer;

use std::env;
use std::fs;
use dot_viewer::graph::parser;
use dot_viewer::repl;
use rustyline::error::ReadlineError;
use rustyline::{ Editor, Result };

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // parse graph.dot in current directory
    let dot = fs::read_to_string(filename).expect("no such file");
    let graph = parser::parse(&dot);

    let mut repl = Editor::<()>::new()?;
    let mut context = repl::context::Context { filename: filename.clone(), graph: graph.clone(), center: graph.nodes.first().unwrap().clone(), depth_limit: 1 };
    loop {
        let line = repl.readline(">> ");
        match line {
            Ok(line) => {
                repl.add_history_entry(line.as_str());
                let result = repl::repl::eval(line.as_str(), &mut context);
                println!("Line: {}", result);
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("exit");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    repl.save_history("history.txt")
} 
