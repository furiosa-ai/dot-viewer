use std::collections::VecDeque;
use crate::repl::context::Context;
use crate::repl::command;
use crate::repl::error::ReplError;

pub fn eval(command: &str, context: &Option<Context>) -> Result<(String, Option<Context>), ReplError> {
    let (command, arguments) = parse(command);
    
    match command {
        "open" => if arguments.len() == 1 {
            command::open(arguments[0], context)
        } else {
            Err(ReplError::ArgumentError)
        },
        "show" => command::show(context), 
        "export" => if arguments.len() == 1 {
            command::export(arguments[0], context)
        } else {
            Err(ReplError::ArgumentError)
        },
        "render" => command::render(context),
        "goto" => if arguments.len() == 1 {
            command::goto(arguments[0], context)
        } else {
            Err(ReplError::ArgumentError)
        },
        "depth" => if arguments.len() == 1 {
            match arguments[0].parse::<u8>() {
                Ok(depth)=> command::depth(depth, context),
                Err(_) => Err(ReplError::ArgumentError)
            }
        } else {
            Err(ReplError::ArgumentError)
        },
        _ => Err(ReplError::UnknownCommandError(command.to_string(), arguments.iter().map(|s| s.to_string()).collect()))
    }
}

fn parse(command: &str) -> (&str, Vec<&str>) {
    let mut tokens: VecDeque<&str> = command.split_whitespace().collect();
    let command = tokens.pop_front().unwrap();
    let arguments: Vec<&str> = tokens.iter().cloned().collect();

    (command, arguments)
}
