pub mod db;
pub mod task;

use crate::db::db_start;


fn main() {
    //On Start
    db_start();
}
