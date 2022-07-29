/*
    Module for everything to do with a task
    2022.07.24      Sven Ponelat

*/


use crate::library::enums::*;
// use serde::{Serialize, Deserialize};






#[derive(Clone )]
pub struct Task {
    pub id: Option<i64>,
    pub hex_id: i64,
    pub description: String,
    pub status: Status,
    pub entry: i64,
    pub due: Option<i64>,
    pub end: Option<i64>,
    pub wait: Option<i64>,
    pub modified: Option<i64>,
    pub parent: Option<i64>,
    pub recur: Option<String>,
    pub rtype: Option<Rtype>,
    pub tags: Vec<String>,
    pub timetrackingseconds: Option<i64>,



}


// impl Task {
    
//     // make an empty task for compilers sake
//     pub fn new() -> Task {
//         Task { 
//             id: None,
//             hex_id: None,
//             description: "".to_string(),
//             status: Status::Waiting, 
//             id: None,
//             id: None,
//             id: None,
//             id: None,
//         }
//     }






// }










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

    
    // // #[ignore]
    // #[test]
    // fn t001_task_new() {
    //     let mut t1 = Task::new();
    //     t1.id = Some(23);
    //     t1.desc = "This is a description".to_string();
    //     t1.status = Status::Pending;

    //     assert_eq!(t1.id.unwrap(), 23);
    // }















} //end of tests
