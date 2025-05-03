pub mod db;
pub mod task;
pub mod gui;
pub mod clihandler;

use crate::db::db_start;
use crate::gui::start_gui;
use crate::clihandler::handle_args;

fn main() {
    //Load Database into memory
    let (tasks, completed) = db_start(); 
    let condition = handle_args();
    match &condition as &str {
        "cli" => run_cli(),
        "gui" => run_gui(),
        _ => panic!("[Error]: main.rs, main(): Panic! condition not set"),
    }
}

fn run_cli(tasks: Vec<Task>, completed: Vec<Task>) {
    println!("Running CLI");
}

fn run_gui(tasks: Vec<Task>, completed: Vec<Task>) {
    println!("Running GUI");
}


