pub mod db;
pub mod task;
pub mod gui;

use crate::db::db_start;
use crate::gui::start_gui;

fn main() {
    //Load Database into memory
    let (tasks, completed) = db_start();
    //Start GUI
    let _ = start_gui();
}
