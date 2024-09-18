use clap::{command, Command};

const EXECUTABLE_NAME: &str = "rs-todo";

fn command() -> Command {
    command!()
        .about("Simple Todo app")
        .subcommand(Command::new("list"))
}
