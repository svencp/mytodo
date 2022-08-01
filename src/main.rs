/*
    This is my to-do list
    2020.07.24      Sven Ponelat

*/

mod library;

use library::my_utils::*;
use library::functions::*;
use library::enums::*;
use library::task::*;
use substring::Substring;
use std::collections::{BTreeMap};
use std::process::exit;
use std::fs::copy;
use std::path::Path;
use std::env;
use termion::{color, style};
use thousands::{Separable};
use std::time::{SystemTime};


const VERSION: &str   = env!("CARGO_PKG_VERSION");
const PENDING: &str   = "./pending.data";
const COMPLETED: &str = "./completed.data";


#[rustfmt::skip]
fn main() {
    let now = SystemTime::now();
    let arguments: Vec<String> = env::args().collect();
    let mut arg_id:   Vec<i64> = Vec::new();
    let mut arg_hex:  Vec<i64> = Vec::new();
    let mut command = None;
    let mut sub1 = None;
    let mut sub2 = None;
    let mut sub3 = None;
    let mut sub4 = None;
    let mut sub5 = None;

    // It seems I need to do this,otherwise temporary variables get dropped
    match arguments.len() {
        2 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
        },
        3 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
        },
        4 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
        }
        5 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
            sub3 = Some(arguments[4].trim().to_owned());
        },
        6 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
            sub3 = Some(arguments[4].trim().to_owned());
            sub4 = Some(arguments[5].trim().to_owned());
        },
        7 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
            sub3 = Some(arguments[4].trim().to_owned());
            sub4 = Some(arguments[5].trim().to_owned());
            sub5 = Some(arguments[6].trim().to_owned());
        },

        _ => { () }

    }// end of match




// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Arguments @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

    // There are no arguments
    if arguments.len() < 2 {
        let message = format!("There are zero arguments");
        feedback(Feedback::Info, message);

    // Too many arguments
    } else if arguments.len() >= 6 {
        let message = format!("There are too many arguments.");
        feedback(Feedback::Warning, message);
    
    
    //majority of arguments    
    } else {

    // What we have to figure out is, if the first arg is an integer, hex or command
        let mut matcho: &str;
    
        let res_int = is_arg_integer(command.clone().unwrap().as_str());
        if res_int.is_ok() {
            arg_id = res_int.unwrap();
            matcho = "int";
        }


        let res_hexi = is_arg_hexidecimal(command.clone().unwrap().as_str());
        if res_hexi.is_ok() {
            arg_hex = res_hexi.unwrap();
            matcho = "hex";
        }


        // let temp_string = command.clone().unwrap().su
        // let comm3 = temp_string


        match comm3 {
            
            "add"  => {
                let res_add = add_task(arguments.clone());     

            }// end of "add"
            
            
            
        
            
            
            
            


            // Not a valid first argument 
            _   => {
                let message = format!("Not a valid first argument ->  {}",arguments[1]);
                feedback(Feedback::Warning, message);
            } //end of _   







        } //  end of match command 













    } // end of else   if arguments.len() 




















    println!("Hello Svenny!");
    show_response(now)



} // End of Main
