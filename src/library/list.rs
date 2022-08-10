/*
    Module for everything to do with a list consisting of tasks
    2022.07.24      Sven Ponelat

*/


use inflections::Inflect;
use crate::library::task::*;
use crate::library::my_utils::*;
use crate::library::enums::*;
use crate::library::functions::*;
use std::str::FromStr;
use std::path::Path;
use std::fs::{ OpenOptions };
use std::io::{BufRead, BufReader};
use std::fs::*;
use std::io::prelude::*;
use std::collections::{BTreeSet, BTreeMap};
use std::process::exit;






#[derive(Clone )]
pub struct List<'a> {
    pub list: Vec<Task>,
    file: &'a str,
}

impl<'a> List<'a> {
    
    // make an empty task for compilers sake
    pub fn new(file: &str) -> List {
        List { 
            list: Vec::new(),
            file: file,
        }
    }


    // return the id of the new task
    pub fn save(&self) -> Result<i64, String> {
        // let path = Path::new(data_file);
        let path = Path::new(self.file);
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

        Ok(self.list.len() as i64)
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
                ret.push_str("\trtype:");
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

// no result needed as files could be messed up
pub fn load_all_tasks(  p_file: &str, c_file: &str, pending: &mut List, 
                        completed: &mut List, hexi_set: &mut BTreeSet<i64>) {
    
    let res_pend = load_task_file(p_file, pending, hexi_set);
    if res_pend.is_err(){
        let message = res_pend.err().unwrap();
        feedback(Feedback::Error, message);
        exit(17);
    }
    
    let res_comp = load_task_file(c_file, completed, hexi_set);
    if res_comp.is_err(){
        let message = res_comp.err().unwrap();
        feedback(Feedback::Error, message);
        exit(17);
    }
}

pub fn load_task_file(task_file: &str, the_list: &mut List, hexi_set: &mut BTreeSet<i64>) -> Result<(), String> {
    // does the file exists, if not return empties
    if ! Path::new(task_file).exists() {
        return Ok(());
    }

    let mut line_counter = 0;
    let file = File::open(task_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        line_counter += 1;
        let mut task;

        if line.is_err() {
            let message = format!("Problems reading file: {} on line number {}",task_file, line_counter);
            return Err(message);            
        }
        let one_line = line.unwrap();
        let split_tab:Vec<_> = one_line.split("\t").collect();

        let res_task = make_task(split_tab);
        if res_task.is_err() {
            let message = format!("Task error: {} on line number {}",
                            res_task.err().unwrap(), line_counter);
            return Err(message);
        }
        task = res_task.unwrap();
        task.id = Some(line_counter);

        hexi_set.insert(task.uuiid_int);
        // task_map.insert(task.id.unwrap(), task);
        // task_map.push(task)
        the_list.list.push(task);
    }

    Ok(())
}



// pub fn load_pending(p_file: &str, pending: &mut List)  -> Result<(), &'static str> {
//     pending.list.clear();

//     // does the file exists
//     if ! Path::new(p_file.clone()).exists() {
//         return Ok(())
//     }


//     let file = File::open(p_file).unwrap();
//     let reader = BufReader::new(file);

//     for line in reader.lines() {
//         let mut task = Task::new();

//         if line.is_err() {
//             return Err("Problems reading pending.data")            
//         }
//         let one_line = line.unwrap();
//         let split_tab:Vec<_> = one_line.split("\t").collect();
        
//         for element in split_tab {
//             let split_colon:Vec<_> = element.split(":").collect();
//             if split_colon.len() != 2 {
//                 return Err("Line in pending.data has faulty elements")            
//             }
//             match split_colon[0] {
//                 "description" => {
//                     task.description = split_colon[1].to_string();
//                 }
                
//                 "uuiid" => {
//                     task.uuiid = split_colon[1].to_string();
//                 }
                
//                 "entry" => {
//                     let res= split_colon[1].parse::<i64>();
//                     if res.is_err(){
//                         return Err("Integer parsing error in pending.data")            
//                     }
//                     task.entry = res.unwrap();
//                 }

//                 "due" => {
//                     let res= split_colon[1].parse::<i64>();
//                     if res.is_err(){
//                         return Err("Integer parsing error in pending.data")            
//                     }
//                     task.due = Some(res.unwrap());
//                 }

//                 "wait" => {
//                     let res= split_colon[1].parse::<i64>();
//                     if res.is_err(){
//                         return Err("Integer parsing error in pending.data")            
//                     }
//                     task.wait = Some(res.unwrap());
//                 }
                
//                 "status" => {
//                     let res = Status::from_str(split_colon[1]);
//                     if res.is_err(){
//                         return Err("Status parsing error in pending.data")            
//                     }
//                     task.status = res.unwrap();
//                 }
                
//                 "tags" => {
//                     let split_comma:Vec<_> = split_colon[1].split(":").collect();
//                     for tag in split_comma {
//                         task.tags.push(tag.to_string());
//                     }
//                 }
                
//                 "start" => {
//                     let res= split_colon[1].parse::<i64>();
//                     if res.is_err(){
//                         return Err("Integer parsing error in pending.data")            
//                     }
//                     task.start = Some(res.unwrap());
//                 }


//                 _ => {
//                     // shouldnt really get here
//                     return Err("unknown element in colon split")            
//                 }
//             }

//         }
        
//     } //end of for line loop



//     // let res_file = File::open(p_file);
//     // if res_file.is_err() {
//     //     return Err("Problem opening settings.txt");
//     // }
//     // let reader = BufReader::new(res_file.unwrap());
    
//     // // for each line
//     // for line in reader.lines() {
//     //     if line.is_err(){
//     //         return Err("Problem reading line in settings");
//     //     }
//     //     let read = Some(line.unwrap());
//     //     if read.clone().is_some() {
//     //         let split_tab = read.clone().unwrap();
//     //         let col = split_tab.clone().split("\t");
//     //         let aaa:Vec<_> =  col.clone().collect();
//     //         let len = aaa.clone().len();
//     //         println!("{}",aaa[0]);


//     //         let rr=8;
//     //     }
//     // }

// return Err("y")

// }


pub fn load_task_list(path: &str, list: &mut List, h_set: &mut BTreeSet<i64>) -> Result<(), String> {
    // clear the list
    list.list.clear();
    // let mut ret_list = List::new();
    // let mut ret_hexi64: BTreeSet<i64> = BTreeSet::new();

    // does the file exists, if not return empties
    if ! Path::new(path).exists() {
        // let ret = ( ret_list, ret_hexi64 );
        return Ok(());
    }


    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut task = Task::new();

        if line.is_err() {
            let message = format!("Problems reading file: {}",path);
            return Err(message);            
        }
        let one_line = line.unwrap();
        let split_tab:Vec<_> = one_line.split("\t").collect();
        
        for element in split_tab {
            let split_colon:Vec<_> = element.split(":").collect();
            if split_colon.len() != 2 {
                let message = format!("Line in file: {} has faulty elements",path);
                return Err(message);           
            }
            
            // to take care of annotation with time, i'm going to make a separate match term
            let mut matcho = split_colon[0];
            if matcho.starts_with("annotation") {
                matcho = "annotation";
            }
            
            match matcho {
                "annotation" => {
                    let split_ann:Vec<_> = split_colon[0].split("_").collect();
                    if split_ann.len() != 2 {
                        let message = format!("Line in file: {} has faulty annotations",path);
                        return Err(message);           
                    }
                    let mut anno = Annotation::new();
                    let date = split_ann[1].parse::<i64>();
                    if date.is_err(){
                        let message = format!("Line in file: {} has faulty annotations times",path);
                        return Err(message);  
                    }
                    anno.date = date.unwrap();
                    anno.desc = split_colon[1].to_string();
                    task.ann.push(anno);

                }

                "description" => {
                    task.description = split_colon[1].to_string();
                }
                
                "due" => {
                    let res= split_colon[1].parse::<i64>();
                    if res.is_err(){
                        let message = format!("Integer parsing error in file: {}",path);
                        return Err(message);           
                    }
                    task.due = Some(res.unwrap());
                }
                
                "end" => {
                    let res= split_colon[1].parse::<i64>();
                    if res.is_err(){
                        let message = format!("Integer parsing error in file: {}",path);
                        return Err(message);           
                    }
                    task.end = Some(res.unwrap());
                }

                "entry" => {
                    let res= split_colon[1].parse::<i64>();
                    if res.is_err(){
                        let message = format!("Integer parsing error in file: {}",path);
                        return Err(message);           
                    }
                    task.entry = res.unwrap();
                }
                
                "parent" => {
                    let parent = split_colon[1].to_string();
                    let res = hexi_verify(&parent);
                    if res.is_err(){
                        let message = format!("Line in file: {} has faulty hex values",path);
                        return Err(message);           
                    }
                    task.parent = Some(parent);
                    task.parent_int = Some(res.unwrap());
                }
                
                "prodigy" => {
                    let res= split_colon[1].parse::<i64>();
                    if res.is_err(){
                        let message = format!("Integer parsing error in file: {}",path);
                        return Err(message);           
                    }
                    task.prodigy = Some(res.unwrap());
                }
                
                "recur" => {
                    task.recur = Some(split_colon[1].to_string());
                }
                
                "rtype" => {
                    let res = Rtype::from_str(split_colon[1]);
                    if res.is_err(){
                        let message = format!("Rtype parsing error in file: {}",path);
                        return Err(message);         
                    }
                    task.rtype = Some(res.unwrap());
                }
                
                "start" => {
                    let res= split_colon[1].parse::<i64>();
                    if res.is_err(){
                        let message = format!("Status parsing error in file: {}",path);
                        return Err(message);             
                    }
                    task.start = Some(res.unwrap());
                }
                
                "status" => {
                    let res = Status::from_str(split_colon[1]);
                    if res.is_err(){
                        let message = format!("Status parsing error in file: {}",path);
                        return Err(message);         
                    }
                    task.status = res.unwrap();
                }
                
                "tags" => {
                    let split_comma:Vec<_> = split_colon[1].split(":").collect();
                    for tag in split_comma {
                        task.tags.push(tag.to_string());
                    }
                }
                
                "timetrackingseconds" => {
                    let res= split_colon[1].parse::<i64>();
                    if res.is_err(){
                        let message = format!("timetrackingseconds parsing error in file: {}",path);
                        return Err(message);             
                    }
                    task.timetrackingseconds = res.unwrap();
                }

                "uuiid" => {
                    let uuiid = split_colon[1].to_string();
                    let res = hexi_verify(&uuiid);
                    if res.is_err(){
                        let message = format!("Line in file: {} has faulty hex values",path);
                        return Err(message);           
                    }
                    task.uuiid = uuiid;
                    let u_int = res.unwrap();
                    task.uuiid_int = u_int;
                    h_set.insert(u_int);
                }

                "wait" => {
                    let res= split_colon[1].parse::<i64>();
                    if res.is_err(){
                        let message = format!("Integer parsing error in file: {}",path);
                        return Err(message);         
                    }
                    task.wait = Some(res.unwrap());
                }
                

                _ => {
                    // shouldnt really get here
                    return Err("Unknown element in colon split".to_string())            
                }
            }

        }
        
         if task.ann.len() > 1 {
            task.ann.sort();    
         }
        list.list.push(task);
    } //end of for line loop


    // let ret = ( ret_list, ret_hexi64 );
    Ok(())
}




// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use crate::library::{enums::Status, lts::lts_to_date_time_string};

    use super::*;
    use std::{fs::copy};
    use substring::Substring;
    use std::fs::remove_file;

    
    // #[ignore]
    #[test]
    fn t001_load_task_file() {

        let mut h_set:BTreeSet<i64> = BTreeSet::new();
        
        let source = "/DATA/programming/Rust/mytodo/test/some-documents/pending1.data";
        let destination = "./test/pending.data";
        copy(source,destination).expect("Failed to copy");
        let mut pending: List = List::new(destination);
        let _res = load_task_file(destination, &mut pending, &mut h_set);
        remove_file(destination).expect("Cleanup test failed");

        assert_eq!(pending.list.len(), 3);
        // let third_one = pending[3];
        let third_one = pending.list.get(2).unwrap();
        assert_eq!(third_one.uuiid, "0x0003");
        assert_eq!(third_one.id.unwrap(), 3);
        
        let second = pending.list.get(1).unwrap();
        let ann = second.clone();
        assert_eq!(ann.ann[0].desc, "remember janes payroll");
    }


    // #[ignore]
    #[test]
    fn t002_load_task_file() {

        let mut h_set:BTreeSet<i64> = BTreeSet::new();
        
        let source = "/DATA/programming/Rust/mytodo/test/some-documents/completedx1.data";
        let destination = "./test/completed.data";
        copy(source,destination).expect("Failed to copy");
        let mut completed: List = List::new(destination);
        let res = load_task_file(destination, &mut completed, &mut h_set);
        remove_file(destination).expect("Cleanup test failed");
        
        assert_eq!(res.is_err(), true);
        let message = res.err().unwrap();
        feedback(Feedback::Warning, message);
        completed.list.clear();
        
        
        let source = "/DATA/programming/Rust/mytodo/test/some-documents/completed1.data";
        let destination = "./test/completed.data";
        copy(source,destination).expect("Failed to copy");
        let _res2 = load_task_file(destination, &mut completed, &mut h_set);
        remove_file(destination).expect("Cleanup test failed");        
        assert_eq!(completed.list.len(), 2);
        
        // let one = completed.clone()[1];
        let one = completed.list.get(0).unwrap();
        let date_str = lts_to_date_time_string(one.end.unwrap());
        let real = "2022-08-03 20:56:50".to_string();
        assert_eq!(date_str, real);
        
        let one1 = 1622160097 as i64;
        let date_str2 = lts_to_date_time_string(one1);
        let ann = lts_to_date_time_string(one.ann[0].date);
        assert_eq!(date_str2, ann);
        
        let h = get_next_hexidecimal(h_set);
        assert_eq!(h, 1);
    }

    // #[ignore]
    #[test]
    fn t003_new() {

        // let mut completed: List = List::new();
        // let mut h_set:BTreeSet<i64> = BTreeSet::new();

        let destination = "./test/trial.data";
        let mut pen = List::new(destination);

        //from line in file
        let line = "description:how do i get the konsole that i have now\tdue:1658513756\t\
                            entry:1658513756\tstart:1658513756\tstatus:pending\tuuiid:0x0011";
        let line2 = "description:how do i do\tdue:1658513756\t\
                            entry:1658512756\tstart:1658513756\tstatus:pending\tuuiid:0x0001";
        let vec:Vec<_> = line.split("\t").collect();
        let vec2:Vec<_> = line2.split("\t").collect();
        let task = make_task(vec);
        let task2 = make_task(vec2);
        pen.list.push(task.unwrap());
        pen.list.push(task2.unwrap());
        let res = pen.save();

        remove_file(destination).expect("Cleanup test failed");

        assert_eq!(res.unwrap(), 2);
    }










} //end of tests








