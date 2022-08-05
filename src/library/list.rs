/*
    Module for everything to do with a list consisting of tasks
    2022.07.24      Sven Ponelat

*/



use inflections::Inflect;
use crate::library::task::*;
use crate::library::enums::*;
use std::str::FromStr;
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

            if task.rtype.is_some() {                                                      //recur
                ret.push_str("\ttype:");
                ret.push_str(&task.rtype.clone().unwrap().text().to_lowercase());
            }
            
            if task.start.is_some() {                                                      //start
                ret.push_str("\tstart:");
                ret.push_str(&task.start.unwrap().to_string());
            }
            
            if task.parent.is_some() {                                                     //parent
                ret.push_str("\tparent:");
                ret.push_str(&task.parent.clone().unwrap());
            }
            
            if task.prodigy.is_some() {                                                    //parent
                ret.push_str("\tprodigy:");
                ret.push_str(&task.prodigy.unwrap().to_string());
            }
            
            if task.timetrackingseconds != 0 {                                             //parent
                ret.push_str("\ttimetrackingseconds:");
                ret.push_str(&task.timetrackingseconds.to_string());
            }
            
            if task.ann.len() != 0 {
                for a in task.ann.clone() {
                    ret.push_str("\tannotation_");
                    let num = a.date.to_string() + ":";
                    ret.push_str(&num);
                    ret.push_str(&a.desc);
                }
            }

            if task.tags.len() > 0 {                                                       //tags
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
    pending.list.clear();

    // does the file exists
    if ! Path::new(p_file.clone()).exists() {
        return Ok(())
    }


    let file = File::open(p_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut task = Task::new();

        if line.is_err() {
            return Err("Problems reading pending.data")            
        }
        let one_line = line.unwrap();
        let split_tab:Vec<_> = one_line.split("\t").collect();
        
        for element in split_tab {
            let split_colon:Vec<_> = element.split(":").collect();
            if split_colon.len() != 2 {
                return Err("Line in pending.data has faulty elements")            
            }
            match split_colon[0] {
                "description" => {
                    task.description = split_colon[1].to_string();
                }
                
                "uuiid" => {
                    task.uuiid = split_colon[1].to_string();
                }
                
                "entry" => {
                    let res= split_colon[1].parse::<i64>();
                    if res.is_err(){
                        return Err("Integer parsing error in pending.data")            
                    }
                    task.entry = res.unwrap();
                }

                "due" => {
                    let res= split_colon[1].parse::<i64>();
                    if res.is_err(){
                        return Err("Integer parsing error in pending.data")            
                    }
                    task.due = Some(res.unwrap());
                }

                "wait" => {
                    let res= split_colon[1].parse::<i64>();
                    if res.is_err(){
                        return Err("Integer parsing error in pending.data")            
                    }
                    task.wait = Some(res.unwrap());
                }
                
                "status" => {
                    let res = Status::from_str(split_colon[1]);
                    if res.is_err(){
                        return Err("Status parsing error in pending.data")            
                    }
                    task.status = res.unwrap();
                }
                
                "tags" => {
                    let split_comma:Vec<_> = split_colon[1].split(":").collect();
                    for tag in split_comma {
                        task.tags.push(tag.to_string());
                    }
                }
                
                "start" => {
                    let res= split_colon[1].parse::<i64>();
                    if res.is_err(){
                        return Err("Integer parsing error in pending.data")            
                    }
                    task.start = Some(res.unwrap());
                }


                _ => {
                    // shouldnt really get here
                    return Err("unknown element in colon split")            
                }
            }

        }
        
    } //end of for line loop



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

        let mut the_list = List::new();
        let text_file = "./test/pending.data";
        let vs: Vec<String> = vec!["Nutting".to_string(), "add".to_string(), "Do a job".to_string(),
                                 "due:2030-01-05".to_string(), "wait:2030-01-01".to_string(), "+household".to_string(),
                                 "+car".to_string()];
        let result = make_task(&vs, 1, 1);
        the_list.list.push(result.unwrap());
        
        let res = the_list.save(text_file);
        assert_eq!(res.unwrap(), 1);
        
        // lets do another one
        let vs: Vec<String> = vec!["Nutting".to_string(), "add".to_string(), "Do a jobby".to_string(),
                                    "due:2030-01-05".to_string(), "wait:2030-01-01".to_string(), "recur:+4m".to_string()];
        let result2 = make_task(&vs, 2, 2);
        the_list.list.push(result2.unwrap());

        let res = the_list.save(text_file);
        the_list.list.clear();
        let res_p = load_pending(text_file, &mut the_list);
        
        assert_eq!(the_list.list.len(), 2);
    }


    // #[ignore]
    #[test]
    fn t002_split1() {
        let str = "description:Do a job;uuiid:0x00001a;entry:1659664228;status:pending;due:1893801600;wait:1893456000;tags:household,car";
        let split:Vec<&str> = str.split(";").collect();
        
        
        
        
        
        assert_eq!(1, 1);

    }













} //end of tests








