
/*
        Module for everything all main functions
        2022.07.31      Sven Ponelat

*/


use substring::Substring;
use std::collections::BTreeSet;
use super::enums::ArgType;
use crate::library::task::*;
use crate::library::list::*;
use crate::library::lts::*;
use crate::library::structs::*;



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

        // let sub2 = "0x";
        // if hexi.substring(0, 2) != sub2 {
        //     return Err("Does not start with 0x");
        // }
        // let n_hexi = hexi.trim_start_matches(sub2);
        // let res_int = i64::from_str_radix(n_hexi, 16);
        // if res_int.is_err() {
        //     return Err("Not a hexidecimal");
        // }
        // ret.push(hexi.to_lowercase().trim().to_string());
    }

    Ok(ret)
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

        _ => {
            return Err("unknown command");
        }
    }
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

// Show the task given by integer id
pub fn report_single_id(){

}


// Show the task given by hexi uuiid
pub fn report_single_uuiid(){

}


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
    let save = pending.save();
    if save.is_err(){
        let message = save.err().unwrap();
        return Err(message);
    }

    Ok(pending.list.len() as i64)
}

// //find next available hexi number
// pub fn get_next_hexidecimal(set: BTreeSet<i64>) -> i64 {
//     let mut index = 0;
//     let mut found = false;

//     for _i in 0..set.len() {
//         index += 1;
//         if ! set.contains(&index){
//             found = true;
//             break;
//         } 
//     }

//     if ! found {
//         let ret = index + 1;
//         return ret;
//     }

//     return index;
// }

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
        
        let save = pen.save();
        assert_eq!(save.unwrap(),2);
        remove_file(destination).expect("Cleanup test failed");
    }



















} //end of tests