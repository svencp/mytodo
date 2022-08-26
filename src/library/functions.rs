
/*
        Module for everything all main functions
        2022.07.31      Sven Ponelat

*/


use substring::Substring;
use std::path::Path;
use std::process::exit;
use std::io;
use std::io::Write;
use std::str::FromStr;
use std::env;
use std::fs;
use crate::library::task::*;
use crate::library::list::*;
use crate::library::lts::*;
use crate::library::structs::*;
use crate::library::my_utils::*;
use crate::library::settings::*;
use crate::library::enums::*;
use crate::library::reports::*;


// align the timeframe like ' 1.5y  '   and       '   7min'
pub fn align_timeframe(secs: i64) -> String {
    let tf = &make_timetracking_timeframe(secs);

    // currently only looking for strings of length 7
    let padded = justify(tf.to_string(), 7, Justify::Right);
    
    if  tf.contains("h") ||
        tf.contains("w") ||
        tf.contains("s") ||
        tf.contains("d") ||
        tf.contains("y") {
            let mut shortened = padded[2..].to_string();
            shortened.push_str("  ");
            return shortened
        }
        
    if tf.contains("mo") {
        let mut shortened = padded[1..].to_string();
        shortened.push_str(" ");
        return shortened
    }
    return padded.to_string()
}

pub fn categorize_term(secs: i64) -> String {
    let ret:String;

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

    Ok(pending.list.len() as i64)
}

// function to add annotations
pub fn command_add_annotation(args: &Vec<String>, v_int: &Vec<i64>, v_hex: &Vec<String>,
                                pend: &mut List, comp: &mut List, allt: &List) -> Result<(), &'static str> {
    if args.len() < 4 {
        let message = "No annotation(s) given.".to_string();
        feedback(Feedback::Error, message);
        exit(17); 
    }
    let annotation = &args[3];
    let mut v_uuiid_int:Vec<i64> = Vec::new();

    match v_int.len() {
        0 => {
            // @@@@@@@@@@@@@@@@@@@@ HEXIDECIMAL @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            for hexi in v_hex {
                let uuiid_int = allt.get_task_from_uuiid(hexi.clone());
                if uuiid_int.is_err() {
                    let message = uuiid_int.err().unwrap().to_string();
                    feedback(Feedback::Error, message);
                    exit(17);
                }
                v_uuiid_int.push(uuiid_int.unwrap().uuiid_int);
            }
            println!("This command will alter {} {}.",v_uuiid_int.len(), units("task", v_uuiid_int.len()));
            
            for int in v_uuiid_int.clone() {
                let found = comp.get_index_of_task_with_uuiid_int(int);
                if found == -1 {
                    return Err("Task does not exist in hexidecimal set.");
                }
                let u = found as usize;
                let task = &mut comp.list.remove(u);
                let mut anno = Annotation::new();
                anno.date = lts_now();
                anno.desc = annotation.to_string();
                task.ann.push(anno);
                
                comp.list.insert(u, task.clone());
                println!("Annotating task {} '{}'.",task.uuiid, task.description);
            }
            
            comp.save();
        }
        _ => {
            // @@@@@@@@@@@@@@@@@@@@ INTEGERS @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
            for inty in v_int {
                let uuiid_int = allt.get_task_from_id(inty.clone());
                if uuiid_int.is_err() {
                    let message = uuiid_int.err().unwrap().to_string();
                    feedback(Feedback::Error, message);
                    exit(17);
                }
                v_uuiid_int.push(uuiid_int.unwrap().uuiid_int);
            }
            println!("This command will alter {} {}.",v_uuiid_int.len(), units("task", v_uuiid_int.len()));
            
            for int in v_uuiid_int.clone() {
                let found = pend.get_index_of_task_with_uuiid_int(int);
                if found == -1 {
                    return Err("Task does not exist in hexidecimal set.");
                }
                let u = found as usize;
                let task = &mut pend.list.remove(u);
                let mut anno = Annotation::new();
                anno.date = lts_now();
                anno.desc = annotation.to_string();
                task.ann.push(anno);
                
                pend.list.insert(u, task.clone());
                println!("Annotating task {} '{}'.",task.uuiid, task.description);
            }
            pend.save();
            
        } // end of _
    } // end of match
    
    println!("Annotated {} {}.", v_uuiid_int.clone().len(), units("task", v_uuiid_int.clone().len()));

    Ok(())
}

// function to delete tasks
pub fn command_delete(v_int: &Vec<i64>, v_hex: &Vec<String>,
                        pend: &mut List, comp: &mut List) -> Result<(), &'static str> {
    let mut v_tasks: Vec<Task> = Vec::new();
    let mut done = false;
    let mut del_counter = 0;

    match v_hex.len() {
        0 => {
            // lets see if they are all valid
            for id in v_int.clone() {
                let task = pend.get_task_from_id(id);
                if task.is_err() {
                    return Err("Invalid task id's given.")
                }
                v_tasks.push(task.unwrap());
            }
            
            match v_tasks.clone().len() {
                1 => {
                    let task = v_tasks.pop().unwrap();
                    while !done {
                        let out_text = format!("Delete task {} '{}'? (yes/no) ", task.clone().id.unwrap(), task.clone().description);
                        let res_reply = get_input(&out_text);
                        if res_reply.is_err() {
                            return Err(res_reply.err().unwrap())
                        }
                        let reply = res_reply.unwrap().to_lowercase();
                        if reply.starts_with("y") || reply.starts_with("n") {
                            match reply.substring(0, 1) {
                                "y" => {
                                    let res_del = delete_task(pend,comp,task.clone());
                                    if res_del.is_err(){
                                        return Err(res_del.err().unwrap())
                                    }
                                }
                                _ => {
                                    println!("Task not deleted.");
                                    println!("Deleted 0 tasks.");
                                }
                            }
                            done = true;
                        }
            
                    }
                }
                _ => {
                    let len = v_tasks.clone().len();

                    while !done {
                        for t in 0..len {
                            let task = v_tasks.get(t).unwrap();

                            let out_text = format!("Delete task {} '{}'? (yes/no/all/quit) ", 
                                                task.clone().id.unwrap(), task.clone().description);
                            let res_reply = get_input(&out_text);
                            if res_reply.is_err() {
                                return Err(res_reply.err().unwrap())
                            }
                            let reply = res_reply.unwrap().to_lowercase();
                            if  reply.starts_with("y") || reply.starts_with("n") ||
                                reply.starts_with("a") || reply.starts_with("q") {
                                match reply.substring(0, 1) {
                                    "y" => {
                                        let res_del = delete_task(pend,comp,task.clone());
                                        if res_del.is_err(){
                                            return Err(res_del.err().unwrap())
                                        }
                                        del_counter += 1;

                                        if t + 1 < len {
                                            println!();
                                            continue;
                                        }
                                        // done = true;
                                    }
                                    "n" => {
                                        println!("Task not deleted.");

                                        if t + 1 < len {
                                            println!();
                                            continue;
                                        }
                                        // done = true;
                                    }
                                    "a" => {
                                        for task in v_tasks.clone() {
                                            let res_del = delete_task(pend,comp,task.clone());
                                            if res_del.is_err(){
                                                return Err(res_del.err().unwrap());
                                            }
                                            del_counter += 1;
                                        }
                                        done = true;
                                        break;
                                    }
                                    "q" => {
                                        println!("Task not deleted.");
                                        done = true;
                                        break;
                                    }
                                    _ => {
                                    }
                                }
                                done = true;
                            }
                        }
                    }
                    println!("Deleted {} {}",del_counter.to_string(), units("task", del_counter));
                }
            }
        }
        // hex values
        _ => {
            // lets see if they are all valid
            for uuiid in v_hex.clone() {
                let task = comp.get_task_from_uuiid(uuiid);
                if task.is_err() {
                    return Err("Invalid task id's given.")
                }
                v_tasks.push(task.unwrap());
            }

            match v_tasks.clone().len() {
                1 => {
                    let task = v_tasks.pop().unwrap();
                    while !done {
                        let out_text = format!("Delete task {} '{}'? (yes/no) ", task.clone().uuiid, task.clone().description);
                        let res_reply = get_input(&out_text);
                        if res_reply.is_err() {
                            return Err(res_reply.err().unwrap())
                        }
                        let reply = res_reply.unwrap().to_lowercase();
                        if reply.starts_with("y") || reply.starts_with("n") {
                            match reply.substring(0, 1) {
                                "y" => {
                                    let res_del = delete_task_from_completed(comp,task.clone());
                                    if res_del.is_err(){
                                        return Err(res_del.err().unwrap())
                                    }
                                }
                                _ => {
                                    println!("Task not deleted.");
                                    println!("Deleted 0 tasks.");
                                }
                            }
                            done = true;
                        }
                    }
                }
                _ => {
                    let len = v_tasks.clone().len();

                    while !done {
                        for t in 0..len {
                            let task = v_tasks.get(t).unwrap();

                            let out_text = format!("Delete task {} '{}'? (yes/no/all/quit) ", 
                                                task.clone().uuiid, task.clone().description);
                            let res_reply = get_input(&out_text);
                            if res_reply.is_err() {
                                return Err(res_reply.err().unwrap())
                            }
                            let reply = res_reply.unwrap().to_lowercase();
                            if  reply.starts_with("y") || reply.starts_with("n") ||
                                reply.starts_with("a") || reply.starts_with("q") {
                                match reply.substring(0, 1) {
                                    "y" => {
                                        let res_del = delete_task_from_completed(comp,task.clone());
                                        if res_del.is_err(){
                                            return Err(res_del.err().unwrap())
                                        }
                                        del_counter += 1;

                                        if t + 1 < len {
                                            println!();
                                            continue;
                                        }
                                        // done = true;
                                    }
                                    "n" => {
                                        println!("Task not deleted.");

                                        if t + 1 < len {
                                            println!();
                                            continue;
                                        }
                                        // done = true;
                                    }
                                    "a" => {
                                        for task in v_tasks.clone() {
                                            let res_del = delete_task_from_completed(comp,task.clone());
                                            if res_del.is_err(){
                                                return Err(res_del.err().unwrap());
                                            }
                                            del_counter += 1;
                                        }
                                        done = true;
                                        break;
                                    }
                                    "q" => {
                                        println!("Task not deleted.");
                                        done = true;
                                        break;
                                    }
                                    _ => {
                                    }
                                }
                                done = true;
                            }
                        }
                    }
                    println!("Deleted {} {}",del_counter.to_string(), units("task", del_counter));
                }
            }
        }
    }

    Ok(())
}


//function to complete tasks; return number of tasks completed
pub fn command_done(v_id:Vec<i64>, pend: &mut List, comp: &mut List, settings: &SettingsMap  ) -> Result<(), &'static str> {
    // remember that tasks are not zero based
    let mut v_uu_int:Vec<i64> = Vec::new();
    let mut v_task:Vec<Task>  = Vec::new();
    let mut mess: Vec<String> = Vec::new();
    let now               = lts_now();

    //lets see if all are valid
    for id in v_id {
        let task = pend.get_task_from_id(id);
        if task.is_err() {
            return Err("One or more id's are not valid.");
        }
        v_uu_int.push(task.unwrap().uuiid_int);
    }

    println!("This command will alter {} {}.", v_uu_int.len(), units("task", v_uu_int.len()));

    for uu in v_uu_int.clone() {
        let index = pend.get_index_of_task_with_uuiid_int(uu);
        let mut task = pend.list.remove(index as usize);

        match task.start.clone() {
            Some(secs) => {
                let diff = now - secs;
                task.timetrackingseconds +=  diff;
            }
            None => {
                let diff = now - task.entry;
                task.timetrackingseconds +=  diff;
            }
        }
        task.start = None;
        task.end = Some(now);
        task.status = Status::Completed;
        
        println!("Completed task {} '{}'.", task.clone().uuiid, task.clone().description);
        let pt = make_timetracking_string(task.timetrackingseconds);
        v_task.push(task.clone());
        mess.push(pt);
    }

    pend.save();
    println!("Completed {} {}.", v_uu_int.clone().len(), units("task", v_uu_int.clone().len()));

    for m in mess.clone() {
        let line = format!("Total Time Tracked: {}\n", m);
        to_orange_feedback(&line);
    }

    for task in v_task {
        comp.list.push(task);
    }
    comp.save();
    show_nag(settings);

    Ok(())
}

pub fn command_modification(args: &Vec<String>, v_id: &Vec<i64>, v_hex: &Vec<String>, pend: &mut List, 
                            comp: &mut List) -> Result<(), &'static str> {
    let mut v_tasks: Vec<Task> = Vec::new();
    let mut v_mods: Vec<String> = Vec::new();

    match v_hex.len() {
        // assume non-completed tasks
        0 => {
            // lets see if they are all valid
            for id in v_id.clone() {
                let task = pend.get_task_from_id(id);
                if task.is_err() {
                    return Err("Invalid task id's given.")
                }
                v_tasks.push(task.clone().unwrap());
            }

            let len = args.clone().len();
            if len < 4 {
                return Err("No modifications given!");
            }

            // give message
            println!("This command will alter {} {}",v_tasks.clone().len(), units("task", v_tasks.clone().len()));

            // lets get the mods
            for m in 3..len {
                let s1 = args.get(m).unwrap().to_string();
                v_mods.push(s1);
            }

            for index in 0..v_tasks.clone().len() {
                let task = v_tasks.get(index).unwrap();
                
                //lets make a function that strips search terms and appends mods
                let new_str = strip_and_dip(&mut v_mods, task);
                if new_str.is_err() {
                    return Err(new_str.err().unwrap());
                }
                let to_be_split = new_str.unwrap();
                let new_split:Vec<_> = to_be_split.split("\t").collect();
                let res_task = make_task(new_split.clone());
                if res_task.is_err() {
                    return Err(res_task.err().unwrap());
                }

                // lets remove and add
                let index = pend.get_index_of_task_with_id(task.id.unwrap()) as usize;
                pend.list.remove(index);
                pend.list.insert(index, res_task.unwrap());

                println!("Modifying task {} '{}'",task.id.unwrap(),task.description);
            }
            
            println!("Modified {} '{}'",v_tasks.len(),units("task", v_tasks.len()));
            pend.save();
        }

        // assume hex uuiid's
        _ => {
            // lets see if they are all valid
            for h in v_hex.clone() {
                let task = comp.get_task_from_uuiid(h);
                if task.is_err() {
                    return Err("Invalid task id's given.")
                }
                v_tasks.push(task.clone().unwrap());
            }

            let len = args.clone().len();
            if len < 4 {
                return Err("No modifications given!");
            }

            // give message
            println!("This command will alter {} {}",v_tasks.clone().len(), units("task", v_tasks.clone().len()));

            // lets get the mods
            for m in 3..len {
                let s1 = args.get(m).unwrap().to_string();
                v_mods.push(s1);
            }

            for index in 0..v_tasks.clone().len() {
                let task = v_tasks.get(index).unwrap();
                
                //lets make a function that strips search terms and appends mods
                let new_str = strip_and_dip(&mut v_mods, task);
                if new_str.is_err() {
                    return Err(new_str.err().unwrap());
                }
                let to_be_split = new_str.unwrap();
                let new_split:Vec<_> = to_be_split.split("\t").collect();
                let res_task = make_task(new_split.clone());
                if res_task.is_err() {
                    return Err(res_task.err().unwrap());
                }

                // lets remove and add
                // let index = comp.get_index_of_task_with_id(task.id.unwrap()) as usize;
                let index = comp.get_index_of_task_with_uuiid_int(task.uuiid_int) as usize;
                comp.list.remove(index);
                comp.list.insert(index, res_task.unwrap());

                println!("Modifying task {} '{}'",task.uuiid, task.description);
            }
            
            println!("Modified {} '{}'",v_tasks.len(),units("task", v_tasks.len()));
            comp.save();
        }
    }

    // comp.save();

    Ok(())                        
}

// start the given tasks
pub fn command_start(v_int: &Vec<i64>, v_hex: &Vec<String>,
                        pend: &mut List, comp: &mut List) -> Result<(), &'static str> {
    let mut v_indeces: Vec<usize> = Vec::new();

    // we can do one hexidecimal start, but balk at more than one
    match v_hex.len() {
        // do one or more integers
        0 => {
            // lets see if they are all valid
            for id in v_int.clone() {
                let task = pend.get_task_from_id(id);
                if task.is_err() {
                    return Err("Invalid task id's given.")
                }
                match task.clone().unwrap().start {
                    None => {
                        let index = pend.get_index_of_task_with_uuiid_int(task.unwrap().uuiid_int) as usize;
                        v_indeces.push(index);
                    }
                    Some(s) => {
                        let now = lts_now();
                        let in_future = s > now;
                        match in_future {
                            true => {
                                println!("Task {} has already been scheduled to start.", task.clone().unwrap().id.unwrap() );
                                let index = pend.get_index_of_task_with_uuiid_int(task.clone().unwrap().uuiid_int) as usize;
                                v_indeces.push(index);
                            }
                            false => {
                                println!("Task {} already started.", task.clone().unwrap().id.unwrap() );
                            }
                        }
                    }
                }
            }

            println!("This command will alter {} {}", v_indeces.len(), units("task", v_indeces.len()));

            for u in v_indeces.clone() {
                pend.start_task(u);
            }

            // save if changes have been made
            if v_indeces.len() > 0 {
                pend.save();
            }
            println!("Started {} {}", v_indeces.len(), units("task", v_indeces.len()));
        }
        1 => {
            // lets see if they are all valid
            for hex in v_hex.clone() {
                let task = comp.get_task_from_uuiid(hex);
                if task.is_err() {
                    return Err("Invalid task hexidecimal given (or not in completed tasks).")
                }
                let index = comp.get_index_of_task_with_uuiid_int(task.unwrap().uuiid_int) as usize;
                v_indeces.push(index);
            }

            println!("This command will alter {} {}", v_indeces.len(), units("task", v_indeces.len()));

            for u in v_indeces.clone() {
                comp.start_task(u);
            }

            // swap out task and save changes
            if v_indeces.len() > 0 {
                let t = comp.list.remove(v_indeces[0]);
                comp.save();
                pend.list.push(t);
                pend.save();
            }
            println!("Started {} {}", v_indeces.len(), units("task", v_indeces.len()));
        }
        // too many hexidecimals
        _ => {
            return Err("Too many hexidecimal tasks are started.")
        }
    }

    Ok(())
}

// stop the given tasks
pub fn command_stop(v_int: &Vec<i64>, v_hex: &Vec<String>,
                    pend: &mut List, allt: &List) -> Result<(), &'static str> {
    // remember that tasks are not zero based
    let mut v_index: Vec<usize> = Vec::new();
    let mut mess: Vec<String> = Vec::new();
    let now = lts_now();

    match v_hex.len() {
        0 => {
            // assume all are id's
            for id in v_int.clone() {
                let res_index = pend.get_index_of_task_with_id(id);
                if res_index < 0 {
                    return Err("One (or all) of the given id's are not valid.")
                }
                let task = pend.list.get(res_index as usize).unwrap();
                match task.clone().start {
                    Some(_s) => {
                        v_index.push(res_index as usize);
                    }
                    None => {
                        println!("Task {} '{}' has not started",task.clone().id.unwrap(), task.clone().description);
                    }
                }
            }
            
            // lets stop the tasks
            for t in v_index.clone() {
                let mut task = pend.list.remove(t);
                // let now = lts_now();
                let diff = now - task.start.unwrap();
                task.timetrackingseconds +=  diff;
                task.start = None;
                
                println!("Stopping task {} '{}'.", task.clone().uuiid, task.clone().description);
                let pt = make_timetracking_string(task.timetrackingseconds);
                mess.push(pt);
                pend.list.insert(t, task.clone());
            }
            
            println!("Stopped {} {}.", v_index.clone().len(), units("task", v_index.clone().len()));
            for m in mess.clone() {
                let line = format!("Total Time Tracked: {}\n", m);
                to_orange_feedback(&line);
            }
            
            if mess.clone().len() > 0 {
                pend.save()
            }
        }
        1 => {
            // assume one hex
            // let mut changed = false;
            let index = allt.get_index_of_task_with_uuiid(&v_hex[0]);
            if index < 0 {
                return Err("The given hexidecimal string is not valid.")
            }
            let temp = allt.list.get(index as usize).unwrap();
            match temp.is_complete() {
                true => {
                    println!("Task {} '{}' not started.",temp.uuiid, temp.description);
                }
                false => {
                    let ii = pend.get_index_of_task_with_uuiid_int(temp.uuiid_int);
                    if ii < 0 {
                        return Err("Seems an invalid index somewhere here.")
                    }
                    let mut task = pend.list.remove(ii as usize);
                    // let now = lts_now();
                    let diff = now - task.start.unwrap();
                    task.timetrackingseconds +=  diff;
                    task.start = None;
                    
                    println!("Stopping task {} '{}'.", task.clone().uuiid, task.clone().description);
                    let pt = make_timetracking_string(task.timetrackingseconds);
                    mess.push(pt);
                    pend.list.insert(ii as usize, task.clone());
                }
            }
            
            println!("Stopped {} {}.", mess.clone().len(), units("task", mess.clone().len()));
            for m in mess.clone() {
                let line = format!("Total Time Tracked: {}\n", m);
                to_orange_feedback(&line);
            }
            
            if mess.clone().len() > 0 {
                pend.save()
            }
        }
        _ => {
            return Err("Too many hexidecimal strings - abandoned.");
        }
    }

    Ok(())
}

// return a vector of filenames
pub fn create_data_dirs() -> Result<Vec<String>, &'static str> {
    let mut ret:Vec<String> = Vec::new();
    let path:String;
    
    let res_cur_dir = env::current_dir();
    if res_cur_dir.is_err() {
        return Err("Error in querying the current directory.")
    }
    let current_directory = res_cur_dir.unwrap().into_os_string().into_string().unwrap();

    match crate::RELEASE {
        true => {
            let working = "data";
            path = current_directory + "/" + working;
            if !Path::new(&path).exists() {
                let res = fs::create_dir(&path);
                if res.is_err() {
                    return Err("Cannot create data directory (maybe debug version)");
                }
            }
        }
        false => {
            let working = "test/working";
            path = current_directory + "/" + working;
            if !Path::new(&path).exists() {
                let res = fs::create_dir(&path);
                if res.is_err() {
                    return Err("Cannot create data directory (maybe debug version)");
                }
            }
        }
    }
    // 0
    let settings_file = path.clone() + "/" + "settings.txt";
    ret.push(settings_file);

    // 1
    let pending_file = path.clone() + "/" + "pending.data";
    ret.push(pending_file);

    // 2
    let completed_file = path + "/" + "completed.data";
    ret.push(completed_file);
    
    Ok(ret)
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

pub fn get_input(text: &str) -> Result<String, &'static str> {
    print!("{}",text);
    match io::stdout().flush() {
        Ok(_) => print!(""),
        Err(_) => return Err("Some kind of flush error"),
    }

    let mut ret = String::new();
    match io::stdin().read_line(&mut ret) {
        Ok(_) => {
            ret = ret.trim().to_string();
            if ret.len() > 0 {
                // let greeting = "Hello, ".to_string() + &name + &", nice to meet you!".to_string();
                // println!("{}", greeting);
                // println!("{}", ret);
            } else {
                // println!("No name entered, goodbye.");
                return Err("Nothing entered!");
            }
        }
        Err(_) => return Err("Some kind of readln error")
    }

    Ok(ret)
}

pub fn get_integer_single_report(settings: &SettingsMap, colors: Colors, uuiid_int: i64, all: &List)
                                -> Result<(), &'static str> {
    let mut found = false;
    let mut task:Task = Task::new();

    for tasky in all.clone().list {
        if uuiid_int == tasky.uuiid_int {
            found = true;
            task = tasky;
            break;
        }
    }

    if ! found {
        return Err("Task id or uuiid does not exist.")
    }

    let result = report_single(settings, &colors, &task);
    if result.is_err() {
        return Err(result.err().unwrap());
    }

    Ok(())
}

pub fn get_task_lines_completed(col_sizes: &Vec<usize>, block: &str, task: &Task) -> Vec<String> {
    let mut ret:Vec<String> = Vec::new();
    let mut line:String;
    let now = lts_now();

    // task line
    line = justify(task.clone().uuiid, col_sizes[0], Justify::Right) + " ";
    let diff = now - task.clone().entry;
    line += &align_timeframe(diff);                                                           // col size = 7
    line += " ";
    line += &justify(make_timetracking_string(task.clone().timetrackingseconds), col_sizes[2], Justify::Right);
    line += " ";
    let num_tags = task.clone().tags.len();
    match num_tags {
        0 => {
            line += &repeat_char(" ".to_string(), col_sizes[3])
        }
        1 => {
            line += &justify(task.clone().tags[0].to_string(), col_sizes[3], Justify::Left);
        }
        _ => {
            let temp = format!("[{}]",task.clone().tags.len());
            line += &justify(temp, col_sizes[3], Justify::Center);
        }
    }
    line += " ";
    line += &lts_to_date_string(task.clone().end.unwrap());
    line += " ";
    line += &justify(task.clone().description, col_sizes[5], Justify::Left);
    ret.push(line);

    
    // lets do the annotations
    for anno in task.clone().ann {
        line = block.to_string();
        line += &lts_to_date_string(anno.date);
        line += " ";

        // take into account: date length(10) and three extra spaces (i know two, dunno three)
        let len_anno = col_sizes[5] - 10 - 3;
        line += &justify(anno.desc, len_anno, Justify::Left);
        ret.push(line);
    }
    
    return ret;
}

pub fn get_task_lines_pending(col_sizes: &Vec<usize>, block: &str, task: &Task) -> Vec<String> {
    let mut ret:Vec<String> = Vec::new();
    let mut line:String;
    let mut diff:i64;
    let now = lts_now();

    // task line
    line = justify(task.clone().id.unwrap().to_string(), col_sizes[0], Justify::Right) + " ";
    diff = now - task.clone().entry;
    line += &align_timeframe(diff);                                                           // col size = 7
    line += " ";
    // line += &justify(make_timetracking_string(task.clone().timetrackingseconds), col_sizes[2], Justify::Right);
    // line += " ";
    let num_tags = task.clone().tags.len();
    match num_tags {
        0 => {
            line += &repeat_char(" ".to_string(), col_sizes[2])
        }
        1 => {
            line += &justify(task.clone().tags[0].to_string(), col_sizes[2], Justify::Left);
        }
        _ => {
            let temp = format!("[{}]",task.clone().tags.len());
            line += &justify(temp, col_sizes[2], Justify::Center);
        }
    }
    line += " ";
    
    // Due like -1.3y   or -13w
    match task.due {
        Some(ts) => {
            diff = ts - now;
            line += &align_timeframe(diff); 
        }
        None => {
            line += &repeat_char(" ".to_string(), col_sizes[3]); 
        }
        
    }
    line += " ";
    line += &justify(task.clone().description, col_sizes[4], Justify::Left);
    ret.push(line);

    // lets do the annotations
    for anno in task.clone().ann {
        line = block.to_string();
        line += &lts_to_date_string(anno.date);
        line += " ";

        // take into account: date length(10) and three extra spaces (i know two, dunno three)
        let len_anno = col_sizes[4] - 10 - 3;
        line += &justify(anno.desc, len_anno, Justify::Left);
        ret.push(line);
    }
    
    return ret;
}

pub fn get_task_line_recurring(col_sizes: &Vec<usize>, block: &str, task: &Task) -> Vec<String> {
    let mut ret:Vec<String> = Vec::new();
    let mut line:String;
    let mut temp_str:String;
    let now = lts_now();


    // ID
    line = justify(task.clone().id.unwrap().to_string(), col_sizes[0], Justify::Right) + " ";

    // UUIID
    temp_str =  task.clone().uuiid + " ";
    line += temp_str.as_str();
    
    // Age
    let diff = now - task.entry;
    temp_str = align_timeframe(diff) + " ";
    line += temp_str.as_str();
    
    // Tags
    match task.is_tagged() {
        true => {
            let num_tags = task.clone().tags.len();
            temp_str = format!("[{}]",num_tags);
            temp_str = justify(temp_str.clone(), col_sizes[3], Justify::Right) + " ";
            line += temp_str.as_str();
            
        }
        false => {
            line += repeat_char(" ".to_string(), col_sizes[3] +1).as_str();
        }
    }
    
    // Description
    temp_str = justify(task.clone().description, col_sizes[4], Justify::Left);
    line += temp_str.as_str();
    ret.push(line);
    
    // Annotations
    for anno in task.clone().ann {
        temp_str = block.to_string() + lts_to_date_string(anno.date).as_str() + " ";
        // who knows wy i'm subtracting (10-date, 2-tab, 1 space between date and desc  = 13)
        line = temp_str + justify(anno.desc, col_sizes[4]-13, Justify::Left).as_str();
        ret.push(line);
    }

    return ret;
}

pub fn get_task_line_waiting(col_sizes: &Vec<usize>, block: &str, task: &Task) -> Vec<String> {
    let mut ret:Vec<String> = Vec::new();
    let mut line:String;
    let mut temp_str:String;
    let mut diff:i64;
    let now = lts_now();


    // ID
    line = justify(task.clone().id.unwrap().to_string(), col_sizes[0], Justify::Right) + " ";

    // UUIID
    temp_str =  task.clone().uuiid + " ";
    line += temp_str.as_str();
    
    // Age
    diff = now - task.entry;
    temp_str = align_timeframe(diff) + " ";
    line += temp_str.as_str();
    
    // Tags
    match task.is_tagged() {
        true => {
            let num_tags = task.clone().tags.len();
            temp_str = format!("[{}]",num_tags);
            temp_str = justify(temp_str.clone(), col_sizes[3], Justify::Right) + " ";
            line += temp_str.as_str();
            
        }
        false => {
            line += repeat_char(" ".to_string(), col_sizes[3] +1).as_str();
        }
    }
    
    // Wait
    diff = task.wait.unwrap() -  now;
    temp_str = align_timeframe(diff) + " ";
    line += temp_str.as_str();


    // Description
    temp_str = justify(task.clone().description, col_sizes[5], Justify::Left);
    line += temp_str.as_str();
    ret.push(line);
    
    // Annotations
    for anno in task.clone().ann {
        temp_str = block.to_string() + lts_to_date_string(anno.date).as_str() + " ";
        // who knows wy i'm subtracting (10-date, 2-tab, 1 space between date and desc  = 13)
        line = temp_str + justify(anno.desc, col_sizes[5]-13, Justify::Left).as_str();
        ret.push(line);
    }

    return ret;
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
        "a" | "-a" | "active" => {
            return Ok("active");
        }

        "add" => {
            return Ok(first);
        }

        "-color_test" | "col" | "color" | "color_test" | "-color" | "colortest" => {
            return Ok("colortest");
        }
        
        "c" | "-c" | "-completed" | "completed" | "comp" | "-comp" => {
            return Ok("completed");
        }
        
        "r" | "-r" | "-recurring" | "recurring" => {
            return Ok("recurring");
        }
        
        "v" | "-v" | "-version" | "ver" | "version" | "-ver" => {
            return Ok("version");
        }
        
        "w" | "-w" | "-waiting" | "waiting"  => {
            return Ok("waiting");
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

// make timetracking timeframe like   1.5y    or   -3w
pub fn make_timetracking_timeframe(secs: i64) -> String {
    let mut neg = false;
    
    if secs < 0 {
        neg = true;
    }
    
    let term = categorize_term(secs.abs());

    if neg {
        let ret = "-".to_string() + &term;
        return ret;
    }

    return term;
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

// function to strip out mod terms in task and run through new make_task
pub fn strip_and_dip(mods: &mut Vec<String>, task: &Task) -> Result<String, &'static str> {
    let mut ret = "".to_string();
    let mut v_index: Vec<usize> = Vec::new();
    let mut replacement: String;

    let line = make_line_from_task(task);
    let mut split_tab:Vec<_> = line.split("\t").collect();

    // we need to replace the '+' sign with 'tags:' in order for AllowMods to work
    let m_len = mods.len();
    for t in 0..m_len {
        let found = mods[t].substring(0, 1) == "+";
        if found {
            replacement = mods.remove(t);
            replacement = replacement.replace("+", "tags:");
            mods.insert(t, replacement);
        }
    }

    // get the allowed mods
    for i in 0..split_tab.clone().len() {
        let term = split_tab[i];
        for m in mods.clone() {
            let res_search = AllowMods::from_str(&m);
            if res_search.is_err() {
                return Err("Unknown search term.")
            }

            let a1 = res_search.unwrap();
            let search_term = a1.text();

            let sub = term.substring(0, 3);
            if sub == search_term {
                v_index.push(i);
            }
        }
    }

    // we need a top-down sort
    v_index.sort();
    v_index.reverse();

    // remove the affected terms
    for i in 0..v_index.len() {
        split_tab.remove(v_index[i]);
    }

    // add the mods
    for m in mods {
        ret.push_str(m);
        ret.push_str("\t");
    }

    // add the rest of the split_tab
    let len_end = split_tab.len() -1;
    for i in 0..=len_end {
        ret.push_str(split_tab[i]);
        if i != len_end {
            ret.push_str("\t");
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
        
        let _res_load = load_task_file(dest1, &mut pen, &mut hd_set);
        remove_file(dest1).expect("Cleanup test failed");
        remove_file(dest2).expect("Cleanup test failed");

        let result = get_integer_single_report(&settings, colors, 1, &pen);
        assert_eq!(result.is_err(), false);
    }
    
    // #[ignore]
    #[test]
    fn t015_align_timeframe() {
        let t:i64 = 100_000;
        let res = align_timeframe(t);
        assert_eq!(res, "   1d  ");
        
        let t:i64 = 1_900_000;
        let res = align_timeframe(t);
        assert_eq!(res, "   3w  ");
        
        let t:i64 = 10_900_000;
        let res = align_timeframe(t);
        assert_eq!(res, "   4mo ");
        
        let t:i64 = 57;
        let res = align_timeframe(t);
        assert_eq!(res, "  57s  ");
        
        let t:i64 = 129;
        let res = align_timeframe(t);
        assert_eq!(res, "   2min");
        
        let t:i64 = 35_000_000;
        let res = align_timeframe(t);
        assert_eq!(res, " 1.1y  ");
        
        let t:i64 = 10_000;
        let res = align_timeframe(t);
        assert_eq!(res, "   3h  ");
    }
    
    // #[ignore]
    #[test]
    fn t016_strip_and_dip() {
        let line = "description:how do i do\tdue:1658513756\t\
        entry:1658512756\tstart:1658513756\tstatus:pending\tuuiid:0x0001\ttags:houseboat";
        let vec:Vec<_> = line.split("\t").collect();
        let task1 = make_task(vec).unwrap();
        let mut mods = vec!["description:New decription Svenny!".to_string(), "wait:2022-09-01".to_string(), "+car".to_string()];
        let res = strip_and_dip(&mut mods, &task1).unwrap();
        let vec2:Vec<_> = res.split("\t").collect();
        let task2 = make_task(vec2).unwrap();
        assert_eq!(task2.description, "New decription Svenny!");
        assert_eq!(task2.entry, 1658512756);
        assert_eq!(task2.tags[0], "car");
        assert_eq!(task2.wait.unwrap(), 1661990400);
    }
    
    // #[ignore]
    #[test]
    fn t017_sort_reverse_vector() {
        let mut vec = vec![1,3,19,0,0,1];
        vec.sort();
        vec.reverse();

        assert_eq!(vec[0],19);
    }













} //end of tests