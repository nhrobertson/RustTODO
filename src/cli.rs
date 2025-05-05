use std::io;
use colored::Colorize;
use crate::task::Task;

pub fn run_cli(tasks: Vec<Task>, completed: Vec<Task>) -> io::Result<()> {
    println!("
Welcome to Task App CLI!\n
Current Tasks:");
    show_tasks(&tasks);
    let mut run = true;
    while run {
        println!("\nInput a command. Type 'help' for a list of commands");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match &buffer.trim() as &str {
            "help" => show_help(),
            "tasks" => show_tasks(&tasks),
            "new" => create_new_task(&tasks),
            "completed" => show_completed(&completed),
            s if s.starts_with("complete ") => {
                let target = &s["complete ".len()..].trim();
                complete_task(target, &tasks, &completed);
            }
            s if s.starts_with("task ") => {
                let target = &s["task ".len()..].trim();
                display_task(target, &tasks);
            }
            "quit" => run = false,
            _ => println!("Unknown Command"),
        }
    }
    Ok(())
}

fn display_task(target: &str, tasks: &Vec<Task>) {
    println!("Task Target: {}", target);
}

fn show_help() {
    println!("help");
}

fn create_new_task(tasks: &Vec<Task>) {
    println!("create new task");
}

fn complete_task(task: &str, tasks: &Vec<Task>, completed: &Vec<Task>) {
    println!("completing task {}", task);
}

fn show_completed(completed: &Vec<Task>) {
    println!("completed");
}

fn show_tasks(tasks: &Vec<Task>) {
    if tasks.len() < 1 {
        println!("{}\n{} {} {}\n{}", 
        "##################################################".cyan(),
        "#".cyan(),
        "No Tasks Yet!                                 ".red(),
        "#".cyan(),
        "##################################################".cyan()
        );

    }
    for task in tasks {
        println!("{}\n{} [{}] {}, {}, {} {}\n{}", 
        "##################################################".cyan(),
        "#".cyan(),
        "X".red(),
        task.name.green(),
        task.creation_date.to_string().green(),
        task.description.green(),
        "#".cyan(),
        "##################################################".cyan()
        );

    }
}
