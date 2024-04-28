use std::{env, usize};
use testi::Task;

fn main() {
    let task = Task::new().expect("Couldn't create the task instance");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let query = &args[1];
        let inp = &args[2..];
        match query.as_str() {
            "add" | "a" => task.add_task(inp),
            "list" | "l" => task.list_task(),
            "mark" | "m" => task.mark_task(args[2].parse::<usize>().unwrap()),
            "remove" | "rm" => task.delete_task(args[2].parse::<usize>().unwrap()),
            "edit" | "e" => task.edit_task(
                args[2]
                    .parse::<usize>()
                    .expect("Failed to parse arg as <usize>"),
                &args[3..],
            ),
            "reset" => task.reset_task(),
            "help" | "h" | _ => help(),
        }
    } else {
        task.list_task();
    }
}

const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - a, add [TASK]
        adds new task
        Example: todo add First task
    - l, list
        lists all tasks
        Example: todo list
    - m, mark [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm, remove [INDEX]
        removes a task
        Example: todo rm 4
    - e, edit [INDEX] [TASK]
        edits a task
        Example: todo e  2 Updated task
    - reset
        deletes all tasks
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
";
pub fn help() {
    // For readability
    println!("{}", TODO_HELP);
}
