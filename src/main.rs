extern crate dot_viewer;

use dot_viewer::repl;
use dot_viewer::repl::context;
use rustyline::error::ReadlineError;
use rustyline::{ Editor, Result };

fn main() -> Result<()> {
    let mut context: Option<context::Context> = None;

    let mut repl = Editor::<()>::new()?;
    if repl.load_history("history.txt").is_err() {
        println!("No previous history"); 
    }
    loop {
        let line = repl.readline(">> ");
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
