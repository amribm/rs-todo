use clap::{command, value_parser, Arg, ArgAction, Command};
use colored::*;
use std::fs::OpenOptions;
use std::io::{stdout, BufReader, BufWriter, Read, Write};
use std::{env, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoAppError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("invalid argument type")]
    InvalidType,

    #[error("invalid number of arguments")]
    InvalidNumberOfArgs,

    #[error("given index: {} doesn't match any todo",.0)]
    IncorrectIndex(usize),

    #[error("env $HOME doesn't exist")]
    HomeNotFound,
}

const EXECUTABLE_NAME: &str = "rs-todo";

pub fn command() -> Command {
    command!()
        .bin_name(EXECUTABLE_NAME)
        .about("Simple Todo app")
        .subcommand(
            Command::new("add").arg(
                Arg::new("TODOS")
                    .help("todos to append")
                    .action(ArgAction::Append)
                    .required(true),
            ),
        )
        .subcommand(Command::new("list"))
        .subcommand(
            Command::new("done").arg(
                Arg::new("INDEXES")
                    .help("todos numbers to mark as done")
                    .value_parser(value_parser!(usize))
                    .action(ArgAction::Append)
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("edit")
                .arg(
                    Arg::new("INDEX")
                        .help("index of the todo to be edited")
                        .value_parser(value_parser!(usize))
                        .allow_negative_numbers(false)
                        .required(true),
                )
                .arg(
                    Arg::new("TODO")
                        .help("replacement to todo for exsiting todo")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("remove").arg(
                Arg::new("INDEXES")
                    .help("todos numbers to remove")
                    .action(ArgAction::Append)
                    .required(true),
            ),
        )
}

pub struct Todo {
    todos: Vec<String>,
    todo_path: String,
}

impl Todo {
    pub fn new() -> Result<Todo, TodoAppError> {
        let home = env::var("HOME").or(Err(TodoAppError::HomeNotFound))?;

        let todo_path: String = match env::var("TODO_PATH") {
            Ok(t) => t,
            Err(_) => {
                format!("{}/.todo", home)
            }
        };

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
            todo_path,
            todos: todo,
        })
    }

    pub fn list(self) -> Result<(), TodoAppError> {
        let mut buffer = BufWriter::new(stdout());

        let mut data = String::new();

        for (index, line) in self.todos.into_iter().enumerate() {
            let symbol = &line[..4];
            let task = &line[4..];
            if "[ ] " == symbol {
                data = format!("{}. {}\n", index + 1, task);
            } else if "[*] " == symbol {
                data = format!("{}. {}\n", index + 1, task.strikethrough());
            }

            buffer.write_all(data.as_bytes())?;
        }
        Ok(())
    }

    pub fn add(&mut self, todos: Vec<&str>) -> Result<(), TodoAppError> {
        if todos.is_empty() {
            return Err(TodoAppError::InvalidNumberOfArgs);
        }

        let todofile = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(&self.todo_path)
            .expect("can't open Todo file");

        let mut buffer = BufWriter::new(todofile);

        for todo in todos {
            if todo.trim().is_empty() {
                continue;
            }

            let line = format!("[ ] {}\n", todo);

            buffer.write_all(line.as_bytes())?;
        }
        Ok(())
    }

    pub fn done(&mut self, indexs: Vec<usize>) -> Result<(), TodoAppError> {
        if indexs.is_empty() {
            return Err(TodoAppError::InvalidNumberOfArgs);
        }

        for ind in indexs {
            if self.todos.len() < ind - 1 {
                return Err(TodoAppError::IncorrectIndex(ind));
            }
            self.todos[ind - 1] = format!("[*] {}", &self.todos[ind - 1][4..])
        }

        let todo_path = OpenOptions::new().write(true).open(&self.todo_path)?;

        let mut buffer = BufWriter::new(todo_path);

        for todo in self.todos.iter() {
            let data = format!("{}\n", &todo);
            buffer.write_all(data.as_bytes())?;
        }

        Ok(())
    }

    pub fn edit(&mut self, index: usize, replacement_todo: String) -> Result<(), TodoAppError> {
        let todo_path = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)?;

        let mut buffer = BufWriter::new(&todo_path);

        for (ind, todo) in self.todos.iter_mut().enumerate() {
            if ind + 1 != index {
                let data = format!("{}\n", todo);
                buffer.write_all(data.as_bytes())?;
                continue;
            }

            let data = format!("{}{}\n", &todo[..4], replacement_todo);

            buffer.write_all(data.as_bytes())?;
        }

        Ok(())
    }

    pub fn remove(&mut self, args: Vec<usize>) -> Result<(), TodoAppError> {
        if args.is_empty() {
            return Err(TodoAppError::InvalidNumberOfArgs);
        }

        let todo_path = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)?;

        let mut buffer = BufWriter::new(&todo_path);

        for (ind, todo) in self.todos.iter().enumerate() {
            if args.contains(&(ind + 1)) {
                continue;
            }
            let data = format!("{}\n", todo);

            buffer.write_all(data.as_bytes())?;
        }

        Ok(())
    }
}
