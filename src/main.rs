/*
    This is my to-do list
    2020.07.24      Sven Ponelat

*/

mod library;

use library::my_utils::*;
use library::functions::*;
use library::enums::*;
use library::settings::*;
use library::structs::*;
use library::task::*;
use library::list::*;
use substring::Substring;
use std::collections::{BTreeMap, BTreeSet};
use std::future::pending;
use std::process::exit;
use std::fs::copy;
use std::path::Path;
use std::env;
use termion::{color, style};
use thousands::{Separable};
use std::time::{SystemTime};


pub const VERSION: &str         = env!("CARGO_PKG_VERSION");
pub const PENDING: &str         = "./test/working/pending.data";
pub const COMPLETED: &str       = "./test/working/completed.data";
pub const SETTINGS_FILE: &str   = "settings.txt";



#[rustfmt::skip]
fn main() {
    let now = SystemTime::now();
    let arguments: Vec<String> = env::args().collect();
    let mut command: String = "".to_string();
    let mut arg_id:   Vec<i64> = Vec::new();
    let mut arg_hex:  Vec<String> = Vec::new();
    // let mut first_arg = None;
    // let mut sub1 = None;
    // let mut sub2 = None;
    // let mut sub3 = None;
    // let mut sub4 = None;
    // let mut sub5 = None;

    let settings = load_settings(SETTINGS_FILE);
    let data_dir = settings.map.get("dataDir").unwrap().to_string();
    let pending_file = data_dir.clone() + "/pending.data";
    let completed_file = data_dir + "/completed.data";
    let mut pending_tasks:List    = List::new(&pending_file);
    let mut completed_tasks:List  = List::new(&completed_file);
    let mut hd_set: Hdeci         = Hdeci::new();

    load_all_tasks( &pending_file,&completed_file, &mut pending_tasks, &mut completed_tasks, &mut hd_set);

    // let mut next_hexi = get_next_hexidecimal(hexi_set);

    // let next_id: i64 = 1;
    // let next_uuiid_int: i64 = 1;

    



// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Arguments @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

    let matcho: ArgType = determine_first_arg(&arguments, &mut arg_id, &mut arg_hex, &mut command);

    // lets do it
    match matcho {
        ArgType::None => {
            println!("No arguments")
        }
        
        ArgType::Integer => {
            let matcho2: ArgType = determine_second_arg(&arguments, &mut command);

            match matcho2 {
                ArgType::None => {
                    println!("No secondary arguments")
                }

                ArgType::Command => {
                    let term = &command;
                    
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
                        // should never get here
                        println!("Should never get here -> (3rd case)" );
                        }
                    }

                }

                ArgType::Unknown => {
                    println!("Unknown secondary command")
                }

                _ => {
                    // should never get here
                    println!("Should never get here -> (2nd case)" );
                }
            }









            println!("Integer arguments")
        }
        
        ArgType::Hexidecimal => {
            println!("Hexidecimal arguments")
        }
        
        ArgType::Command => {
            match command.as_str() {
                "add" => {
                    let result_add = command_add_task(&arguments ,&mut pending_tasks, &mut hd_set);
                    if result_add.is_err(){
                        let message = result_add.err().unwrap().to_string();
                        feedback(Feedback::Error, message);
                        exit(17);
                    }
                    println!("Created task {}",result_add.unwrap());
                    // for i in hd_set.set {
                    //     println!(" {}",hexidecimal_to_string(i));
                    // }

                }

                "mycompleted" => {

                }
                
                "version" => {
                    println!("Version is {}",VERSION);
                }

                _ => { 
                    // should never get here
                    println!("Should never get here -> command unknown")
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
