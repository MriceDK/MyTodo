use std::env;
use todo::{data_file_path, load, save, TodoList};

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = data_file_path();
    let mut todo_list = load(&path, "Todo");
    let command = args.get(1).map(|s| s.to_lowercase());

    match command.as_deref() {
         Some("list") => list_todo_list(&mut todo_list),
         Some("add") => add_to_todo_list(&mut todo_list, &args),
         Some("remove") => remove_from_todo_list(&mut todo_list, &args),
         Some("check") => check_item(&mut todo_list, &args),
         Some("help") => print_help(),
         None => print_help(),
         Some(&_) => {
             println!("Unknown Command : {}", args[1]);
             print_help()
         },
    }

    save(&todo_list, &path).expect("failed to save todos");
}

fn list_todo_list(todo_list: &mut TodoList) {
    println!("{todo_list}")
}

fn add_to_todo_list(todo_list: &mut TodoList, args: &Vec<String>) {
    let task = args[2..].join(" ");
    if task.trim().is_empty() {
        println!("Usage: todo add <task>");
        return;
    }
    todo_list.add(task);

    println!("{todo_list}");

}

fn remove_from_todo_list(todo_list: &mut TodoList, args: &Vec<String>) {
    let id = args[2].parse::<usize>().unwrap() - 1;
    todo_list.remove(id);
    println!("{todo_list}");

}

fn check_item(todo_list: &mut TodoList, args: &Vec<String>) {
    let id = args[2].parse::<usize>().unwrap() - 1;
    let item = todo_list.find_mut(id);
    if item.unwrap().is_done {
        todo_list.mark_as_unfinished(id);
    } else {
        todo_list.mark_as_done(id);
    }

    println!("{todo_list}");

}

fn print_help() {
    println!("Todo — a simple terminal todo list");
    println!();
    println!("USAGE:");
    println!("  todo <command> [args]");
    println!();
    println!("COMMANDS:");
    println!("  add <task>       Add a new item to the list");
    println!("  remove <id>      Remove the item at position <id>");
    println!("  check <id>       Toggle done/undone for the item at position <id>");
    println!("  list             Show all items in the list");
    println!("  help             Show this message");
    println!();
}