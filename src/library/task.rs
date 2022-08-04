/*
    Module for everything to do with a task
    2022.07.24      Sven Ponelat

*/


use std::time::SystemTime;
use chrono::*;
use chronoutil::*;
use crate::library::enums::*;
// use dateparser::DateTimeUtc;


const DAY_SECS: i64         =      86400;
const WEEK_SECS: i64        =     604800;
const DATE_FORMAT: &str     = "%Y-%m-%d";
const ADD_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";



#[derive(Clone )]
pub struct Task {
    pub uuiid: String,
    pub uuiid_int: Option<i64>,
    pub id: Option<i64>,
    pub description: String,
    pub entry: i64,
    pub start: Option<i64>,
    pub due: Option<i64>,
    pub end: Option<i64>,
    pub wait: Option<i64>,
    // pub modified: Option<i64>,
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
            uuiid: "".to_string(),
            uuiid_int: None,
            description: "".to_string(),
            status: Status::Pending, 
            entry: Utc::now().naive_local().timestamp(),
            start: None,
            due: None,
            end: None,
            wait: None,
            // modified: None,
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
pub fn make_task(args: &Vec<String>, uuiid_int: i64, id: i64) -> Result<Task, &'static str> {
    let mut ret = Task::new();
    ret.id = Some(id);
    ret.uuiid_int = Some(uuiid_int);
    ret.uuiid = make_hexi(uuiid_int);

    //lets do args[2] here
    let desc = args[2].trim().to_string();
    ret.description = desc;

    

    for t in 3..=args.len() {
        let split: Vec<&str> = args[t].split(":").collect();

        match split[0] {
            "due" => {
                if split.len() != 2 {
                    return Err("Malformed due term");
                }
                let resultant_time = determine_timestamp( &ret.entry, split[1]);
                if resultant_time.is_err(){
                    return Err("parsing error in term");
                }
                ret.due = Some(resultant_time.unwrap());
            }



            _ => {

            }
        }



        // println!("{}",a)
    }




    Ok(ret)
}


pub fn make_hexi(uuiid_int: i64) -> String {
    // make it hex
    let str = format!("{:x}",uuiid_int);

    // pad with leading zeros with up to six places
    let lead = format!("{:0>6}",str);

    // add the 0x
    let ret = "0x".to_string() + lead.as_str();

    return ret;
}


pub fn determine_timestamp(time: &i64, term: &str) -> Result< i64, &'static str> {
    let char_arr: Vec<char> = term.chars().collect();

    // if a date is given eg 2022-08-03
    if char_arr[0] != '+' {
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
    
}


pub fn make_naive_dt_from_str(date_str: &str) -> Result<NaiveDateTime, &'static str> {
    let res = NaiveDate::parse_from_str(date_str, DATE_FORMAT);
    if res.is_err() {
        return Err("Parse error from date string");
    } 
    let ret = res.unwrap().and_hms(0, 0, 0);
    return Ok(ret);
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
        let str = make_hexi(15);
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












} //end of tests
