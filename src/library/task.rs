/*
    Module for everything to do with a task
    2022.07.24      Sven Ponelat

*/


use crate::library::enums::*;
use crate::library::functions::*;
use crate::library::lts::*;
use crate::library::structs::*;
use crate::library::list::*;

use std::str::FromStr;
use substring::Substring;





#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Annotation {
    pub date: i64,
    pub desc: String,
}


impl Annotation {
    pub fn new() -> Annotation {
        Annotation {
            date: 0,
            desc: "".to_string(),
        }
    }


}//end of impl



#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Task {
    pub ann: Vec<Annotation>, 
    pub description: String,
    pub due: Option<i64>,
    pub end: Option<i64>,
    pub entry: i64,
    pub id: Option<i64>,
    pub parent_int: Option<i64>,
    pub parent: Option<String>,
    pub prodigy: Option<i64>,
    pub recur: Option<String>,
    pub rtype: Option<Rtype>,
    pub start: Option<i64>,
    pub status: Status,
    pub tags: Vec<String>,
    pub timetrackingseconds: i64,
    pub uuiid_int: i64,
    pub uuiid: String,
    pub virtual_tags: Vec<VirtualTags>,
    pub wait: Option<i64>,



}


impl Task {
    
    pub fn has_due(&self) -> bool {
        if self.due.is_some(){
            return true;
        }
        return false;
    }

    pub fn has_prodigy(&self) -> bool {
        if self.prodigy.is_some(){
            return true;
        }
        return false;
    }

    pub fn has_recur(&self) -> bool {
        if self.recur.is_some(){
            return true;
        }
        return false;
    }
    
    pub fn has_start(&self) -> bool {
        if self.start.is_some(){
            return true;
        }
        return false;
    }

    pub fn has_wait(&self) -> bool {
        if self.wait.is_some(){
            return true;
        }
        return false;
    }

    pub fn increase_prodigy(&mut self) -> Result<(), &'static str> {
        if self.prodigy.is_some(){
            let mut num = self.prodigy.unwrap();
            num += 1;
            self.prodigy = Some(num);
            return Ok(())
        }
        return Err("task has no ability to have prodigy");
    }

    pub fn is_active(&self) -> bool {
        for v in self.virtual_tags.clone() {
            if v == VirtualTags::Active {
                return true;
            }
        } 
        return false;
    }
    
    pub fn is_annotated(&self) -> bool {
        if self.ann.len() > 0 {
            return true;
        }
        return false;
    }
    
    pub fn is_child(&self) -> bool {
        if self.has_recur() {
            if self.parent.is_some() {
                return true;
            }
        }
        return false;
    }

    pub fn is_complete(&self) -> bool {
        if self.end.is_some() {
            return true;
        }
        return false;
    }

    pub fn is_overdue(&self) -> bool {
        for v in self.virtual_tags.clone() {
            if v == VirtualTags::Overdue {
                return true;
            }
        } 
        return false;
    }

    pub fn is_parent(&self) -> bool {
        if self.has_recur() {
            if self.parent.is_none(){
                return true;
            }
        }
        return false;
    }

    pub fn is_periodic(&self) -> bool {
        if self.rtype.is_some(){
            if self.clone().rtype.unwrap() == Rtype::Periodic {
                return true;
            }
            return false;
        }
        return false;
    }

    pub fn is_recurring(&self) -> bool {
        if self.status == Status::Recurring {
            return true;
        }
        return false;
    }
    
    pub fn is_tagged(&self) -> bool {
        if self.tags.len() > 0 {
            return true;
        }
        return false;
    }

    pub fn is_waiting(&self) -> bool {
        for v in self.virtual_tags.clone() {
            if v == VirtualTags::Waiting {
                return true;
            }
        } 
        return false;
    }



    // make an empty task for compilers sake
    pub fn new() -> Task {
        Task { 
            ann: Vec::new(),
            description: "".to_string(),
            due: None,
            end: None,
            entry: lts_now(),
            id: None,
            parent_int: None,
            parent: None,
            prodigy: Some(0),
            recur: None,
            rtype: None,
            start: None,
            status: Status::Pending, 
            tags: Vec::new(),
            timetrackingseconds: 0,
            uuiid_int: 0,
            uuiid: "".to_string(),
            virtual_tags: Vec::new(),
            wait: None,
        }
    }


    pub fn update_status(&mut self) {
        let now = lts_now();

        // if this does not have a parent, and has recur, it is a parent; change to recurring
        if self.recur.is_some(){
            if self.parent.is_none(){
                self.status = Status::Recurring;
            }
        }
        match self.status {
            Status::Pending => {
                match self.wait {
                    Some(ts) => {
                        if now < ts && self.start.is_none() {
                            self.status = Status::Waiting;
                        } else {
                            self.status = Status::Pending;
                        }
                    }
                    None => {
                        self.status = Status::Pending;
                    }
                }
            }
            Status::Waiting => {
                match self.wait {
                    Some(ts) => {
                        if now < ts {
                            self.status = Status::Waiting;
                        } else {
                            self.status = Status::Pending;
                        }
                    }
                    None => {
                        self.status = Status::Pending;
                    }
                }
            }
            _ => {
            }
        }
    }
    
    pub fn update_virtual_tags(&mut self) {
        self.virtual_tags = Vec::new();
        let now = lts_now();
    
        // Active
        if self.start.is_some(){
            if !self.is_parent() {
                self.virtual_tags.push(VirtualTags::Active);
            }
        }
    
        // Annotated
        if self.ann.len() > 0 {
            self.virtual_tags.push(VirtualTags::Annotated);
        }
    
        // Child
        if self.parent.is_some() {
            self.virtual_tags.push(VirtualTags::Child);
        }
    
        // Completed
        if self.end.is_some() {
            self.virtual_tags.push(VirtualTags::Completed);
        }
    
        // Deleted
        if self.status == Status::Deleted {
            self.virtual_tags.push(VirtualTags::Deleted);
        }
        
        // Overdue
        if self.due.is_some() {
            if now > self.due.unwrap() {
                if !self.is_parent() {
                    self.virtual_tags.push(VirtualTags::Overdue);
                }
            }
        } 
        
        // Parent
        if self.status == Status::Recurring {
            self.virtual_tags.push(VirtualTags::Parent);
        }
        
        // Pending
        if self.status == Status::Pending {
            self.virtual_tags.push(VirtualTags::Pending);
        }
        
        // Tagged
        if self.tags.len() > 0 {
            self.virtual_tags.push(VirtualTags::Tagged);
        }
        
        // Waiting
        if self.status == Status::Waiting {
            self.virtual_tags.push(VirtualTags::Waiting);
        }
        // if self.wait.is_some() {
        //     if now < self.wait.unwrap() {
        //         if !self.is_parent() {
        //             self.virtual_tags.push(VirtualTags::Waiting);
        //         }
        //     }
        // }
    }

    




} //end of impl task










// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// given a term like +3m -> return timestamp
pub fn determine_due_start_wait(term: &str) -> Result<i64, &'static str> {
    // now
    if term.starts_with("now") {
        return Ok(lts_now());
    }

    // 1600 000 000
    let res_s64 = lts_from_str64_to_timestamp(term);
    if res_s64.is_ok(){
        return Ok(res_s64.unwrap())
    }
    
    // 2020-02-27
    let res_date = lts_date_string_to_timestamp(term);
    if res_date.is_ok(){
        return Ok(res_date.unwrap())
    }
    
    // +3m
    if term.starts_with("+") {
        let now = lts_now();
        let res_term = lts_add_timestamp_to_recur_term(now, term);
        if res_term.is_ok() {
            return Ok(res_term.unwrap())
        }
    }

    Err("unknown term for due: start: wait:")
}

pub fn generate_child(pend: &List, comp: &List, parent: &Task, hd_set: &mut Hdeci) -> Task {
    let mut ret = parent.clone();
    let now = lts_now();

    // update some defaults
    ret.entry = now;
    ret.parent = Some(parent.clone().uuiid);
    ret.parent_int = Some(parent.clone().uuiid_int);
    ret.start = None;
    ret.timetrackingseconds = 0;
    ret.prodigy = None;
    ret.status = Status::Pending;
    ret.end = None;

    let next_hexidecimal = hd_set.get_next_hexidecimal();
    ret.uuiid_int = next_hexidecimal;
    ret.uuiid = hexidecimal_to_string(next_hexidecimal);
    hd_set.add(next_hexidecimal);

    let new_id = pend.list.len() as i64  + 1;
    ret.id = Some(new_id);

    // lets sort out the times
    match parent.clone().rtype.unwrap() {
        Rtype::Chained => {
            let latest_child = find_latest_child(comp, parent);
            let term = RecurTerm::new(&parent.clone().recur.unwrap()).unwrap();

            match latest_child {
                Some(child) => {

                    let ets = child.end.unwrap();

                    match child.due {
                        Some(_ts) => {
                            let new_ts = term.multiply_from_timestring(ets, parent.prodigy.unwrap()); 
                            ret.due = Some( new_ts );
                        }
                        None => {
                            ret.due = None;
                        }
                    }
                    match child.wait {
                        Some(_ts) => {
                            match child.due {
                                Some(_ts) => {
                                    let diff = parent.due.unwrap() - parent.wait.unwrap();
                                    ret.wait = Some(ret.due.unwrap() - diff);
                                }
                                None => {
                                    let new_ts = term.multiply_from_timestring(parent.wait.unwrap(), parent.prodigy.unwrap()); 
                                    ret.due = Some( new_ts );
                                }
                            }
                            // let new_ts = term.multiply_from_timestring(ts, parent.prodigy.unwrap()); 
                            // ret.wait = Some( new_ts );
                        }
                        None => {
                            ret.wait = None;
                        }
                    }
                }
                None => {
                    match parent.due {
                        Some(ts) => {
                            let new_ts = term.multiply_from_timestring(ts, parent.prodigy.unwrap()); 
                            ret.due = Some( new_ts );
                        }
                        None => {
                            ret.due = None;
                        }
                    }
                    match parent.wait {
                        Some(ts) => {
                            let new_ts = term.multiply_from_timestring(ts, parent.prodigy.unwrap()); 
                            ret.wait = Some( new_ts );
                        }
                        None => {
                            ret.wait = None;
                        }
                    }
                }
            }
        }
        // @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Periodic @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
        Rtype::Periodic => {
            let term = RecurTerm::new(&parent.clone().recur.unwrap()).unwrap();
            match parent.due {
                Some(ts) => {
                    let new_ts = term.multiply_from_timestring(ts, parent.prodigy.unwrap()); 
                    ret.due = Some( new_ts );
                }
                None => {
                    ret.due = None;
                }
            }
            match parent.wait {
                Some(ts) => {
                    let new_ts = term.multiply_from_timestring(ts, parent.prodigy.unwrap()); 
                    ret.wait = Some( new_ts );
                }
                None => {
                    ret.wait = None;
                }
            }
        }       
    }

    ret.update_status();

    return ret;
}

// Main make task function
pub fn make_task(vec:Vec<&str>) -> Result<Task, &'static str> {
    let mut ret = Task::new();
    let mut split_colon:Vec<_> = Vec::new();
    // let now = lts_now();

    for element in vec {
        let starts_with = element.substring(0, 3);
        match starts_with {
            "des" | "ann" => {
                split_colon.clear();
                let res_opt = element.split_once(":");
                if res_opt.is_none(){
                    return Err("Something wrong with description line.");
                }
                split_colon.push(res_opt.unwrap().0);
                split_colon.push(res_opt.unwrap().1);
            }
            _ => {
                split_colon = element.split(":").collect();
            }
        }

        // match element.starts_with("description") {
        //     true => {
        //         let res_opt = element.split_once(":");
        //         if res_opt.is_none(){
        //             return Err("Something wrong with description line.");
        //         }
        //         split_colon.push(res_opt.unwrap().0);
        //         split_colon.push(res_opt.unwrap().1);
        //     }
        //     false => {
        //         split_colon = element.split(":").collect();
        //     }
        // }

        let number_of_terms = split_colon.len();
        match number_of_terms {
            1 => {
                if split_colon[0].len() < 2 {
                    return Err("tag term is too short")
                }
                let one =  split_colon[0].clone().to_string();
                let first_char = one.substring(0, 1);
                match first_char {
                    "+" => {
                        let tag = one[1..].to_string();
                        ret.tags.push(tag);
                    }
                    
                    _ => {
                        if ret.description.len() != 0 {
                            return Err("too many descriptions")
                        } 
                        ret.description = one;
                    }
                }
            }

            2 => {

                // to take care of annotation with time, i'm going to make a separate match term
                let mut matcho = split_colon[0];
                if matcho.starts_with("annotation") {
                    matcho = "annotation";
                }

                match matcho {
                    "annotation" => {
                        let split_ann:Vec<_> = split_colon[0].split("_").collect();
                        if split_ann.len() != 2 {
                            // let message = format!("Line in file: {} has faulty annotations",path);
                            return Err("element has faulty annotations");           
                        }
                        let mut anno = Annotation::new();
                        let date = split_ann[1].parse::<i64>();
                        if date.is_err(){
                            // let message = format!("Line in file: {} has faulty annotations times",path);
                            return Err("element has faulty annotations times(date)");  
                        }
                        anno.date = date.unwrap();
                        anno.desc = split_colon[1].to_string();
                        ret.ann.push(anno);

                    }

                    "description" => {
                        if ret.description.len() != 0 {
                            return Err("too many descriptions")
                        } 
                        ret.description = split_colon[1].to_string();
                    }
                    
                    "due" => {
                        let res = determine_due_start_wait(split_colon[1]);
                        if res.is_err() {
                            return Err(res.err().unwrap())
                        }
                        ret.due = Some(res.unwrap());
                    }
                    
                    "end" => {
                        let res= split_colon[1].parse::<i64>();
                        if res.is_err(){
                            return Err("Integer parsing error");           
                        }
                        ret.end = Some(res.unwrap());
                    }

                    "entry" => {
                        let res= split_colon[1].parse::<i64>();
                        if res.is_err(){
                            return Err("Integer parsing error");           
                        }
                        ret.entry = res.unwrap();
                    }
                    
                    "parent" => {
                        let parent = split_colon[1].to_string();
                        let res = hexi_verify(&parent);
                        if res.is_err(){
                            return Err("faulty hex values");           
                        }
                        ret.parent = Some(parent);
                        ret.parent_int = Some(res.unwrap());
                    }
                    
                    "prodigy" => {
                        let res= split_colon[1].parse::<i64>();
                        if res.is_err(){
                            return Err("Integer parsing error");          
                        }
                        ret.prodigy = Some(res.unwrap());
                    }
                    
                    "recur" => {
                        ret.recur = Some(split_colon[1].to_string());

                        //default to periodic
                        if ret.rtype.is_none(){
                            ret.rtype = Some(Rtype::Periodic);
                        }
                    }
                    
                    "rtype" => {
                        let res = Rtype::from_str(split_colon[1]);
                        if res.is_err(){
                            return Err("Rtype parsing error");         
                        }
                        ret.rtype = Some(res.unwrap());
                    }
                    
                    "start" => {
                        let res = determine_due_start_wait(split_colon[1]);
                        if res.is_err() {
                            return Err(res.err().unwrap())
                        }
                        ret.start = Some(res.unwrap());
                    }
                    
                    "status" => {
                        let res = Status::from_str(split_colon[1]);
                        if res.is_err(){
                            return Err("Status parsing error");         
                        }
                        ret.status = res.unwrap();
                    }
                    
                    "tags" => {
                        let split_comma:Vec<_> = split_colon[1].split(",").collect();
                        for tag in split_comma {
                            ret.tags.push(tag.to_string());
                        }
                    }
                    
                    "timetrackingseconds" => {
                        let res= split_colon[1].parse::<i64>();
                        if res.is_err(){
                            return Err("timetrackingseconds parsing error");             
                        }
                        ret.timetrackingseconds = res.unwrap();
                    }

                    "uuiid" => {
                        let uuiid = split_colon[1].to_string();
                        let res = hexi_verify(&uuiid);
                        if res.is_err(){
                            return Err("faulty hex values");           
                        }
                        ret.uuiid = uuiid;
                        let u_int = res.unwrap();
                        ret.uuiid_int = u_int;
                    }

                    "wait" => {
                        let res = determine_due_start_wait(split_colon[1]);
                        if res.is_err() {
                            return Err(res.err().unwrap())
                        }
                        ret.wait = Some(res.unwrap());
                    }
                    
                    _ => {
                        // shouldnt really get here
                        return Err("Unknown element in colon split")            
                    }
                }
            }
            _ => {
                return Err("too many terms per element")
            }
        }
    } // end of for element
    ret.update_status();
    ret.update_virtual_tags();

    // Check initial prodigy is valid
    match ret.is_parent(){
        true => {
            // parents dont start
            ret.start = None;
        }
        false => {
            // only parents have prodigy
            ret.prodigy = None;
        }
    }

    Ok(ret)
}

// pub fn make_virtual_tags(task: Task) -> Vec<VirtualTags> {
//     let mut ret: Vec<VirtualTags> = Vec::new();
//     let now = lts_now();

//     // Active
//     if task.start.is_some(){
//         if !task.is_parent() {
//             ret.push(VirtualTags::Active);
//         }
//     }

//     // Annotated
//     if task.ann.len() > 0 {
//         ret.push(VirtualTags::Annotated);
//     }

//     // Child
//     if task.parent.is_some() {
//         ret.push(VirtualTags::Child);
//     }

//     // Completed
//     if task.end.is_some() {
//         ret.push(VirtualTags::Completed);
//     }

//     // Deleted
//     if task.status == Status::Deleted {
//         ret.push(VirtualTags::Deleted);
//     }
    
//     // Overdue
//     if task.due.is_some() {
//         if now > task.due.unwrap() {
//             if !task.is_parent() {
//                 ret.push(VirtualTags::Overdue);
//             }
//         }
//     } 
    
//     // Parent
//     if task.status == Status::Recurring {
//         ret.push(VirtualTags::Parent);
//     }
    
//     // Pending
//     if task.status == Status::Pending {
//         ret.push(VirtualTags::Pending);
//     }
    
//     // Tagged
//     if task.tags.len() > 0 {
//         ret.push(VirtualTags::Tagged);
//     }
    
//     // Waiting
//     if task.wait.is_some() {
//         if now < task.wait.unwrap() {
//             if !task.is_parent() {
//                 ret.push(VirtualTags::Waiting);
//             }
//         }
//     }

//     return ret;
// }


// // update the status
// pub fn update_status(now: i64, task: Task) -> Status {
//     // if this does not have a parent, and has recur, it is a parent; change to recurring
//     if task.recur.is_some(){
//         if task.parent.is_none(){
//             return Status::Recurring;
//         }
//     }
//     match task.status {
//         Status::Pending => {
//             if task.wait.is_some(){
//                 if now < task.wait.unwrap() {
//                     return Status::Waiting;
//                 }
//                 return Status::Pending;
//             }
//             return Status::Pending;
//         }
//         Status::Waiting => {
//             if now > task.wait.unwrap() {
//                 return Status::Pending;
//             }
//             return Status::Waiting;
//         }
//         _ => {
//             return task.status
//         }
//     }
// }












// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    
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
    
    // #[ignore]
    #[test]
    fn t002_hexi() {
        let str = hexidecimal_to_string(15);
        assert_eq!(str, "0x00000f".to_string());
    }

    // #[ignore]
    #[test]
    fn t003_virtual_tags() {
        let vs: Vec<&str> = vec!["First Task", "due:2040-01-05", "+household"];
        let res = make_task(vs);
        let pending = res.clone().unwrap().virtual_tags.contains(&VirtualTags::Pending);
        assert_eq!(pending, true);
        
        let waiting =res.clone().unwrap().virtual_tags.contains(&VirtualTags::Waiting);
        assert_eq!(waiting, false);
        
        let tagged =res.clone().unwrap().virtual_tags.contains(&VirtualTags::Tagged);
        assert_eq!(tagged, true);
        
        let overdue =res.clone().unwrap().virtual_tags.contains(&VirtualTags::Overdue);
        assert_eq!(overdue, false);
    }
    
    
    // #[ignore]
    #[test]
    fn t004_split() {
        let destination = "./test/trial.data";
        let mut pen = List::new(destination);
        let mut hdeci: Hdeci = Hdeci::new();
        let vs: Vec<String> = vec![ "1".to_string(), 
                                    "1".to_string(), 
                                    "First Task: due to some error: --> hello!".to_string(),
                                    "due:2040-01-05".to_string(), 
                                    "+household".to_string()];
        let _res = command_add_task(&vs, &mut pen, &mut hdeci);
        let task = pen.get_task_from_id(1);
        let t1 = task.clone().unwrap().virtual_tags.contains(&VirtualTags::Pending);
        assert_eq!(t1, true);
        
        remove_file(destination).expect("Cleanup test failed");
        let waiting = task.clone().unwrap().virtual_tags.contains(&VirtualTags::Waiting);
        assert_eq!(waiting, false);
        
        let tagged = task.clone().unwrap().virtual_tags.contains(&VirtualTags::Tagged);
        assert_eq!(tagged, true);
        
        let overdue = task.clone().unwrap().virtual_tags.contains(&VirtualTags::Overdue);
        assert_eq!(overdue, false);
    }


    // #[ignore]
    #[test]
    fn t005_make_task() {
        let vs: Vec<&str> = vec!["First Task", "due:2030-01-05", "start:now", "+household"];
        let res = make_task(vs);
        assert_eq!(res.unwrap().start.unwrap(), lts_now() );
        
        //from line in file
        let line = "description:how do i get the konsole that i have now\tdue:1658513756\t\
                        entry:1658513756\tstart:1658513756\tstatus:pending\tuuiid:0x0011";
        let vec:Vec<_> = line.split("\t").collect();
        let task = make_task(vec);
        assert_eq!(task.clone().unwrap().start.unwrap(), 1658513756 );
        assert_eq!(task.clone().unwrap().status, Status::Pending );
        assert_eq!(task.clone().unwrap().uuiid_int, 17 );
    }















} //end of tests
