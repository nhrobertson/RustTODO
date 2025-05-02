use rusqlite::{Connection, Result};
use crate::task;

pub fn db_start() {
    let conn = handle_db_load();
}

fn handle_db_load() -> Connection {
    let conn = Connection::open("tasks.db").unwrap();

    conn.execute(
    "create table if not exists tasks (
            name text not null,
            creation_date text not null,
            target_date text,
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
            creation_date text not null,
            target_date text,
            description text,
            task_type text,
            completed bool not null,
            id integer not null unique
        )",
    [],
    ).expect("[Error] db.rs, handle_db_load(): Unable to find or create SQL table 'completed'"); 

    conn
}