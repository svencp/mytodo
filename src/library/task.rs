/*
    Module for everything to do with a task
    2022.07.24      Sven Ponelat

*/


use std::str::FromStr;
use chrono::*;
use chronoutil::*;
use substring::Substring;
use crate::library::enums::*;
use crate::library::functions::*;
use crate::library::lts::*;
use crate::library::structs::*;






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



#[derive(Clone )]
pub struct Task {
    pub uuiid: String,
    pub uuiid_int: i64,
    pub id: Option<i64>,
    pub description: String,
    pub entry: i64,
    pub start: Option<i64>,
    pub due: Option<i64>,
    pub end: Option<i64>,
    pub wait: Option<i64>,
    pub prodigy: Option<i64>,
    pub parent: Option<String>,
    pub parent_int: Option<i64>,
    pub recur: Option<String>,
    pub status: Status,
    pub rtype: Option<Rtype>,
    pub tags: Vec<String>,
    pub virtual_tags: Vec<String>,
    pub timetrackingseconds: i64,
    pub ann: Vec<Annotation>,



}


impl Task {
    
    // make an empty task for compilers sake
    pub fn new() -> Task {
        Task { 
            id: None,
            uuiid: "".to_string(),
            uuiid_int: 0,
            description: "".to_string(),
            status: Status::Pending, 
            // entry: chrono::offset::Local::now().timestamp().timestamp(),
            entry: lts_now(),
            start: None,
            due: None,
            end: None,
            wait: None,
            prodigy: None,
            parent: None,
            parent_int: None,
            recur: None,
            rtype: None,
            tags: Vec::new(),
            virtual_tags: Vec::new(),
            ann: Vec::new(),
            timetrackingseconds: 0,
        }
    }






}










// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

pub fn from_timestamp_to_date_str(num: i64) -> String {
    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp(num, 0);

    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    
    // Format the datetime how you want
    let newdate = naive.format("%Y-%m-%d %H:%M:%S");

    return newdate.to_string()
}


pub fn make_task(vec:Vec<&str>) -> Result<Task, &'static str> {
    let mut ret = Task::new();

    for element in vec {
        let split_colon: Vec<_> = element.split(":").collect();
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

                        // let res= split_colon[1].parse::<i64>();
                        // if res.is_err(){
                        //     let term = split_colon[1].trim().to_lowercase();
                        //     if term.starts_with("now") {
                        //         ret.due = Some(chrono::offset::Local::now().timestamp());
                        //     }
                        //     else {
                        //         return Err("Integer parsing error");           
                        //     }
                        // } 
                        // else {
                        //     ret.due = Some(res.unwrap());
                        // }
                    }
                    
                    "end" => {
                        let res= split_colon[1].parse::<i64>();
                        if res.is_err(){
                            // let message = format!("Integer parsing error in file: {}",path);
                            return Err("Integer parsing error");           
                        }
                        ret.end = Some(res.unwrap());
                    }

                    "entry" => {
                        let res= split_colon[1].parse::<i64>();
                        if res.is_err(){
                            // let message = format!("Integer parsing error in file: {}",path);
                            return Err("Integer parsing error");           
                        }
                        ret.entry = res.unwrap();
                    }
                    
                    "parent" => {
                        let parent = split_colon[1].to_string();
                        let res = hexi_verify(&parent);
                        if res.is_err(){
                            // let message = format!("Line in file: {} has faulty hex values",path);
                            return Err("faulty hex values");           
                        }
                        ret.parent = Some(parent);
                        ret.parent_int = Some(res.unwrap());
                    }
                    
                    "prodigy" => {
                        let res= split_colon[1].parse::<i64>();
                        if res.is_err(){
                            // let message = format!("Integer parsing error in file: {}",path);
                            return Err("Integer parsing error");          
                        }
                        ret.prodigy = Some(res.unwrap());
                    }
                    
                    "recur" => {
                        ret.recur = Some(split_colon[1].to_string());
                    }
                    
                    "rtype" => {
                        let res = Rtype::from_str(split_colon[1]);
                        if res.is_err(){
                            // let message = format!("Rtype parsing error in file: {}",path);
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

                        // let res= split_colon[1].parse::<i64>();
                        // if res.is_err(){
                        //     let term = split_colon[1].trim().to_lowercase();
                        //     if term.starts_with("now") {
                        //         ret.start = Some(chrono::offset::Local::now().timestamp());
                        //     }
                        //     else {
                        //         return Err("Integer parsing error");           
                        //     }
                        // } 
                        // else {
                        //     ret.start = Some(res.unwrap());
                        // }
                    }
                    
                    "status" => {
                        let res = Status::from_str(split_colon[1]);
                        if res.is_err(){
                            // let message = format!("Status parsing error in file: {}",path);
                            return Err("Status parsing error");         
                        }
                        ret.status = res.unwrap();
                    }
                    
                    "tags" => {
                        let split_comma:Vec<_> = split_colon[1].split(":").collect();
                        for tag in split_comma {
                            ret.tags.push(tag.to_string());
                        }
                    }
                    
                    "timetrackingseconds" => {
                        let res= split_colon[1].parse::<i64>();
                        if res.is_err(){
                            // let message = format!("timetrackingseconds parsing error in file: {}",path);
                            return Err("timetrackingseconds parsing error");             
                        }
                        ret.timetrackingseconds = res.unwrap();
                    }

                    "uuiid" => {
                        let uuiid = split_colon[1].to_string();
                        let res = hexi_verify(&uuiid);
                        if res.is_err(){
                            // let message = format!("Line in file: {} has faulty hex values",path);
                            return Err("faulty hex values");           
                        }
                        ret.uuiid = uuiid;
                        let u_int = res.unwrap();
                        ret.uuiid_int = u_int;
                        // h_set.insert(u_int);
                    }

                    "wait" => {
                        let res = determine_due_start_wait(split_colon[1]);
                        if res.is_err() {
                            return Err(res.err().unwrap())
                        }
                        ret.wait = Some(res.unwrap());

                        // let res= split_colon[1].parse::<i64>();
                        // if res.is_err(){
                        //     // let message = format!("Integer parsing error in file: {}",path);
                        //     return Err("Integer parsing error");         
                        // }
                        // ret.wait = Some(res.unwrap());
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

    }




    Ok(ret)
}


// pub fn make_hexi(uuiid_int: i64) -> String {
//     // make it hex
//     let str = format!("{:x}",uuiid_int);

//     // pad with leading zeros with up to six places
//     let lead = format!("{:0>6}",str);

//     // add the 0x
//     let ret = "0x".to_string() + lead.as_str();

//     return ret;
// }


pub fn determine_timestamp(time: &i64, term: &str) -> Result< i64, &'static str> {
    let char_arr: Vec<char> = term.chars().collect();

    // if NOT +
    if char_arr[0] != '+' {

        // if now
        if term == "now" {
            let ret = chrono::offset::Local::now().timestamp();
            return Ok(ret);
        }

        // if date eg 2022-09-08
        let res_date = NaiveDate::parse_from_str(term, DATE_FORMAT);
        if res_date.is_err() {
            return Err("Error in parsing date (or maybe no + symbol)")
        }
        let date_time = res_date.unwrap().and_hms(0, 0, 0);
        let timestamp = date_time.timestamp();
        return Ok(timestamp);
    }

    // + 
    let str = term.replace("+", "");
    let mut n_arr:Vec<char> = Vec::new();
    let mut c_arr:Vec<char> = Vec::new();
    let str_arr: Vec<char> = str.chars().collect();
    for c in str_arr {
        if c.is_numeric() {
            n_arr.push(c);
            continue;
        }
        c_arr.push(c);
    }

    // is it a number
    let s_num: String = n_arr.iter().collect();
    let res_num = s_num.parse::<i64>();
    if res_num.is_err() {
        return Err("Number could not be parsed");
    }
    let num = res_num.unwrap();
    
    // has the term got the right chars (only d,w,m,y)
    // let s_char: String = c_arr.iter().collect();
    if c_arr.len() > 1 {
        return Err("Too many characters in duration");
    }
    if c_arr.len() < 1 {
        return Err("No duration symbol given");
    }

    let time_ndt = NaiveDateTime::from_timestamp(*time, 0);

    match c_arr[0] {
        'd' => {
            let addition = num * DAY_SECS;
            let ret = time_ndt.timestamp() + addition;
            return Ok(ret);
        }
        'w' => {
            let addition = num * WEEK_SECS;
            let ret = time_ndt.timestamp() + addition;
            return Ok(ret);
        }
        'm' => {
            let delta = RelativeDuration::months(num as i32);
            let ndt = time_ndt + delta;
            return Ok(ndt.timestamp());
        }
        'y' => {
            let delta = RelativeDuration::years(num as i32);
            let ndt = time_ndt + delta;
            return Ok(ndt.timestamp());
        }
        _ => {
            return Err("Illegal duration symbol");
        }
    }
    
} // end of determine_timestamp





pub fn make_naive_dt_from_str(date_str: &str) -> Result<NaiveDateTime, &'static str> {
    let res = NaiveDate::parse_from_str(date_str, DATE_FORMAT);
    if res.is_err() {
        return Err("Parse error from date string");
    } 
    let ret = res.unwrap().and_hms(0, 0, 0);
    return Ok(ret);
}

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
    
    // #[ignore]
    #[test]
    fn t002_hexi() {
        let str = hexidecimal_to_string(15);
        assert_eq!(str, "0x00000f".to_string());
    }

    // #[ignore]
    #[test]
    fn t003_determine_tstamp1() {
        let date = "2022-05-01";
        let ndt = make_naive_dt_from_str(date).unwrap();

        let d = determine_timestamp(&ndt.timestamp(), "+365d");
        assert_eq!(d.unwrap(), 1682899200);
        
        let w = determine_timestamp(&ndt.timestamp(), "+52w");
        assert_eq!(w.unwrap(), 1682812800);
        
        let m = determine_timestamp(&ndt.timestamp(), "+14m");
        assert_eq!(m.unwrap(), 1688169600);
        
        let y = determine_timestamp(&ndt.timestamp(), "+2y");
        assert_eq!(y.unwrap(), 1714521600);
        
        let e1 = determine_timestamp(&ndt.timestamp(), "+365 d");
        assert_eq!(e1.is_err(), true);
        
        let e2 = determine_timestamp(&ndt.timestamp(), "+365f");
        assert_eq!(e2.is_err(), true);
        
        let e3 = determine_timestamp(&ndt.timestamp(), "+365");
        assert_eq!(e3.is_err(), true);
        
        let e4 = determine_timestamp(&ndt.timestamp(), "365");
        assert_eq!(e4.is_err(), true);
        
        let e5 = determine_timestamp(&ndt.timestamp(), "+w");
        assert_eq!(e5.is_err(), true);
        
        let e6 = determine_timestamp(&ndt.timestamp(), "2w");
        assert_eq!(e6.is_err(), true);
    }
    
    
    // #[ignore]
    #[test]
    fn t004_determine_due_start_wait() {
        // let term1 = "now";


        // let term2 = "2021-09-30";


        // let vs: Vec<String> = vec!["Nutting".to_string(), "add".to_string(), "Do a job".to_string(),
        //                         "due:2030-01-05".to_string(), "start:now".to_string(), "+household".to_string()];
        // let result = make_task(&vs, 26, 30);
        // let now = chrono::offset::Local::now().timestamp().timestamp();
        // assert_eq!(result.unwrap().start.unwrap(), now);
        
        // let vs2: Vec<String> = vec!["Nutting".to_string(), "add".to_string(), "Do a job2".to_string(),
        //                         "due:now".to_string(), "recur:+4m".to_string(), "rtype:chained".to_string()];
        // let result = make_task(&vs2, 2, 2);
        // assert_eq!(result.unwrap().rtype.unwrap(), Rtype::Chained);
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
