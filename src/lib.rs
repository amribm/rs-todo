type Error = Box<dyn std::error::Error>;

use clap::{arg, command, ArgAction, Command};

const EXECUTABLE_NAME: &str = "rs-todo";

pub fn command() -> Command {
    command!()
        .bin_name(EXECUTABLE_NAME)
        .about("Simple Todo app")
        .subcommand(
            Command::new("add").arg(
                arg!([TODO])
                    .help("todos to append")
                    .action(ArgAction::Append),
            ),
        )
}

struct Todo {
    todos: Vec<String>,
    todo_path: String,
    todo_backup: String,
    no_backup: String,
}

impl Todo {
    fn new() -> Result<Todo, Error> {}
}
