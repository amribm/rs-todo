use clap::{command, value_parser, Arg, ArgAction, Command};
use colored::*;
use std::fs::{self};
use std::io::{stdout, BufWriter, Write};
use std::path::Path;
use std::{env, io};
use thiserror::Error;

mod db;

use db::DB;

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

    #[error(transparent)]
    SqliteErr(#[from] rusqlite::Error),
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
    todos: Vec<Task>,
    db: DB,
}

impl Todo {
    pub fn new() -> Result<Todo, TodoAppError> {
        let home = env::var("HOME").or(Err(TodoAppError::HomeNotFound))?;

        let todo_path: String = match env::var("TODO_PATH") {
            Ok(t) => t,
            Err(_) => {
                format!("{}/.todo/todo.db", home)
            }
        };

        let path = Path::new(&todo_path);

        if !path.exists() {
            let _ = fs::File::create(path)?;
        }

        let db = DB::new(path)?;

        let todos = db.get_todos()?;

        Ok(Self { todos, db })
    }

    pub fn list(self) -> Result<(), TodoAppError> {
        let mut buffer = BufWriter::new(stdout());

        for task in self.todos {
            let data = if !task.done {
                format!("{}. {}\n", task.id, task.name)
            } else {
                format!("{}. {}\n", task.id, task.name.strikethrough())
            };

            buffer.write_all(data.as_bytes())?;
        }
        Ok(())
    }

    pub fn add(&mut self, todos: Vec<&str>) -> Result<(), TodoAppError> {
        if todos.is_empty() {
            return Err(TodoAppError::InvalidNumberOfArgs);
        }

        let mut last_index = self.todos.len();

        for todo in todos {
            last_index += 1;
            let task = Task::new(last_index, todo.to_string(), false);

            self.db.insert_todo(task)?;
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
            let task = &mut self.todos[ind - 1];
            task.done = true;
            self.db.edit_todo(task)?;
        }

        Ok(())
    }

    pub fn edit(&mut self, index: usize, replacement_todo: String) -> Result<(), TodoAppError> {
        let task = &mut self
            .todos
            .get_mut(index - 1)
            .ok_or(TodoAppError::IncorrectIndex(index))?;

        task.name = replacement_todo;

        self.db.edit_todo(&task)?;

        Ok(())
    }

    pub fn remove(self, args: Vec<usize>) -> Result<(), TodoAppError> {
        if args.is_empty() {
            return Err(TodoAppError::InvalidNumberOfArgs);
        }

        self.db.delete_all_rows()?;

        let mut count = 1;
        for task in self.todos {
            if args.contains(&task.id) {
                continue;
            }

            self.db.insert_todo(Task::from(task, count))?;
            count += 1;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Task {
    id: usize,
    name: String,
    done: bool,
}

impl Task {
    fn new(id: usize, name: String, done: bool) -> Task {
        Task { id, name, done }
    }

    fn from(task: Task, id: usize) -> Task {
        Task {
            id,
            name: task.name,
            done: task.done,
        }
    }
}
