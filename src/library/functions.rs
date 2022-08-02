use inflections::Inflect;
use substring::Substring;

use super::enums::ArgType;

/*
    Module for everything all main functions
    2022.07.31      Sven Ponelat

*/


use crate::library::enums::*;




// determine the first argument type
pub fn determine_arg(args: &Vec<String>, v_int: &mut Vec<i64>, v_hex: &mut Vec<String>, command: &mut String) -> ArgType {

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
        let sub2 = "0x";
        if hexi.substring(0, 2) != sub2 {
            return Err("Does not start with 0x");
        }
        let n_hexi = hexi.trim_start_matches(sub2);
        let res_int = i64::from_str_radix(n_hexi, 16);
        if res_int.is_err() {
            return Err("Not a hexidecimal");
        }
        ret.push(hexi.to_lowercase().trim().to_string());
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


// Show the task given by integer id
pub fn report_single_id(){

}


// Show the task given by hexi uuiid
pub fn report_single_uuiid(){

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
    fn t002_determine_arg() {
        let mut comm = "".to_string();
        let mut vi: Vec<i64> = Vec::new();
        let mut vh: Vec<String> = Vec::new();
        let vs: Vec<String> = vec!["Nutting".to_string(), "23,43,0".to_string(),];
        let res = determine_arg(&vs, &mut vi, &mut vh, &mut comm);

        assert_eq!(res, ArgType::Integer);
    }


    // #[ignore]
    #[test]
    fn t003_determine_arg() {
        let mut comm = "".to_string();
        let mut vi: Vec<i64> = Vec::new();
        let mut vh: Vec<String> = Vec::new();
        let vs: Vec<String> = vec!["Nutting".to_string(), "23,".to_string(),];
        let res = determine_arg(&vs, &mut vi, &mut vh, &mut comm);

        assert_eq!(res, ArgType::Unknown);
    }

    // #[ignore]
    #[test]
    fn t004_determine_arg() {
        let mut comm = "".to_string();
        let mut vi: Vec<i64> = Vec::new();
        let mut vh: Vec<String> = Vec::new();
        let vs: Vec<String> = vec!["Nutting".to_string(), "23".to_string(),];
        let _res = determine_arg(&vs, &mut vi, &mut vh, &mut comm);

        assert_eq!(vi.len(), 1);
    }
    
    // #[ignore]
    #[test]
    fn t005_determine_hex() {
        let mut comm = "".to_string();
        let mut vi: Vec<i64> = Vec::new();
        let mut vh: Vec<String> = Vec::new();
        let vs: Vec<String> = vec!["Nutting".to_string(), "0x23,0x00f,0x01,0x1a".to_string(),];
        let _res = determine_arg(&vs, &mut vi, &mut vh, &mut comm);

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
        let _res = determine_arg(&vs, &mut vi, &mut vh, &mut comm);
        assert_eq!(_res, ArgType::Unknown);

        let vs: Vec<String> = vec!["Nutting".to_string(), "ver".to_string(),];
        let _res = determine_arg(&vs, &mut vi, &mut vh, &mut comm);
        assert_eq!(_res, ArgType::Command);
    }
    


























} //end of tests