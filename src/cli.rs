use std::io;
use colored::Colorize;
use crate::task::{Task, TaskType};
use utc_dt::{UTCDatetime};
use utc_dt::date::{UTCDate};
use utc_dt::time::{UTCTransformations, UTCTimeOfDay};  

pub fn run_cli(tasks: &mut Vec<Task>, completed: &mut Vec<Task>) -> io::Result<()> {
    println!("
Welcome to Task App CLI!\n
Current Tasks:");
    show_tasks(&tasks);
    let mut run = true;
    while run {
        println!("\nInput a command. Type 'help' for a list of commands");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match buffer.trim() as &str {
            "help" => show_help(),
            "tasks" => show_tasks(&tasks),
            "new" => create_new_task(tasks)?,
            "completed" => show_completed(completed),
            s if s.starts_with("complete ") => {
                let target = &s["complete ".len()..].trim();
                complete_task(target, tasks, completed);
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
    for task in tasks {
        if task.name == target {
            println!("{}\n{} {}, {}, {} {}\n{}", 
                "##################################################".cyan(),
                "#".cyan(),
                task.name.green(),
                task.creation_date.to_string().green(),
                task.description.green(),
                "#".cyan(),
                "##################################################".cyan()
            );
            break;
        }
    }
}

fn show_help() {
    println!("
RustTODO:
Below are a list of commands and how to use them for this simple Rust TODO App in the CLI version.

'help'              -> shows this help menu
'tasks'             -> shows all unfinished tasks
'task <task>'       -> show specific task
'new'               -> creates a new task, you will be prompted with the creation pipeline.
'completed'         -> shows all completed tasks
'complete <task>'   -> completes the specified task
'quit'              -> closes out of the application
        ");
}

fn create_new_task(tasks: &mut Vec<Task>) -> io::Result<()> {
    let mut buffer = String::new();
    println!("Create a new task.\n Enter a name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut buffer)?;
    name = buffer.trim().to_string();
    buffer.clear();
    let creation_time = UTCDatetime::try_from_system_time().unwrap();
    
    println!("Would you like to specify a target date and time for completion? Enter: Y/n");
    io::stdin().read_line(&mut buffer)?;
    let mut have_td: bool;
    match buffer.trim() as &str {
        "Y" => have_td = true,
        "y" => have_td = true,
        "N" => have_td = false,
        "n" => have_td = false,
        _ => { println!("Invalid Input, assuming NO"); have_td = false; }, 
    }
    buffer.clear();
    let mut date: Option<UTCDate> = None;
    let mut tod: Option<UTCTimeOfDay> = None;
    if have_td {    
        println!("Enter a target date in format: YYYY-MM-DD");
        io::stdin().read_line(&mut buffer)?;
        date = Some(UTCDate::try_from_iso_date(&buffer.trim()).unwrap());
        println!("Enter a target time in format: hh-mm-ss");
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        let time_str = format!("T{}Z", buffer.trim());
        tod = Some(UTCTimeOfDay::try_from_iso_tod(&time_str).unwrap());
    }
    
    let target_date: Option<UTCDatetime> = match have_td {
        true => Some(UTCDatetime::from_components(date.unwrap(), tod.unwrap())),
        false => None,
    };
    println!("Write a description.");
    buffer.clear();
    let mut description = String::new();
    io::stdin().read_line(&mut buffer)?;
    description = buffer.trim().to_string();

    buffer.clear();
    println!("Please assign this task it type(s)\nTo enter its type input the corresponding number to its type, you can specify multiple by seperating the numbers with ','\n");
    println!("List of types:\n\t1: Personal\n\t2: Work\n\t3: Hobby\n\t4: Programming\n\t5: Exercise\n\t6: Other");
    io::stdin().read_line(&mut buffer)?;
    let number_types = buffer.trim().split(",");
    let mut task_type: Vec<TaskType> = Vec::new();
    for num in number_types {
        match num {
            "1" => task_type.push(TaskType::Personal),
            "2" => task_type.push(TaskType::Work),
            "3" => task_type.push(TaskType::Hobby),
            "4" => task_type.push(TaskType::Programming),
            "5" => task_type.push(TaskType::Exercise),
            "6" => task_type.push(TaskType::Other),
            _ => println!("Error: Invalid task type, skipping...")
        }
    }
    
    tasks.push(Task::new(name, creation_time, target_date, description, task_type));
    Ok(())
}

fn complete_task(task: &str, tasks: &mut Vec<Task>, completed: &mut Vec<Task>) {
    println!("completing task {}", task);
    for i in 0..tasks.len() {
        if tasks[i].name == task {
            tasks[i].complete();
            let t = tasks.remove(i);
            completed.push(t);
            break;
        }
    }
}

fn show_completed(completed: &Vec<Task>) {
    if completed.len() < 1 {
        println!("{}\n{} {} {}\n{}", 
        "##################################################".cyan(),
        "#".cyan(),
        "No Completed Tasks Yet!                           ".red(),
        "#".cyan(),
        "##################################################".cyan()
        );

    }
    for task in completed {
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
