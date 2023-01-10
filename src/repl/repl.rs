use std::collections::VecDeque;
use crate::repl::context::Context;
use crate::repl::command;
use crate::repl::error::ReplError;

pub fn eval(command: &str, context: &Option<Context>) -> Result<(String, Option<Context>), ReplError> {
    let (command, arguments) = parse(command);
    
    match command {
        "open" => command::open(arguments[0], context),
        "show" => command::show(context), 
        "export" => command::export(arguments[0], context), 
        "render" => command::render(context),
        "goto" => command::goto(arguments[0], context),
        "depth" => command::depth(arguments[0].parse::<u8>().unwrap(), context),
        _ => Err(ReplError::UnknownCommandError(command.to_string(), arguments.iter().map(|s| s.to_string()).collect()))
    }
}

fn parse(command: &str) -> (&str, Vec<&str>) {
    let mut tokens: VecDeque<&str> = command.split_whitespace().collect();
    let command = tokens.pop_front().unwrap();
    let arguments: Vec<&str> = tokens.iter().cloned().collect();

    (command, arguments)
}
