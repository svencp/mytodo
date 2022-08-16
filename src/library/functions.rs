
/*
        Module for everything all main functions
        2022.07.31      Sven Ponelat

*/


use substring::Substring;
use std::convert::TryFrom;
use std::process::exit;
use termion::{color, style};
use crate::library::task::*;
use crate::library::list::*;
use crate::library::lts::*;
use crate::library::structs::*;
use crate::library::my_utils::*;
use crate::library::settings::*;
use crate::library::enums::*;
use crate::library::reports::*;



// function to add task from command line
pub fn command_add_task(args: &Vec<String>,  pending: &mut List, hdeci: &mut Hdeci ) -> Result<i64, String> {

    // for add - remove the first two arguments
    let result = shorten_front_of_vec_by_2(&args);
    if result.is_err(){
        let message = result.err().unwrap().to_string();
        return Err(message);
    }

    let t_result = make_task(result.unwrap());
    if t_result.is_err(){
        let message = t_result.err().unwrap().to_string();
        return Err(message);
    }

    let mut task = t_result.unwrap();
    let len_pen = pending.list.len() as i64;
    let id = len_pen + 1;
    task.id = Some(id);

    let next_hexidecimal = hdeci.get_next_hexidecimal();
    task.uuiid_int = next_hexidecimal;
    task.uuiid =  hexidecimal_to_string(next_hexidecimal);
    hdeci.add(next_hexidecimal);

    pending.list.push(task);
    pending.save();
    // if save.is_err(){
    //     let message = save.err().unwrap();
    //     return Err(message);
    // }

    Ok(pending.list.len() as i64)
}

//function to complete tasks; return number of tasks completed
pub fn command_done(colors: &Colors, vec_int:Vec<i64>, pending: &mut List, completed: &mut List ) -> Result<i64, &'static str> {
    // remember that tasks are not zero based
    let len = pending.list.len() as i64;
    let mut vec_mess:Vec<String> = Vec::new();

    for element in vec_int.clone() {
        if element > len {
            return Err("Included task number greater than number of tasks")
        }

        let index = ( element - 1) as usize;
        let mut task = &mut pending.list[index];
        // let mut task = *pending.list.clone().get(index).unwrap();
        if task.id.unwrap() != ( index + 1 ) as i64 {
            return Err("a task has been fetched whose id's don't match")
        }

        task.end = Some(lts_now());
        task.status = Status::Completed;
        
        let start:i64;
        match task.start {
            None => {
                start = task.entry;
            }
            Some(i) => {
                start = i;
            }
        }

        task.timetrackingseconds = task.timetrackingseconds + (task.end.unwrap() - start);
        task.start = None;

        // copy to completed
        let c_task = task.clone();
        completed.list.insert(0, c_task);

        // give message
        println!("Completed task {} '{}'",task.uuiid, task.description);


    } //end of for element

    let size = vec_int.len();
    println!("Completed {} {}",size, units("task",size));


    //loop over vector while deciding to remove element
    pending.list.retain(|task| {
        let mut remove = false;
        if task.status == Status::Completed {
            remove = true;
            let line = format!("Total Time Tracked: {}\n",make_timetracking_string(task.timetrackingseconds));
            to_orange_feedback(colors, &line);
        }
        !remove
    });

    Ok(vec_int.len() as i64)
}


// start the given tasks
pub fn command_start(vec_int:Vec<i64>, pending: &mut List ) -> Result<i64, &'static str> {
    // remember that tasks are not zero based
    let len = pending.list.len() as i64;
    let mut num_started = 0 as i64;

    for element in vec_int.clone() {
        if element > len {
            return Err("Included task number greater than number of tasks")
        }

        let index = ( element - 1) as usize;
        let mut task = &mut pending.list[index];
        if task.id.unwrap() != ( index + 1 ) as i64 {
            return Err("a task has been fetched whose id's don't match")
        }

        if task.start.is_some() {
            println!("Task {} '{}' already started.",task.id.unwrap(), task.description);
            continue;
        }

        task.start = Some(lts_now());
        num_started += 1;
    }

    Ok( num_started )
}

// stop the given tasks
pub fn command_stop(vec_int:Vec<i64>, pending: &mut List, sett: &SettingsMap ) -> Result<(), &'static str> {
    // remember that tasks are not zero based
    let len = pending.list.len() as i64;
    let mut num_stopped = 0 as i64;
    let stop = lts_now();
    let mut vec_mess:Vec<String> = Vec::new();

    for element in vec_int.clone() {
        if element > len {
            return Err("Included task number greater than number of tasks")
        }

        let index = ( element - 1) as usize;
        let mut task = &mut pending.list[index];
        if task.id.unwrap() != ( index + 1 ) as i64 {
            return Err("a task has been fetched whose id's don't match")
        }

        if task.start.is_none() {
            println!("Task {} '{}' has not started.",task.id.unwrap(), task.description);
            continue;
        }
        
        task.timetrackingseconds = task.timetrackingseconds + ( stop - task.start.unwrap());
        task.status = Status::Pending;
        task.start = None;
        
        println!("Stopping task {} '{}'.",task.uuiid, task.description);

        num_stopped += 1;

        let mess = "Total Time Tracked: ".to_string() + &make_timetracking_string(task.timetrackingseconds);
        vec_mess.push(mess);
    }

    let s1 = sett.clone().get_color("color_general_orange");
    if s1.is_err(){
        return Err("Colour missing in settings file.")
    }

    println!("Stopped {} {}.",num_stopped.to_string(), units("task",num_stopped as usize));
    
    let my_report_orange: color::Rgb = s1.unwrap();
    for line in vec_mess {
        print!("{}",color::Fg(my_report_orange));
        print!("{}.", line.to_string()); 
        print!("{}\n", style::Reset);  
    }

    Ok(())
}

// determine the first argument type
pub fn determine_first_arg(args: &Vec<String>, v_int: &mut Vec<i64>, v_hex: &mut Vec<String>, command: &mut String) -> ArgType {
    
    // if none
    if args.len() == 1 {
        return ArgType::None
    }
    
    let first = args[1].as_str();
    
    let res_int = is_arg_integer(first);
    if res_int.is_ok(){
        *v_int = res_int.unwrap();
        return ArgType::Integer;
    }
    
    let res_hex = is_arg_hexidecimal(first);
    if res_hex.is_ok(){
        *v_hex = res_hex.unwrap();
        return ArgType::Hexidecimal;
    }
    
    let res_com = is_arg_command(first);
    if res_com.is_ok(){
        *command = res_com.unwrap().to_string();
        return ArgType::Command;
    }
    
    return ArgType::Unknown;
}

// determine the second argument type
pub fn determine_second_arg(args: &Vec<String>, command: &mut String) -> ArgType {
    // if none
    if args.len() == 2 {
        return ArgType::None
    }
    
    let second = args[2].as_str();    
    let res_com = is_arg_secondary_command(second);
    if res_com.is_ok(){
        *command = res_com.unwrap().to_string();
        return ArgType::Command;
    }
    
    return ArgType::Unknown;
}

pub fn get_integer_single_report(settings: &SettingsMap, colors: Colors, id: i64, pending: &List)
                                -> Result<(), &'static str> {
    let mut found = false;
    let mut task:Task = Task::new();

    for t in pending.clone().list {
        let t_id = t.id.unwrap();
        if id == t_id {
            found = true;
            task = t;
            break;
        }
    }

    if ! found {
        return Err("Task id does not exist.")
    }

    let width = settings.get_integer("useTerminalWidthOf");
    if width.is_err() {
        return Err("Cannot find terminal width.")
    }
    let u_width = usize::try_from(width.unwrap()).unwrap();

    let result = report_single(u_width, colors, task);
    if result.is_err() {
        return Err(result.err().unwrap());
    }

    Ok(())
}


// get the termianl width size in characters
pub fn get_terminal_width(settings: &SettingsMap) -> i64 {
    let res = settings.get_integer("useTerminalWidthOf");
    if res.is_err(){
        let message = format!("Cannot determine dimensions of terminal from settings.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    return res.unwrap();
}


// function to verify my hexidecimal string
pub fn hexi_verify(str: &str) -> Result<i64, &'static str> {
    let sub2 = "0x";
    
    if str.substring(0, 2) != sub2 {
        return Err("Does not start with 0x");
    }
    let n_hexi = str.trim_start_matches(sub2);
    let res_int = i64::from_str_radix(n_hexi, 16);
    if res_int.is_err() {
        return Err("Not a hexidecimal string");
    }
    
    Ok(res_int.unwrap())
}

// Function to determine if first argument is a command
pub fn is_arg_command(first: &str) -> Result< &str, &str> {
    
    match first {
        "add" => {
            return Ok(first);
        }
        
        "mycompleted" => {
            return Ok(first);
        }
        
        "-v" | "-version" | "v" | "ver" | "version" | "-ver" => {
            return Ok("version");
        }
        
        "-c" | "-color_test" | "c" | "color" | "color_test" | "-color" => {
            return Ok("colortest");
        }

        _ => {
            return Err("unknown command");
        }
    }
}

// Function to determine whether the first argument is hexidecimal
pub fn is_arg_hexidecimal(first: &str) -> Result<Vec<String>, &str> {
    let mut ret: Vec<String> = Vec::new(); 
    let split: Vec<&str> = first.split(",").collect();

    for hexi in split {

        let res = hexi_verify(hexi);
        if res.is_err() {
            return Err(res.err().unwrap());
        }

        ret.push(hexi.to_lowercase().trim().to_string());
    }

    Ok(ret)
}

// Function to determine whether the first argument is an ineteger
pub fn is_arg_integer<'a>(first: &str) -> Result<Vec<i64>, &str> {
    let mut ret: Vec<i64> = Vec::new(); 
    let split: Vec<&str> = first.split(",").collect();
    
    for num in split {
        let res_int = num.parse::<i64>();
        if res_int.is_err() {
            return Err("Not an integer");
        }
        ret.push(res_int.unwrap());
    }
    
    Ok(ret)
}



// Function to determine if first argument is a command
pub fn is_arg_secondary_command(second: &str) -> Result< &str, &str> {
    if second.len() < 3 {
        return Err("second argument is too short");
    }
    
    let term = second.substring(0, 3);
    match term {
        "ann" => {
            return Ok(term);
        }
        
        "del" => {
            return Ok(term);
        }
        
        "den" => {
            return Ok(term);
        }
        
        "don" => {
            return Ok(term);
        }
        
        "dup" => {
            return Ok(term);
        }
        
        "hel" => {
            return Ok(term);
        }
        
        "mod" => {
            return Ok(term);
        }
        
        "pur" => {
            return Ok(term);
        }
        
        "sta" => {
            return Ok(term);
        }
        
        "sto" => {
            return Ok(term);
        }
        
        _ => {
            return Err("unknown command");
        }
    }
}

// build the time tracking string e.g. P191DT6H43M35S
pub fn make_timetracking_string(secs: i64) -> String {
    if secs < 1 {
        return "".to_string();
    }

    const DAY_SECS: i64    = 86_400;
    const HOUR_SECS: i64   =  3_600;
    const MINUTE_SECS: i64 =     60;

    let days:i64;
    let hours:i64;
    let minutes:i64;
    let mut ret:String = "P".to_string();
    let mut remainder:i64;


    // how many days
    days = secs / DAY_SECS;
    remainder = secs - (days * DAY_SECS);
    
    // how many hours
    hours = remainder / HOUR_SECS;
    remainder = remainder - ( hours * HOUR_SECS );
    
    // how many minutes
    minutes = remainder / MINUTE_SECS;
    remainder = remainder - ( minutes * MINUTE_SECS );

    // lets build
    match days {
        0 => {
            ret.push_str("T");
        }
        
        _ => {
            match remainder {
                0 => {
                    let temp = days.to_string() + "D";
                    ret.push_str(&temp);
                }

                _ => {
                    let temp = days.to_string() + "DT";
                    ret.push_str(&temp);
                }
            }
        }
    }
    
    match hours {
        0 => {
        }
        
        _ => {
            let temp = hours.to_string() + "H";
            ret.push_str(&temp);
        }
    }
    
    match minutes {
        0 => {
        }
        
        _ => {
            let temp = minutes.to_string() + "M";
            ret.push_str(&temp);
        }
    }
    
    // how many seconds = remainder
    match remainder {
        0 => {
        }
        
        _ => {
            let temp = remainder.to_string() + "S";
            ret.push_str(&temp);
        }
    }

    return ret;
}

// make timetracking timeframe
pub fn make_timetracking_timeframe(secs: i64) -> String {
    let mut ret = "".to_string();

    // 1 min
    if secs < 60 {
        ret = secs.to_string() + "s";
        return ret;
    }
    
    // 1 hour
    if secs < 3600 {
        let float = secs as f64 / 60 as f64;
        let ans = float.round() as i64;

        ret = ans.to_string() + "min";
        return ret;
    }
    
    // 1 day
    if secs < 86_400 {
        let float = secs as f64 / 3600 as f64;
        let ans = float.round() as i64;

        ret = ans.to_string() + "h";
        return ret;
    }
    
    // 2 weeks
    if secs < 1_209_600 {
        let float = secs as f64 / 86_400 as f64;
        let ans = float.round() as i64;

        ret = ans.to_string() + "d";
        return ret;
    }
    
    // 12 weeks
    if secs < 7_257_600 {
        let float = secs as f64 / 604_800 as f64;
        let ans = float.round() as i64;

        ret = ans.to_string() + "w";
        return ret;
    }
    
    // 12 months
    if secs < 31_536_000 {
        let float = secs as f64 / 2_592_000 as f64;
        let ans = float.round() as i64;
        
        ret = ans.to_string() + "mo";
        return ret;
    }
    
    // years
    let float = secs as f64 / 31_536_000 as f64;
    ret = format!("{:.1}{}",float,"y");

    return ret;
}


// Show the task given by integer id
pub fn report_single_id(){
    
}


// Show the task given by hexi uuiid
pub fn report_single_uuiid(){

}



// shorten vec from the front by ... 
pub fn shorten_front_of_vec_by_2<'a>(args: &'a Vec<String>) -> Result<Vec<&'a str>, &'static str> {

    let mut ret: Vec<&str> = Vec::new();
    let len:i32 = args.len() as i32;

    let can_do = len - 2;
    if can_do <= 0 {
        return Err("there are no arguments to act open");
    }

    for i in 0..args.len() {
        match i {
            0 | 1 => { 
                // do nothing
            }

            _     => {
                ret.push(&args[i]);
            }
        }
    }

    Ok(ret)
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
    fn t001_is_arg_integer() {
        let first = "23,67,0";
        let res = is_arg_integer(first);

        assert_eq!(res.unwrap().len(), 3);
    }
    
    
    // #[ignore]
    #[test]
    fn t002_determine_first_arg() {
        let mut comm = "".to_string();
        let mut vi: Vec<i64> = Vec::new();
        let mut vh: Vec<String> = Vec::new();
        let vs: Vec<String> = vec!["Nutting".to_string(), "23,43,0".to_string(),];
        let res = determine_first_arg(&vs, &mut vi, &mut vh, &mut comm);

        assert_eq!(res, ArgType::Integer);
    }


    // #[ignore]
    #[test]
    fn t003_determine_first_arg() {
        let mut comm = "".to_string();
        let mut vi: Vec<i64> = Vec::new();
        let mut vh: Vec<String> = Vec::new();
        let vs: Vec<String> = vec!["Nutting".to_string(), "23,".to_string(),];
        let res = determine_first_arg(&vs, &mut vi, &mut vh, &mut comm);

        assert_eq!(res, ArgType::Unknown);
    }

    // #[ignore]
    #[test]
    fn t004_determine_first_arg() {
        let mut comm = "".to_string();
        let mut vi: Vec<i64> = Vec::new();
        let mut vh: Vec<String> = Vec::new();
        let vs: Vec<String> = vec!["Nutting".to_string(), "23".to_string(),];
        let _res = determine_first_arg(&vs, &mut vi, &mut vh, &mut comm);

        assert_eq!(vi.len(), 1);
    }
    
    // #[ignore]
    #[test]
    fn t005_determine_hex() {
        let mut comm = "".to_string();
        let mut vi: Vec<i64> = Vec::new();
        let mut vh: Vec<String> = Vec::new();
        let vs: Vec<String> = vec!["Nutting".to_string(), "0x23,0x00f,0x01,0x1a".to_string(),];
        let _res = determine_first_arg(&vs, &mut vi, &mut vh, &mut comm);

        assert_eq!(vh.len(), 4);
    }
    
    // #[ignore]
    #[test]
    fn t006_determine_hex() {
        let vs: &str = "0x2g";
        let _res = is_arg_hexidecimal(&vs);
        
        assert_eq!(_res.is_ok(), false);
    }
    
    // #[ignore]
    #[test]
    fn t007_determine_comm() {
        let mut comm = "".to_string();
        let mut vi: Vec<i64> = Vec::new();
        let mut vh: Vec<String> = Vec::new();
        let vs: Vec<String> = vec!["Nutting".to_string(), "versio".to_string(),];
        let _res = determine_first_arg(&vs, &mut vi, &mut vh, &mut comm);
        assert_eq!(_res, ArgType::Unknown);

        let vs: Vec<String> = vec!["Nutting".to_string(), "ver".to_string(),];
        let _res = determine_first_arg(&vs, &mut vi, &mut vh, &mut comm);
        assert_eq!(_res, ArgType::Command);
    }
    
    // #[ignore]
    #[test]
    fn t008_hexi_verify() {
        let hexi = "0x0ff";
        let res = hexi_verify(hexi);
        assert_eq!(res.unwrap(), 255);
        
        let hexi2 = "0x0fgf";
        let res2 = hexi_verify(hexi2);
        let error = res2.err().unwrap();
        assert_eq!(error, "Not a hexidecimal string");
    }

    // #[ignore]
    #[test]
    fn t009_get_next_hexi() {
        let mut set  = Hdeci::new();
        set.add(1);
        set.add(2);
        set.add(4);
        let num = set.get_next_hexidecimal();
        assert_eq!(num,3);
        
        set.add(3);
        let num2 = set.get_next_hexidecimal();
        assert_eq!(num2,5);

        set.add(7);
        let num3 = set.get_next_hexidecimal();
        assert_eq!(num3,5);
        
        set.add(5);
        set.add(6);
        set.add(8);
        let num4 = set.get_next_hexidecimal();
        assert_eq!(num4,9);
    }

    // #[ignore]
    #[test]
    fn t010_copy_part_vector() {
        //want to only copy elements 2,3,4
        let data = vec![11, 22, 25, 44, 59, 67];
        let mut part = vec![0; 3];
        
        part.copy_from_slice(&data[1..4]);
        assert_eq!(part,vec![22,25,44]);
        
    }


    // #[ignore]
    #[test]
    fn t011_command_add() {
        let destination = "./test/trial.data";
        let mut pen = List::new(destination);
        let mut h_set:Hdeci = Hdeci::new();
        h_set.add(2);
        h_set.add(1);
        let next =  h_set.get_next_hexidecimal();
        
        let vs1: Vec<String> = vec!["Something".to_string(),
                                    "another".to_string()];
        let result_add1 = command_add_task(&vs1 ,&mut pen, &mut h_set);

        let vs2: Vec<String> = vec!["Something".to_string(),
                                    "another".to_string(),
                                    "First Task".to_string()];
        let result_add2 = command_add_task(&vs2 ,&mut pen, &mut h_set);
        
        let vs3: Vec<String> = vec!["Something".to_string(),
                                    "another".to_string(),
                                    "First Task".to_string(),
                                    "household".to_string()];
        let result_add3 = command_add_task(&vs3 ,&mut pen, &mut h_set);

        let vs4: Vec<String> = vec!["Something".to_string(),
                                    "another".to_string(),
                                    "First Task".to_string(),
                                    "start:noow".to_string(),
                                    "+household".to_string()];
        let result_add4 = command_add_task(&vs4 ,&mut pen, &mut h_set);

        let vs5: Vec<String> = vec!["Something".to_string(),
                                    "another".to_string(),
                                    "First Task".to_string(),
                                    "start:now".to_string(),
                                    "due:2030-01-05".to_string(),
                                    "+household".to_string()];
        let result_add5 = command_add_task(&vs5 ,&mut pen, &mut h_set);
        
        assert_eq!(result_add1.is_err(),true);
        assert_eq!(result_add2.is_err(),false);
        assert_eq!(result_add3.is_err(),true);
        assert_eq!(result_add4.is_err(),true);
        assert_eq!(result_add5.is_err(),false);
        
        pen.save();
        assert_eq!(pen.list.len(),2);
        remove_file(destination).expect("Cleanup test failed");
    }
    
    // #[ignore]
    #[test]
    fn t012_time_tracking_string() {
        let time_track = 3000 as i64;
        let str = make_timetracking_string(time_track);
        assert_eq!("PT50M",str);
        
        let time_track1 = 31_000_000 as i64;
        let str1 = make_timetracking_string(time_track1);
        assert_eq!("P358DT19H6M40S",str1);
        
        let time_track2 = 59 as i64;
        let str2 = make_timetracking_string(time_track2);
        assert_eq!("PT59S",str2);
        
        let time_track3 = 0 as i64;
        let str3 = make_timetracking_string(time_track3);
        assert_eq!("",str3);
        
        let time_track4 = 300 as i64;
        let str4 = make_timetracking_string(time_track4);
        assert_eq!("PT5M",str4);
        
        let time_track5 = 7200 as i64;
        let str5 = make_timetracking_string(time_track5);
        assert_eq!("PT2H",str5);
        
        let time_track6 = 1 as i64;
        let str6 = make_timetracking_string(time_track6);
        assert_eq!("PT1S",str6);
        
        let time_track7 = 172_800 as i64;
        let str7 = make_timetracking_string(time_track7);
        assert_eq!("P2D",str7);
    }
    
    // #[ignore]
    #[test]
    fn t013_time_tracking_timeframe() {
        let tt = 17 as i64;
        let tf = make_timetracking_timeframe(tt);
        assert_eq!("17s",tf);
        
        let tt = 61 as i64;
        let tf = make_timetracking_timeframe(tt);
        assert_eq!("1min",tf);
        
        let tt = 3580 as i64;
        let tf = make_timetracking_timeframe(tt);
        assert_eq!("60min",tf);
        
        let tt = 8900 as i64;
        let tf = make_timetracking_timeframe(tt);
        assert_eq!("2h",tf);
        
        let tt = 85_200 as i64;
        let tf = make_timetracking_timeframe(tt);
        assert_eq!("24h",tf);
        
        let tt = 1_209_000 as i64;
        let tf = make_timetracking_timeframe(tt);
        assert_eq!("14d",tf);
        
        let tt = 7_257_000 as i64;
        let tf = make_timetracking_timeframe(tt);
        assert_eq!("12w",tf);
        
        let tt = 40_209_001 as i64;
        let tf = make_timetracking_timeframe(tt);
        assert_eq!("1.3y",tf);
        
        let tt = 235_752_000 as i64;
        let tf = make_timetracking_timeframe(tt);
        assert_eq!("7.5y",tf);

    }
    
    // #[ignore]
    #[test]
    fn t014_get_report_single() {
        let colors = Colors::new();
        let settings = SettingsMap::new();
        let mut hd_set: Hdeci         = Hdeci::new();
        let dest1 = "./test/pending.data";
        let dest2 = "./test/settings.txt";
        let mut pen = List::new(dest1);
        let _res_sett = settings.save(dest2);

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
        pen.list.clear();
        
        let res_load = load_task_file(dest1, &mut pen, &mut hd_set);
        remove_file(dest1).expect("Cleanup test failed");
        remove_file(dest2).expect("Cleanup test failed");


        let result = get_integer_single_report(&settings, colors, 1, &pen);




        assert_eq!(result.is_err(), false);
    }
















} //end of tests