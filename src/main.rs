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
use library::reports::*;
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

    let settings = load_settings(SETTINGS_FILE);
    let terminal_width = get_terminal_width(&settings);
    let colors = load_colors(&settings);
    let data_dir = settings.map.get("dataDir").unwrap().to_string();

    let pending_file = data_dir.clone() + "/pending.data";
    let completed_file = data_dir + "/completed.data";

    let mut pending_tasks:List    = List::new(&pending_file);
    let mut completed_tasks:List  = List::new(&completed_file);
    let mut all_tasks:List        = List::new_no_file();
    let mut hd_set: Hdeci         = Hdeci::new();
    
    load_all_tasks( &pending_file,&completed_file, &mut pending_tasks, &mut completed_tasks, &mut hd_set);
    all_tasks.append(pending_tasks.clone());
    all_tasks.append(completed_tasks.clone());
    // let all_tasks= &mut completed_tasks.list;
    // all_tasks.append(&mut pending_tasks.clone().list);
    // all_tasks.sort();

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
        
        ArgType::Integer | ArgType::Hexidecimal => {
            let matcho2: ArgType = determine_second_arg(&arguments, &mut command);

            match matcho2 {
                
                ArgType::Command => {
                    let term = command.as_str();
                    
                    match term {
                        "ann" => {

                            let result = command_add_annotation(&arguments, &arg_id, &arg_hex,
                                                        &mut pending_tasks, &mut completed_tasks, &all_tasks);
                            if result.is_err() {
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Error, message);
                                exit(17); 
                            }

                            println!("{}",term);
                        } // end of ann
                        
                        "del" => {
                            println!("{}",term);
                        }
                        
                        "den" => {
                            println!("{}",term);
                        }
                        
                        // done
                        "don" => {
                            let result = command_done(&colors,arg_id, 
                                                            &mut pending_tasks, &mut completed_tasks);
                            if result.is_err(){
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Error, message);
                                exit(17);
                            }
                            // let size = result.unwrap() as usize;
                            pending_tasks.save();
                            completed_tasks.save();
                            // if save1.is_err() || save2.is_err() {
                            //     let message = "Problems saving data files".to_string();
                            //     feedback(Feedback::Error, message);
                            //     exit(17);
                            // }
                            show_nag(&settings,colors);
                        }
                        
                        "dup" => {
                            println!("{}",term);
                        }
                        
                        "hel" => {
                            println!("{}",term);
                        }
                        
                        "mod" => {
                            println!("{}",term);
                        }
                        
                        "pur" => {
                            println!("{}",term);
                        }

                        // start
                        "sta" => {
                            let result = command_start(arg_id, &mut pending_tasks);
                            if result.is_err(){
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Error, message);
                                exit(17);
                            }
                            let size = result.unwrap() as usize;
                            pending_tasks.save();
                            // if save1.is_err() {
                            //     let message = "Problems saving pending data files".to_string();
                            //     feedback(Feedback::Error, message);
                            //     exit(17);
                            // }
                            println!("Started {} {}.",size, units("task",size));
                        }
                        
                        "sto" => {
                            let result = command_stop(arg_id, &mut pending_tasks, &settings);
                            if result.is_err(){
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Error, message);
                                exit(17);
                            }
                            pending_tasks.save();
                            // if save1.is_err() {
                            //     let message = "Problems saving pending data files".to_string();
                            //     feedback(Feedback::Error, message);
                            //     exit(17);
                            // }

                        }
                        
                        _ => {
                            // should never get here
                            println!("Should never get here -> (3rd case)" );
                        }
                    }
                    
                } // end of ArgType::Command

                ArgType::None => {

                    match matcho {
                        ArgType::Hexidecimal => {
                            if arg_hex.len() != 1 {
                                let message = "No match -> too many tasks".to_string();
                                feedback(Feedback::Warning, message);
                                exit(17);
                            }
                            let uuiid = arg_hex.clone().get(0).unwrap().to_string();
                            let res_task = all_tasks.clone().get_task_from_uuiid(uuiid);
                            if res_task.is_err() {
                                let message = res_task.err().unwrap().to_string();
                                feedback(Feedback::Warning, message);
                                exit(17);
                            }
                            let uuiid_int = res_task.unwrap().uuiid_int;
                            let result = get_integer_single_report(&settings, colors, uuiid_int, &all_tasks);
                            if result.is_err() {
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Warning, message);
                                exit(17);
                            }
                            
                        }
                        ArgType::Integer => {
                            if arg_id.len() != 1 {
                                let message = "No match -> too many tasks".to_string();
                                feedback(Feedback::Warning, message);
                                exit(17);
                            }

                            let id = *arg_id.get(0).unwrap();
                            let res_task = pending_tasks.get_task_from_id(id);
                            if res_task.is_err() {
                                let message = res_task.err().unwrap().to_string();
                                feedback(Feedback::Warning, message);
                                exit(17);
                            }
                            let uuiid_int = res_task.unwrap().uuiid_int;
                            let result = get_integer_single_report(&settings, colors, uuiid_int, &all_tasks);
                            if result.is_err() {
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Warning, message);
                                exit(17);
                            }

                        }
                        _ => {

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
        
        // ArgType::Hexidecimal => {
        //     println!("Hexidecimal arguments")
        // }
        
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

                "colortest" => {
                    color_test(colors);
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



















    // println!("Hello Svenny!");
    // let show = settings.clone().get_bool("showResponseTimes");
    let show = settings.get_bool("showResponseTimes");
    if show.unwrap(){
        show_response(now)
    }



} // End of Main
