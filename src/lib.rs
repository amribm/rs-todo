use clap::{arg, command, ArgAction, Command};
use colored::*;
use std::env;
use std::fs::OpenOptions;
use std::io::{stdout, BufReader, BufWriter, Read, Write};

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
        .subcommand(Command::new("list"))
        .subcommand(
            Command::new("done").arg(
                arg!([INDEXES])
                    .help("todos numbers to mark as done")
                    .action(ArgAction::Append),
            ),
        )
        .subcommand(
            Command::new("edit").arg(arg!([TODO]).help("todo to edit").action(ArgAction::Append)),
        )
        .subcommand(
            Command::new("remove").arg(
                arg!([INDEXES])
                    .help("todos numbers to remove")
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

    pub fn list(self) -> Result<(), Error> {
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

    pub fn add(&mut self, todos: Vec<&str>) -> Result<(), Error> {
        if todos.is_empty() {
            return Err("need one or more todos".into());
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

    pub fn done(&mut self, indexs: Vec<usize>) -> Result<(), Error> {
        if indexs.is_empty() {
            return Err("rs-todo takes atleast one argument".into());
        }

        for ind in indexs {
            if self.todos.len() < ind - 1 {
                return Err(format!("incorrect index: {}", ind).into());
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

    pub fn edit(&mut self, args: Vec<String>) -> Result<(), Error> {
        if args.len() < 2 {
            return Err("edit expects atleast two arguments [INDEX] [TODO]".into());
        }

        let todo_path = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)?;

        let mut buffer = BufWriter::new(&todo_path);

        for (ind, todo) in self.todos.iter_mut().enumerate() {
            if !args[0].contains(&(ind + 1).to_string()) {
                let data = format!("{}\n", todo);
                buffer.write_all(data.as_bytes())?;
                continue;
            }

            let data = format!("{}{}\n", &todo[..4], args[1]);

            buffer.write_all(data.as_bytes())?;
        }

        Ok(())
    }

    pub fn remove(&mut self, args: Vec<usize>) -> Result<(), Error> {
        if args.is_empty() {
            return Err("remove expects atleast one arguments [INDEX]".into());
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

            println!("args not contains: {}", data);
            buffer.write_all(data.as_bytes())?;
        }

        Ok(())
    }
}
