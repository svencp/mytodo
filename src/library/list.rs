/*
Module for everything to do with a list consisting of tasks
2022.07.24      Sven Ponelat

*/


use inflections::Inflect;
use crate::library::task::*;
use crate::library::my_utils::*;
use crate::library::enums::*;
use crate::library::structs::*;
use crate::library::lts::*;
use std::path::Path;
use std::fs::{ OpenOptions };
use std::io::{BufRead, BufReader};
use std::fs::*;
use std::io::prelude::*;
use std::process::exit;

use super::reports::to_orange_feedback;






#[derive(Clone )]
pub struct List<'a> {
    pub list: Vec<Task>,
    file: &'a str,
}

impl<'a> List<'a> {
    
    // append another list to this one
    pub fn append(&mut self, other: List){
        for task in other.list {
            self.list.push(task);
        }
    }

    pub fn get_index_of_task_with_id(&self, id: i64) -> i64 {
        let mut index = 0 as i64;
        for task in self.list.clone() {
            if task.id.is_some() {
                if task.id.unwrap() == id {
                    return index;
                }
            }
            index += 1;
        }
        return -1;
    }
    
    pub fn get_index_of_task_with_uuiid(&self, uuiid: &str) -> i64 {
        let mut index = 0 as i64;
        for task in self.list.clone() {
            if task.uuiid == uuiid {
                return index;
            }
            index += 1;
        }
        return -1;
    }

    pub fn get_index_of_task_with_uuiid_int(&self, uuiid_int: i64) -> i64 {
        let mut index = 0 as i64;
        for task in self.list.clone() {
            if task.uuiid_int == uuiid_int {
                return index;
            }
            index += 1;
        }
        return -1;
    }

    // get task from given id
    pub fn get_task_from_id(&self, id: i64) -> Result<Task, &'static str> {
        for task in self.list.clone() {
            if task.id.is_some() {
                if id == task.id.unwrap() {
                    return Ok(task)
                }
            }
        }
        Err("Task not found with given id")
    }
    
    pub fn get_task_from_uuiid(&self, uuiid: String) -> Result<Task, &'static str> {
        for task in self.list.clone() {
            if uuiid == task.uuiid {
                return Ok(task)
            }
        }
        Err("Task not found with given uuiid")
    }
    
    pub fn get_task_from_uuiid_int(&self, uuiid_int: i64) -> Result<Task, &'static str> {
        for task in self.list.clone() {
            if uuiid_int == task.uuiid_int {
                return Ok(task)
            }
        }
        Err("Task not found with given uuiid_int")
    }

    // make a big string to save to a text file
    pub fn make_big_string(&self) -> String {
        let mut ret:String =  "".to_string();
        
        for task in &self.list {

            let mut line = make_line_from_task(task);
            line += "\n";

            ret.push_str(&line);

            // ret.push_str("description:");
            // ret.push_str(&task.description);
            // ret.push_str("\t");
            // ret.push_str("uuiid:");
            // ret.push_str(&task.uuiid);
            // ret.push_str("\t");
            // ret.push_str("entry:");
            // ret.push_str(&task.entry.to_string());
            // ret.push_str("\t");
            // ret.push_str("status:");
            // ret.push_str(&task.status.text().to_lower_case());
    
            // if task.ann.len() != 0 {
            //     for a in task.ann.clone() {
            //         ret.push_str("\tannotation_");
            //         let num = a.date.to_string() + ":";
            //         ret.push_str(&num);
            //         ret.push_str(&a.desc);
            //     }
            // }
            // if task.due.is_some() {                                                        //due
            //     ret.push_str("\tdue:");
            //     ret.push_str(&task.due.unwrap().to_string());
            // }
            // if task.end.is_some() {                                                        //due
            //     ret.push_str("\tend:");
            //     ret.push_str(&task.end.unwrap().to_string());
            // }
            // if task.parent.is_some() {                                                     //parent
            //     ret.push_str("\tparent:");
            //     ret.push_str(&task.parent.clone().unwrap());
            // }
            // if task.prodigy.is_some() {                                                    //parent
            //     ret.push_str("\tprodigy:");
            //     ret.push_str(&task.prodigy.unwrap().to_string());
            // }
            // if task.recur.is_some() {                                                      //recur
            //     ret.push_str("\trecur:");
            //     ret.push_str(&task.recur.clone().unwrap());
            // }
            // if task.rtype.is_some() {                                                      //recur
            //     ret.push_str("\trtype:");
            //     ret.push_str(&task.rtype.clone().unwrap().text().to_lowercase());
            // }
            // if task.start.is_some() {                                                      //start
            //     ret.push_str("\tstart:");
            //     ret.push_str(&task.start.unwrap().to_string());
            // }
            // if task.tags.len() > 0 {                                                       //tags
            //     let mut vec:String = "".to_string();
            //     for tag in task.tags.clone() {
            //         let str = tag + ",";
            //         vec.push_str(&str);
            //     }
                
            //     //remove last comma
            //     let len = vec.len();
            //     let end = len -1 ;
            //     let taggings = vec[0..end].to_string();
                
            //     ret.push_str("\ttags:");                                                  
            //     ret.push_str(&taggings);
            // }
            // if task.timetrackingseconds != 0 {                                             //parent
            //     ret.push_str("\ttimetrackingseconds:");
            //     ret.push_str(&task.timetrackingseconds.to_string());
            // }
            // if task.wait.is_some() {                                                       //wait
            //     ret.push_str("\twait:");
            //     ret.push_str(&task.wait.unwrap().to_string());
            // }
    
            // ret.push_str("\n")
    
        }
    
        return ret;
    
    } // end of make_big_string 



    // make an empty task for compilers sake
    pub fn new(file: &str) -> List {
        List { 
            list: Vec::new(),
            file: file,
        }
    }
    
    // create a list with no reference to a file
    pub fn new_no_file() -> List<'a> {
        List { 
            list: Vec::new(),
            file: "",
        }
    }
    
    // return the id of the new task
    pub fn save(&self)  {
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
            
            Err(_) => { 
                let message = "Problems opening data files".to_string();
                feedback(Feedback::Error, message);
                exit(17);
            }

            Ok(file)   => { file }
        };
        
        match file.write_all(big_str.as_bytes()) {
            Err(_)      => { 
                let message = "Problem writing data file".to_string();
                feedback(Feedback::Error, message);
                exit(17);
            } 
            Ok(file) => { file }
        }

        // Ok(self.list.len() as i64)
    }
    
    // start the task
    pub fn start_task(&mut self, index: usize) {
        let mut task = self.list.remove(index);
        if task.end.is_some() {
            task.end = None;
        }
        task.status = Status::Pending;
        task.start = Some(lts_now());
        if task.timetrackingseconds < 0 {
            task.timetrackingseconds = 0;
        }

        println!("Starting task {} '{}'.",task.uuiid,task.description);

        self.list.insert(index, task.clone());
    }

    // remove from list given only the id
    pub fn remove_with_id(&mut self, id:i64) -> Result<Task,&'static str> {
        let index = self.get_index_of_task_with_id(id);
        if index < 0 {
            return Err("ID not found in list.")
        }
        let ret = self.list.remove(index as usize);

        Ok(ret)
    }






} // end of impl















// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

pub fn delete_task(pend: &mut List, comp: &mut List, task: Task ) -> Result<(),&'static str> {
    let index = pend.get_index_of_task_with_uuiid_int(task.uuiid_int);
    if index < 0 {
        return Err("Error deleting task in pending file.")
    }
    let mut deleted = pend.list.remove(index as usize);
    deleted.status = Status::Deleted;
    
    // lets move the start date (if it has one to the end)
    match deleted.start.is_some() {
        true => {
            deleted.end = Some(deleted.clone().start.unwrap()); 
        }
        false => {
            deleted.end = Some(lts_now());
        }
    }

    pend.save();
    comp.list.insert(0, deleted.clone());
    comp.save();

    println!("Deleting task {} '{}'.", deleted.clone().uuiid, deleted.description);

    Ok(())
}

pub fn delete_task_from_completed(comp: &mut List, task: Task ) -> Result<(),&'static str> {
    let index = comp.get_index_of_task_with_uuiid_int(task.uuiid_int);
    if index < 0 {
        return Err("Error deleting task in completed file.")
    }
    let mut deleted = comp.list.remove(index as usize);
    deleted.status = Status::Deleted;
    
    // lets move the start date (if it has one to the end)
    match deleted.start.is_some() {
        true => {
            deleted.end = Some(deleted.clone().start.unwrap()); 
        }
        false => {
            deleted.end = Some(lts_now());
        }
    }

    comp.list.insert(0, deleted.clone());
    comp.save();

    println!("Deleting task {} '{}'.", deleted.clone().uuiid, deleted.description);

    Ok(())
}

pub fn find_children(pend: &List, task: &Task) -> Option<Vec<Task>> {
    let mut ret: Vec<Task> = Vec::new();

    for t in pend.clone().list {
        if t.parent.is_some() {
            if t.clone().parent.unwrap() == task.uuiid {
                ret.push(t.clone());
            }
        }
    }

    if ret.len() == 0 {
        return None;
    }

    return Some(ret)
}

pub fn find_latest_child(comp: &List, parent: &Task) -> Option<Task> {
    let res_children = find_children(comp, parent);
    if res_children.is_none(){
        return None;
    }
    let mut children = res_children.unwrap();

    // sort by end date  i.e. highest at the start
    children.sort_by(|a,b| {
        return b.end.unwrap().cmp(&a.end.unwrap())
    });

    // return the one at the start
    return Some(children[0].clone())
}

pub fn generate_recurring_tasks(pend: &mut List, comp: &mut List, hd_set: &mut Hdeci ) {
    let mut recurring_tasks: Vec<Task> = Vec::new();
    let mut no_children: Vec<Task> = Vec::new();

    // find recurring tasks that are parents
    for task in pend.clone().list {
        if task.is_recurring() && task.is_parent() {
            recurring_tasks.push(task);
        }
    } 

    // do all have children
    for task in recurring_tasks {
        let children = find_children(&pend, &task);
        if children.is_none() {
            no_children.push(task);
        }
    }

    // lets generate children
    for p in no_children.iter_mut() {
        let child = generate_child(pend,comp, &p, hd_set);
        pend.list.push(child.clone());

        // we have to increase the prodigy of the parent
        let mut parent = pend.remove_with_id(p.id.unwrap()).unwrap();
        let res = parent.increase_prodigy();
        match res.is_ok() {
            true => {
                let index = p.id.unwrap() as usize - 1;
                pend.list.insert(index, parent);
                let line = format!("Task {} '{}' has been created.\n", child.id.unwrap(), child.description);
                to_orange_feedback(&line);
                pend.save();
            }
            false => {
                let message = "Something wrong in generating children (should not really happen)".to_string();
                feedback(Feedback::Error, message);
                exit(17);
            }
        }
    }
}

// no result needed as files could be messed up
pub fn load_all_tasks(  p_file: &str, c_file: &str, pending: &mut List, 
                        completed: &mut List, hexi_set: &mut Hdeci) {
    
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

pub fn load_task_file(task_file: &str, the_list: &mut List, hexi_set: &mut Hdeci) -> Result<(), String> {
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

        // only assign id if not completed
        match task.end {
            Some(_e) => {
                task.id = None;
            }
            None => {
                task.id = Some(line_counter);
            }
        }

        hexi_set.add(task.uuiid_int);

        // we have to do the virtual tags as well
        // task.virtual_tags = make_virtual_tags(task.clone());
        task.update_virtual_tags();

        the_list.list.push(task);
    }

    Ok(())
}

// make a string (line) from a task - dont add a newline here though
pub fn make_line_from_task(task: &Task) -> String {
    let mut ret = "".to_string();

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

    if task.ann.len() != 0 {
        for a in task.ann.clone() {
            ret.push_str("\tannotation_");
            let num = a.date.to_string() + ":";
            ret.push_str(&num);
            ret.push_str(&a.desc);
        }
    }
    if task.due.is_some() {                                                        //due
        ret.push_str("\tdue:");
        ret.push_str(&task.due.unwrap().to_string());
    }
    if task.end.is_some() {                                                        //due
        ret.push_str("\tend:");
        ret.push_str(&task.end.unwrap().to_string());
    }
    if task.parent.is_some() {                                                     //parent
        ret.push_str("\tparent:");
        ret.push_str(&task.parent.clone().unwrap());
    }
    if task.prodigy.is_some() {                                                    //parent
        ret.push_str("\tprodigy:");
        ret.push_str(&task.prodigy.unwrap().to_string());
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
    if task.timetrackingseconds != 0 {                                             //parent
        ret.push_str("\ttimetrackingseconds:");
        ret.push_str(&task.timetrackingseconds.to_string());
    }
    if task.wait.is_some() {                                                       //wait
        ret.push_str("\twait:");
        ret.push_str(&task.wait.unwrap().to_string());
    }

    // ret.push_str("\n");

    return ret;
}



// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    // use crate::library::{enums::Status, lts::lts_to_date_time_string};

    use super::*;
    
    // #[ignore]
    #[test]
    fn t001_load_task_file() {

        let mut h_set:Hdeci = Hdeci::new();
        
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

        let mut h_set:Hdeci = Hdeci::new();
        
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
        
        let h = h_set.get_next_hexidecimal();
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
        pen.save();

        remove_file(destination).expect("Cleanup test failed");

        assert_eq!(pen.list.len(), 2);
    }
    
    // #[ignore]
    #[test]
    fn t004_get_index_of_task_with_id() {

        // let mut completed: List = List::new();
        // let mut h_set:BTreeSet<i64> = BTreeSet::new();
        let mut hd_set: Hdeci         = Hdeci::new();
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
        pen.save();

        let mut pending_tasks = List::new(&destination);
        let _res = load_task_file(pending_tasks.file, &mut pending_tasks, &mut hd_set);
        remove_file(destination).expect("Cleanup test failed");
        
        let index = pending_tasks.get_index_of_task_with_id(2);
        assert_eq!(1, index);
        
        let ii = pending_tasks.get_index_of_task_with_uuiid("0x0011");
        assert_eq!(ii, 0);
    }








} //end of tests








