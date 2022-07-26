/*
    This is my to-do list
    2022.07.24      Sven Ponelat
                    Use the shortcut 'so' to compile and move release executable to its place

    2022-08-29  1.2.0   adding the search function -> report
    2022-08-31  1.2.2   adding the purge command    1) eg.  m 2,3 purge
                                                    2) eg.  m purge
    2022-09-06  1.2.6   adding status column to completed report   
    2022-09-06  1.2.7   make ID 3 chars long in reports
    2022-09-13  1.2.9   denotate
    2022-09-26  1.2.10  recurring chained too long, period should count from last task end time
    2022-10-04  1.2.11  change the sort order on the task waiting screen - soonest at the bottom
    2022-10-16  1.3.0   introduce shortcuts to add command { '-'  ->  currently only due:now start:now }
    2022-11-15  1.3.1   change the sort order on the All Pending report, with longest past due on the bottom
    2022-12-01  1.3.2   make searching case insensitive by making both (term and search) lowercase
                        removed lts module and use the local_timestamps crate instead
                        removed my_utils module and use the error_feedback crate instead
    




*/

mod library;

use error_feedback::*;
// use library::my_utils::*;
use library::functions::*;
use library::enums::*;
use library::settings::*;
use library::structs::*;
use library::reports::*;
use library::list::*;
use std::process::exit;
use std::env;
use std::time::{SystemTime};


pub const RELEASE: bool            = false;
// pub const RELEASE: bool            = true;
pub const VERSION: &str            = env!("CARGO_PKG_VERSION");





#[rustfmt::skip]
fn main() {
    let now = SystemTime::now();
    let arguments: Vec<String>    = env::args().collect();
    let mut command: String       = "".to_string();
    let mut arg_id:   Vec<i64>    = Vec::new();
    let mut arg_hex:  Vec<String> = Vec::new();
    
    let res_data_dir = create_data_dirs();
    if res_data_dir.is_err(){
        let message = res_data_dir.err().unwrap().to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }
    let settings_file = res_data_dir.clone().unwrap().get(0).unwrap().to_string();
    let pending_file = res_data_dir.clone().unwrap().get(1).unwrap().to_string();
    let completed_file = res_data_dir.clone().unwrap().get(2).unwrap().to_string();

    let settings = load_settings(&settings_file);
    let colors = load_colors(&settings);

    let mut pending_tasks:List    = List::new(&pending_file);
    let mut completed_tasks:List  = List::new(&completed_file);
    let mut all_tasks:List        = List::new_no_file();
    let mut hd_set: Hdeci         = Hdeci::new();
    
    // load_all_tasks( &pending_file,&completed_file, &mut pending_tasks, &mut completed_tasks, &mut hd_set);
    load_all_tasks( &mut pending_tasks, &mut completed_tasks, &mut hd_set);
    all_tasks.append(completed_tasks.clone());
    all_tasks.append(pending_tasks.clone());

    // generate recurring tasks
    generate_recurring_tasks(&mut pending_tasks, &mut completed_tasks, &mut hd_set);
    handle_orphans(&mut pending_tasks);
    



// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Arguments @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

    let matcho: ArgType = determine_first_arg(&arguments, &mut arg_id, &mut arg_hex, &mut command);
    
    // lets do it
    match matcho {
        ArgType::None => {
            let result = report_all_pending(&pending_tasks, colors, &settings);
            if result.is_err() {
                let message = result.err().unwrap().to_string();
                feedback(Feedback::Info, message);
            }
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
                        } // end of ann
                        
                        "del" => {
                            let result = command_delete( &arg_id, &arg_hex, 
                                                    &mut pending_tasks, &mut completed_tasks);
                            if result.is_err(){
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Error, message);
                                exit(17);
                            }
                        }
                        
                        "den" => {
                            let result = command_den(&arguments, &arg_id, &arg_hex, 
                                                &mut pending_tasks, &mut completed_tasks, &all_tasks);
                            if result.is_err() {
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Error, message);
                                exit(17); 
                            }
                        }
                        
                        // done
                        "don" => {
                            let result = command_done(arg_id, &mut pending_tasks, &mut completed_tasks, &settings);
                            if result.is_err(){
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Error, message);
                                exit(17);
                            }
                        }
                        
                        "dup" => {
                            println!("{}",term);
                        }
                        
                        "hel" => {
                            println!("{}",term);
                        }
                        
                        "mod" => {
                            let result = command_modification(&arguments, &arg_id, 
                                                    &arg_hex, &mut pending_tasks, &mut completed_tasks);
                            if result.is_err(){
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Error, message);
                            }
                        }
                        
                        "pur" => {
                            let result = command_purge(&arg_id, &arg_hex, 
                                                    &mut pending_tasks, &mut completed_tasks);
                            if result.is_err(){
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Error, message);
                            }
                        }

                        // start
                        "sta" => {
                            let result = command_start( &arg_id, &arg_hex,
                                                        &mut pending_tasks, &mut completed_tasks);
                            if result.is_err(){
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Error, message);
                                exit(17);
                            }
                        }
                        
                        // stop
                        "sto" => {
                            let result = command_stop( &arg_id, &arg_hex,
                                        &mut pending_tasks, &all_tasks);
                            if result.is_err(){
                                let message = result.err().unwrap().to_string();
                                feedback(Feedback::Error, message);
                                exit(17);
                            }
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


            // println!("Integer arguments")
        }
        
        
        ArgType::Command => {
            match command.as_str() {
                "active" => {
                    let result = report_active(&colors, &settings, &pending_tasks);
                    if result.is_err() {
                        let message = result.err().unwrap().to_string();
                        feedback(Feedback::Warning, message);
                    }
                }

                "add" => {
                    let result_add = command_add_task(&arguments ,&mut pending_tasks, &mut hd_set);
                    if result_add.is_err(){
                        let message = result_add.err().unwrap().to_string();
                        feedback(Feedback::Error, message);
                        exit(17);
                    }
                    println!("Created task {}",result_add.unwrap());
                }

                "all" => {
                    let result = report_search(&arguments, &colors, &settings, &all_tasks);
                    if result.is_err() {
                        let message = result.err().unwrap().to_string();
                        feedback(Feedback::Warning, message);
                    }
                }

                "colortest" => {
                    color_test(colors);
                }

                "completed" => {
                    let result = report_completed(&colors, &settings, &completed_tasks);
                    if result.is_err() {
                        let message = result.err().unwrap().to_string();
                        feedback(Feedback::Warning, message);
                    }
                }

                "purge" => {
                    let result = purge_deleted(&mut completed_tasks);
                    if result.is_err() {
                        let message = result.err().unwrap().to_string();
                        feedback(Feedback::Warning, message);
                    }                   
                }
                
                "recurring" => {
                    let result = report_recurring(&colors, &settings, &pending_tasks);
                    if result.is_err() {
                        let message = result.err().unwrap().to_string();
                        feedback(Feedback::Warning, message);
                    }
                }

                "version" => {
                    println!("Version is {}",VERSION);
                }

                "waiting" => {
                    let result = report_waiting(&colors, &settings, &pending_tasks);
                    if result.is_err() {
                        let message = result.err().unwrap().to_string();
                        feedback(Feedback::Warning, message);
                    }
                }

                _ => { 
                    // should never get here
                    println!("Should never get here -> command unknown")
                }
            }
        }

        _ => {
            println!("Garbage in -> garbage out!")
        }

    }//end of match





    let show = settings.get_bool("showResponseTimes");
    if show.unwrap(){
        show_response(now)
    }



} // End of Main
