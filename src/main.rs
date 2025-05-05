pub mod db;
pub mod task;
pub mod gui;
pub mod cli;
pub mod argshandler;

use crate::db::db_start;
use crate::gui::run_gui;
use crate::cli::run_cli;
use crate::argshandler::handle_args;
use crate::task::Task;

fn main() {
    //Load Database into memory
    let (tasks, completed) = db_start(); 
    let condition = handle_args();
    match &condition as &str {
        "cli" => option_cli(tasks, completed),
        "gui" => option_gui(tasks, completed),
        _ => panic!("[Error]: main.rs, main(): Panic! condition not set"),
    }
}

fn option_cli(tasks: Vec<Task>, completed: Vec<Task>) {
    let _ = run_cli(tasks, completed);
}

fn option_gui(tasks: Vec<Task>, completed: Vec<Task>) {
    println!("Running GUI");
}


