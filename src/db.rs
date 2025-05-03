use rusqlite::{Connection, Result, Statement};
use crate::task::{Task, TaskType};
use utc_dt::{UTCDatetime};
use utc_dt::time::{UTCTransformations, UTCTimestamp};

pub fn db_start() -> (Vec<Task>, Vec<Task>) {
    let conn = handle_db_load();
    let (tasks, completed) = extract_db(&conn);

    let _ = conn.close();
    (tasks, completed)
}

fn handle_db_load() -> Connection {
    let conn = Connection::open("tasks.db").unwrap();

    conn.execute(
    "create table if not exists tasks (
            name text not null,
            creation_date integer not null,
            target_date integer,
            description text,
            task_type text,
            completed bool not null,
            id integer not null unique
        )",
    [],
    ).expect("[Error] db.rs, handle_db_load(): Unable to find or create SQL table 'tasks'");

    conn.execute(
    "create table if not exists completed (
            name text not null,
            creation_date integer not null,
            target_date integer,
            description text,
            task_type text,
            completed bool not null,
            id integer not null unique
        )",
    [],
    ).expect("[Error] db.rs, handle_db_load(): Unable to find or create SQL table 'completed'"); 

    conn
}

fn extract_db(conn: &Connection) -> (Vec<Task>, Vec<Task>) {
    
    //Prepare the connection to the "tasks" table
    let mut stmt = conn.prepare("SELECT name, creation_date, target_date, description, task_type, completed, id FROM tasks").unwrap();

    let tasks = convert_table_to_tasks(&mut stmt);
    
    //Prepare the connection to the "completed" table
    stmt = conn.prepare("SELECT name, creation_date, target_date, description, task_type, completed, id FROM completed").unwrap();

    let completed = convert_table_to_tasks(&mut stmt);
    
    (tasks, completed)
}

fn convert_table_to_tasks(stmt: &mut Statement) -> Vec<Task> {
    let mut vec = Vec::<Task>::new();
    for row_result in stmt.query_map([], |row| {
            let raw_cd = row.get::<_, u64>(1).expect("[Error] db.rs, convert_table_to_tasks(stmt: &Statement) -> Vec<Task>: creation_date"); 
            let cd_timestamp = UTCTimestamp::from_secs(raw_cd);
            let _creation_date: UTCDatetime = UTCDatetime::from_timestamp(cd_timestamp);
            let raw_td = row.get::<_, u64>(2).expect("[Error] db.rs, convert_table_to_tasks(stmt: &Statement) -> Vec<Task>: target_date");
            let td_timestamp = UTCTimestamp::from_secs(raw_td);
            let _target_date: UTCDatetime = UTCDatetime::from_timestamp(td_timestamp);
            let _task_type = get_task_type(&row.get::<_, String>(4).expect("[Error] db.rs, convert_table_to_tasks(stmt: &Statement) -> Vec<Task>: task_type"));
            Ok(Task {
                name: row.get(0).expect("[Error] db.rs, convert_table_to_tasks(stmt: &Statement) -> Vec<Task>: name"),
                creation_date: _creation_date,
                target_date: Some(_target_date),
                description: row.get(3).expect("[Error] db.rs, convert_table_to_tasks(stmt: &Statement) -> Vec<Task>: description"),
                task_type: _task_type,
                completed: row.get(5).expect("[Error] db.rs, convert_table_to_tasks(stmt: &Statement) -> Vec<Task>: completed"),
                id: row.get(6).expect("[Error] db.rs, convert_table_to_tasks(stmt: &Statement) -> Vec<Task>: id"),
            })
        }).expect("[Error] db.rs, convert_table_to_tasks(stmt: &Statement): Error in stmt.query_map") {
        vec.push(row_result.unwrap());
    }

    vec
}

fn get_task_type(task_type_str: &str) -> Vec<TaskType> {
    let mut vec = Vec::<TaskType>::new();
    if task_type_str.contains(",") {
        let task_types = task_type_str.split(",");
        for task_type in task_types {
            match task_type {
                "Personal" => { vec.push(TaskType::Personal); },
                "Work" => { vec.push(TaskType::Work); },
                "Hobby" => { vec.push(TaskType::Hobby); },
                "Programming" => { vec.push(TaskType::Programming); },
                "Exercise" => { vec.push(TaskType::Exercise); },
                _ => { vec.push(TaskType::Other); },
            }
        }
    } else {    
        match task_type_str {
            "Personal" => { vec.push(TaskType::Personal); },
            "Work" => { vec.push(TaskType::Work); },
            "Hobby" => { vec.push(TaskType::Hobby); },
            "Programming" => { vec.push(TaskType::Programming); },
            "Exercise" => { vec.push(TaskType::Exercise); },
            _ => { vec.push(TaskType::Other); },
        }
    }

    vec
}
