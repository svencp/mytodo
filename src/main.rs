/*
    This is my to-do list
    2020.07.24      Sven Ponelat

*/

mod library;

use library::my_utils::*;
use library::functions::*;
use library::enums::*;
use library::settings::*;
use library::task::*;
use library::list::*;
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
const SETTINGS_FILE: &str = "settings.txt";



#[rustfmt::skip]
fn main() {
    let now = SystemTime::now();
    let arguments: Vec<String> = env::args().collect();
    let mut command: String = "".to_string();
    let mut arg_id:   Vec<i64> = Vec::new();
    let mut arg_hex:  Vec<String> = Vec::new();
    let mut first_arg = None;
    let mut sub1 = None;
    let mut sub2 = None;
    let mut sub3 = None;
    let mut sub4 = None;
    let mut sub5 = None;

    let settings = load_settings(SETTINGS_FILE);
    let data_dir = settings.map.get("dataDir").unwrap().to_string();
    let pending_file = data_dir.clone() + "/pending.data";
    let completed_file = data_dir + "/completed.data";
    let mut completed_tasks = List::new();
    let mut pending_tasks = List::new();

    let next_id: i64 = 1;
    let next_uuiid_int: i64 = 1;

    


    // It seems I need to do this,otherwise temporary variables get dropped
    match arguments.len() {
        2 => {
            first_arg = Some(arguments[1].to_lowercase().trim().to_owned());
        },
        3 => {
            first_arg = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
        },
        4 => {
            first_arg = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
        }
        5 => {
            first_arg = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
            sub3 = Some(arguments[4].trim().to_owned());
        },
        6 => {
            first_arg = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
            sub3 = Some(arguments[4].trim().to_owned());
            sub4 = Some(arguments[5].trim().to_owned());
        },
        7 => {
            first_arg = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
            sub3 = Some(arguments[4].trim().to_owned());
            sub4 = Some(arguments[5].trim().to_owned());
            sub5 = Some(arguments[6].trim().to_owned());
        },

        _ => { () }

    }// end of match




// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Arguments @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

    let matcho: ArgType = determine_arg(&arguments, &mut arg_id, &mut arg_hex, &mut command);

    // lets do it
    match matcho {
        ArgType::None => {
            println!("No arguments")
        }
        
        ArgType::Integer => {
            println!("Integer arguments")
        }
        
        ArgType::Hexidecimal => {
            println!("Hexidecimal arguments")
        }
        
        ArgType::Command => {
            match command.as_str() {
                "add" => {
                    let result = make_task(&arguments, next_uuiid_int, next_id);
                    if result.is_err() {
                        let message = result.err().unwrap().to_string();
                        feedback(Feedback::Error, message);
                        exit(17);
                    }
                    pending_tasks.list.push(result.unwrap());
                    let res_save = pending_tasks.save(&pending_file);
                }

                "mycompleted" => {

                }

                _ => { 
                    // should never get here
                    println!("Sould never get here -> command unknown")
                }
            }
        }



        _ => {
            println!("What happened here!")
        }

    }//end of match



















    println!("Hello Svenny!");
    show_response(now)



} // End of Main
