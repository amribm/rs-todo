use rs_todo::command;

fn main() {
    let matches = command().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let todos = sub_matches
                .get_many::<String>("TODO")
                .unwrap_or_default()
                .map(|v| v.as_str())
                .collect::<Vec<_>>();

            println!("todos: {:?}", todos)
        }
        Some(("list", _)) => {}
        Some(("rm", _)) => {}
        Some(("done", _)) => {}
        // Some(("raw", _)) => {}
        Some(("edit", _)) => {}
        Some(("sort", _)) => {}
        Some(("reset", _)) => {}
        Some(("restore", _)) => {}
        _ => println!("hello world!"),
    }
}
