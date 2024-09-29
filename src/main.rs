use rs_todo::{command, Todo, TodoAppError};

fn main() -> Result<(), TodoAppError> {
    let matches = command().get_matches();

    let mut todo = match Todo::new() {
        Ok(t) => t,
        Err(e) => {
            panic!("unable to run todo manager due to e: {}", e)
        }
    };

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let todos = sub_matches
                .get_many::<String>("TODOS")
                .ok_or(TodoAppError::InvalidType)?
                .map(|v| v.as_str())
                .collect::<Vec<_>>();

            let _ = todo.add(todos).is_err_and(|e| panic!("error: {}", e));
        }
        Some(("list", _)) => {
            let _ = todo.list().is_err_and(|e| panic!("error: {}", e));
        }
        Some(("rm", _)) => {}
        Some(("done", sub_matches)) => {
            let indexes = sub_matches
                .get_many("INDEXES")
                .ok_or(TodoAppError::InvalidType)?
                .map(|x| *x)
                .collect::<Vec<_>>();
            let _ = todo.done(indexes).is_err_and(|e| panic!("error: {}", e));
        }
        Some(("edit", sub_matches)) => {
            let index = sub_matches
                .get_one("INDEX")
                .ok_or(TodoAppError::InvalidType)?;

            let replacement = sub_matches
                .get_one::<String>("TODO")
                .ok_or(TodoAppError::InvalidType)?;

            let _ = todo
                .edit(*index, replacement.to_string())
                .is_err_and(|e| panic!("error: {}", e));
        }
        Some(("sort", _)) => {}
        Some(("remove", sub_matches)) => {
            let indexes = sub_matches
                .get_many::<String>("INDEXES")
                .ok_or(TodoAppError::InvalidType)?
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let _ = todo.remove(indexes).is_err_and(|e| panic!("error: {}", e));
        }
        Some(("restore", _)) => {}
        _ => println!("hello world!"),
    }
    Ok(())
}
