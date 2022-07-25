/*
    Module for everything to do with a task
    2022.07.24      Sven Ponelat

*/


use crate::library::enums::*;
// use serde::{Serialize, Deserialize};






#[derive(Clone )]
pub struct Task {
    pub id: Option<i32>,
    pub desc: String,
    pub status: Status,
    // entered: Some(i64),



}


impl Task {
    
    // make an empty task for compilers sake
    pub fn new() -> Task {
        Task { 
            id: None,
            desc: "".to_string(),
            status: Status::Waiting, 
        }
    }






}










// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@







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
        t1.desc = "This is a description".to_string();
        t1.status = Status::Pending;

        assert_eq!(t1.id.unwrap(), 23);
    }















} //end of tests
