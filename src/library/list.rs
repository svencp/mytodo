/*
    Module for everything to do with a list consisting of tasks
    2022.07.24      Sven Ponelat

*/



use crate::library::task::*;
use std::path::Path;
use std::fs::{ OpenOptions };
// use serde::{Serialize, Deserialize};
// use std::fs::File;
use std::io::prelude::*;


#[derive(Clone )]
pub struct List {
    list: Vec<Task>,
}

impl List {
    
    // make an empty task for compilers sake
    pub fn new() -> List {
        List { 
            list: Vec::new(),
        }
    }


    pub fn save(&self, data_file: &str) -> Result<(), String> {
        // let path = Path::new(data_file);
        
        // let serialized = serde_json::to_string(&self.list);
        // let mut file = match OpenOptions::new()
        //                         .read(false)
        //                         .write(true)
        //                         .create(true)
        //                         .truncate(true)
        //                         .open(path)  {
            
        //     Err(_) => { return Err("Problem exporting species json file".to_string()); }
        //     Ok(file)   => { file }
        // };
        
        // match file.write_all(serialized.unwrap().as_bytes()) {
        //     Err(_) => { return Err("Problem writing species json file".to_string()); } 
        //     Ok(_)   => { Ok(()) } 
        // }
    Ok(())
    }




}


// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@







// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use crate::library::enums::Status;

    use super::*;
    use std::{fs::copy};
    use substring::Substring;
    use std::fs::remove_file;

    
    // #[ignore]
    #[test]
    fn t001_list_new() {

        let json_file = "./test/pending.data";

        let mut task = Task::new();
        task.description = "Hello Svenny".to_string();
        task.id = Some(1);
        task.status = Status::Waiting;


        let mut l = List::new();
        l.list.push(task);
        assert_eq!(l.list.len(), 1);
        
        let res = l.save(json_file);
        remove_file(json_file).expect("Cleanup test failed");
        assert_eq!(res.is_ok(), true);



    }















} //end of tests








