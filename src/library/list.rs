/*
    Module for everything to do with a list consisting of tasks
    2022.07.24      Sven Ponelat

*/



use inflections::Inflect;
use crate::library::task::*;
use std::path::Path;
use std::fs::{ OpenOptions };
use std::io::{BufRead, BufReader};
use std::fs::*;
use std::io::prelude::*;


#[derive(Clone )]
pub struct List {
    pub list: Vec<Task>,
}

impl List {
    
    // make an empty task for compilers sake
    pub fn new() -> List {
        List { 
            list: Vec::new(),
        }
    }


    // return the id of the new task
    pub fn save(&self, data_file: &str) -> Result<i32, String> {
        let path = Path::new(data_file);
        let big_str = self.make_big_string();

        // let serialized = serde_json::to_string(&self.list);
        let mut file = match OpenOptions::new()
                                .read(false)
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open(path)  {
            
            Err(_) => { return Err("Problem exporting species json file".to_string()); }
            Ok(file)   => { file }
        };
        
        match file.write_all(big_str.as_bytes()) {
            Err(_)      => { return Err("Problem writing pending data file".to_string()); } 
            Ok(file) => { file }
        }


        Ok(self.list.len() as i32)
    }

    // make a big string to save to a text file
    pub fn make_big_string(&self) -> String {
        let mut ret:String =  "".to_string();
    
        for task in &self.list {
            ret.push_str("description:");
            ret.push_str(&task.description);
            ret.push_str("\t");
            ret.push_str("uuiid:");
            ret.push_str(&task.uuiid);
            ret.push_str("\t");
            ret.push_str("entry:");
            ret.push_str(&task.entry.to_string());
            ret.push_str("\t");
            ret.push_str("status:");
            ret.push_str(&task.status.text().to_lower_case());

            if task.due.is_some() {                                                        //due
                ret.push_str("\tdue:");
                ret.push_str(&task.due.unwrap().to_string());
            }
            if task.wait.is_some() {                                                       //wait
                ret.push_str("\twait:");
                ret.push_str(&task.wait.unwrap().to_string());
            }
            
            if task.recur.is_some() {                                                      //recur
                ret.push_str("\trecur:");
                ret.push_str(&task.recur.clone().unwrap());
            }
            
            if task.start.is_some() {                                                      //start
                ret.push_str("\tstart:");
                ret.push_str(&task.start.unwrap().to_string());
            }
            
            if task.parent.is_some() {                                                     //parent
                ret.push_str("\tparent:");
                ret.push_str(&task.parent.clone().unwrap());
            }
            
            if task.tags.len() > 0 {                                                        //tags
                let mut vec:String = "".to_string();
                for tag in task.tags.clone() {
                    let str = tag + ",";
                    vec.push_str(&str);
                }
                
                //remove last comma
                let len = vec.len();
                let end = len -1 ;
                let taggings = vec[0..end].to_string();
                
                ret.push_str("\ttags:");                                                  
                ret.push_str(&taggings);

            }

            ret.push_str("\n")

        }

        return ret;

    } // end of make_big_string 










} // end of impl















// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
pub fn load_all_tasks(p_file: &str, c_file: &str, pending: &mut List, completed: &mut List)   {

    let res_pend = load_pending(p_file,pending);
    

}



pub fn load_pending(p_file: &str, pending: &mut List)  -> Result<(), &'static str> {
    
    // does the file exists
    if ! Path::new(p_file.clone()).exists() {
        return Ok(())
    }


    let file = File::open(p_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.is_err() {
            return Err("Problems reading pending.data")            
        }
        let l = line.unwrap();
        let c:Vec<_> = l.split("\t").collect();
        let r = 1 + 3;
        
    }



    // let res_file = File::open(p_file);
    // if res_file.is_err() {
    //     return Err("Problem opening settings.txt");
    // }
    // let reader = BufReader::new(res_file.unwrap());
    
    // // for each line
    // for line in reader.lines() {
    //     if line.is_err(){
    //         return Err("Problem reading line in settings");
    //     }
    //     let read = Some(line.unwrap());
    //     if read.clone().is_some() {
    //         let split_tab = read.clone().unwrap();
    //         let col = split_tab.clone().split("\t");
    //         let aaa:Vec<_> =  col.clone().collect();
    //         let len = aaa.clone().len();
    //         println!("{}",aaa[0]);


    //         let rr=8;
    //     }
    // }

return Err("y")

}







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

        let text_file = "./test/pending.data";
        let vs: Vec<String> = vec!["Nutting".to_string(), "add".to_string(), "Do a job".to_string(),
                                 "due:2030-01-05".to_string(), "wait:2030-01-01".to_string(), "+household".to_string(),
                                 "+car".to_string()];
        let result = make_task(&vs, 26, 30);
        let mut ll = List::new();
        ll.list.push(result.unwrap());

        let res = ll.save(text_file);
        assert_eq!(res.unwrap(), 1);
        
        let mut pending = List::new();
        let res_p = load_pending(text_file, &mut pending);
        
        
        
        
        
        assert_eq!(1, 1);

    }

    // #[ignore]
    #[test]
    fn t002_split1() {

        let str = "description:Do a job;uuiid:0x00001a;entry:1659664228;status:pending;due:1893801600;wait:1893456000;tags:household,car";
        let split:Vec<&str> = str.split(";").collect();
        
        
        
        
        
        assert_eq!(1, 1);

    }













} //end of tests








