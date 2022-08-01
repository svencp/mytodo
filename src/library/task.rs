/*
    Module for everything to do with a task
    2022.07.24      Sven Ponelat

*/


use std::time::SystemTime;

use crate::library::enums::*;
// use serde::{Serialize, Deserialize};






#[derive(Clone )]
pub struct Task {
    pub uuiid: i64,
    pub id: Option<i64>,
    pub description: String,
    pub entry: i64,
    pub due: Option<i64>,
    pub end: Option<i64>,
    pub wait: Option<i64>,
    pub modified: Option<i64>,
    pub parent: Option<i64>,
    pub recur: Option<String>,
    pub status: Status,
    pub rtype: Option<Rtype>,
    pub tags: Vec<String>,
    pub timetrackingseconds: i64,



}


impl Task {
    
    // make an empty task for compilers sake
    pub fn new() -> Task {
        Task { 
            id: None,
            uuiid: 0,
            description: "".to_string(),
            status: Status::Pending, 
            entry: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
            due: None,
            end: None,
            wait: None,
            modified: None,
            parent: None,
            recur: None,
            rtype: None,
            tags: Vec::new(),
            timetrackingseconds: 0,
        }
    }






}










// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
pub fn add_task(args: Vec<String>) -> Result<(usize), String> {

    Ok(1)
}






// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::copy};
    use substring::Substring;
    use std::fs::remove_file;

    
    // #[ignore]
    #[test]
    fn t001_task_new() {
        let mut t1 = Task::new();
        t1.id = Some(23);
        t1.description = "This is a description".to_string();
        t1.status = Status::Pending;
        

        let yebo: bool = t1.entry > 1650000000;
        assert_eq!(yebo, true);
    }















} //end of tests
