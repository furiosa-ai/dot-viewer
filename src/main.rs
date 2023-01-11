extern crate dot_viewer;

use std::env;
use dot_viewer::repl;
use dot_viewer::repl::context;
use dot_viewer::repl::helper;
use rustyline::error::ReadlineError;
use rustyline::{ Config, Editor, Result };

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut context: Option<context::Context> = if args.len() == 2 {
        let filename = &args[1];
        context::Context::new(filename)
    } else {
        None
    };

    match &context {
        Some(_) => println!("Opened file"),
        None => println!("Failed to open file")
    };

    let config = Config::builder().build();
    let mut repl = Editor::with_config(config)?;
    repl.set_helper(Some(helper::ReplHelper { colored_prompt: format!("\x1b[1;32mdot-viewer> \x1b[m") }));
    if repl.load_history("history.txt").is_err() {
        println!("No previous history"); 
    }
    loop {
        let line = repl.readline("dot-viewer> ");
        match line {
            Ok(line) => {
                repl.add_history_entry(line.as_str());
                let result = repl::repl::eval(line.as_str(), &context);
                match result {
                    Ok((result, ctxt)) => {
                        context = ctxt;
                        println!("{}", result)
                    },
                    Err(err) => println!("Error: {:?}", err)
                }
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
