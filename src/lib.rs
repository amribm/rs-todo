use clap::{arg, command, ArgAction, Command};
use std::env;
use std::fs::OpenOptions;
use std::io::{BufReader, Read};

type Error = Box<dyn std::error::Error>;

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

pub struct Todo {
    todos: Vec<String>,
    todo_path: String,
    todo_backup: String,
    no_backup: bool,
}

impl Todo {
    pub fn new() -> Result<Todo, Error> {
        let home = env::var("HOME")?;

        let todo_path: String = match env::var("TODO_PATH") {
            Ok(t) => t,
            Err(_) => {
                format!("{}/.todo", home)
            }
        };

        let todo_bak: String = match env::var("TODO_BACKUP_DIR") {
            Ok(b) => b,
            Err(_) => String::from("/tmp/todo.bak"),
        };

        let no_backup = env::var("TODO_BACKUP").is_ok();

        let todofile = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&todo_path)
            .expect("Can't open todo file");

        let mut buf_reader = BufReader::new(todofile);

        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents)?;

        let todo = contents.lines().map(str::to_string).collect();

        Ok(Self {
            todo_backup: todo_bak,
            todo_path,
            todos: todo,
            no_backup,
        })
    }
}
