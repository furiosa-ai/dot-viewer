use std::collections::VecDeque;
use crate::repl::context::Context;
use crate::repl::command;

pub fn eval(command: &str, context: &mut Context) -> String {
    let (command, arguments) = parse(command);
    
    match command {
        "open" => command::open(arguments[0], context),
        "show" => command::show(context), 
        "export" => command::export(arguments[0], context), 
        "render" => command::render(context),
        "goto" => command::goto(arguments[0], context),
        "depth" => command::depth(arguments[0].parse::<u8>().unwrap(), context),
        _ => format!("Unknown command {} with arguments {:?}", command, arguments)
    }
}

fn parse(command: &str) -> (&str, Vec<&str>) {
    let mut tokens: VecDeque<&str> = command.split_whitespace().collect();
    let command = tokens.pop_front().unwrap();
    let arguments: Vec<&str> = tokens.iter().cloned().collect();

    (command, arguments)
}
