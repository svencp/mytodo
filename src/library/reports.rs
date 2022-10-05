/*
    Module for everything to do with reports
    2022.08.12      Sven Ponelat

*/


use crate::library::enums::Rtype;
use crate::library::functions::*;
use crate::library::lts::*;
use crate::library::structs::*;
use crate::library::settings::*;
use crate::library::my_utils::*;
use crate::library::task::*;
use crate::library::list::*;
use termion::{color, style};
use std::cmp::Ordering;
use std::process::exit;


pub const COLOR_ORANGE: color::Rgb = color::Rgb(246,116,0);










// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// a function to determine the width (number of spaces) for the annotation block
pub fn determine_block_width(col_sizes: &Vec<usize>) -> usize {
    let mut width:usize = 0;

    // only do up to the last coloumn
    let end = col_sizes.len() - 1;
    for i in 0..end  {
        width += col_sizes[i] + 1;
    }

    // add two for annotation tab
    return width + 2;
}

pub fn color_test(colors: Colors) {
    let line1 = "This is my test of the orange_feedback color (123 000)";
    let fg = colors.color_feedback_orange;
    let bg:Option<color::Rgb> = None;
    to_color_message(fg, bg, line1);
}

pub fn format_report_active(col_sizes: &Vec<usize>, headers: Vec<&str>, tasks: &Vec<Task>, colors: &Colors, settings: &SettingsMap) {
    let fg   = colors.clone().color_complete_orphan;
    let bg   = colors.clone().color_active_bg;
    let fgov = colors.clone().color_overdue;
    let bgov = colors.clone().color_black_bg;
    let anno_block = make_annotation_block(col_sizes);

    // make_heading(col_sizes,headers,settings);
    make_heading(col_sizes,headers,colors, "Active");
    
    for task in tasks {

        match task.is_active() {
            true => {
                print!("{}{}",color::Fg(fg), color::Bg(bg));
            }
            
            // assume that it is overdue
            false => {
                print!("{}{}",color::Fg(fgov), color::Bg(bgov));
            }
        }

        match task.ann.len() {
            // No annotations
            0 => {
                print_description_line(col_sizes, task);
            }

            _ => {
                print_description_line(col_sizes, &task);

                // Annotations
                for ann in task.ann.clone() {

                    match task.is_active() {
                        true => {
                            make_active(settings);
                        }
                        false => {
                            make_overdue(settings);
                        }
                    }

                    let desc = lts_to_date_string(ann.date) + " " + &ann.desc;
                    let d = justify(desc.clone(), col_sizes[5]-2, Justify::Left);
                    print!("{}{}",anno_block, d);
                    print!("{}\n",style::Reset);
                }
            }
        }
    }

    print!("\n");
    println!("{} {} \n",tasks.len(), units("task", tasks.len()));

}

pub fn format_report_all_pending(col_sizes: &Vec<usize>, headers: Vec<&str>, tasks: &Vec<Task>, colors: &Colors, heading: &str) {
    let normal_fg   = colors.clone().color_complete_orphan;
    let tagged_fg   = colors.clone().color_tagged;
    let rperiod_fg  = colors.clone().color_recur_period_fg;
    let rchained_fg = colors.clone().color_recur_chain_fg;
    let black_bg    = colors.clone().color_black_bg;
    let active_bg    = colors.clone().color_active_bg;
    let anno_block = make_annotation_block(col_sizes);

    let mut remainder: i64;
    let mut index: i64 = 0;
    let mut v_lines:Vec<String>;

    make_heading(col_sizes, headers, colors, heading);


    for task in tasks {
        index += 1;
        remainder = index % 2;

        v_lines = get_task_lines_pending(col_sizes, &anno_block, &task);

        match remainder {
            // The normally light option
            0 => {
                match task.is_active() {
                    true => {
                        make_dark_print(v_lines, normal_fg, active_bg)
                    }
                    false => {
                        match task.has_recur() {
                            true => {
                                match task.is_periodic() {
                                    true => {
                                        make_print(v_lines, rperiod_fg);
                                    }
                                    false => {
                                        make_print(v_lines, rchained_fg);
                                    }
                                }
                            }
                            false => {
                                match task.is_tagged() {
                                    true => {
                                        make_print(v_lines, tagged_fg);
                                    }
                                    false => {
                                        make_print(v_lines, normal_fg);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // make this one the dark one
            _ => {
                match task.is_active() {
                    true => {
                        make_dark_print(v_lines, normal_fg, active_bg)
                    }
                    false => {
                        match task.has_recur() {
                            true => {
                                match task.is_periodic() {
                                    true => {
                                        make_dark_print(v_lines, rperiod_fg, black_bg);
                                    }
                                    false => {
                                        make_dark_print(v_lines, rchained_fg, black_bg);
                                    }
                                }
                            }
                            false => {
                                match task.is_tagged() {
                                    true => {
                                        make_dark_print(v_lines, tagged_fg, black_bg);
                                    }
                                    false => {
                                        make_dark_print(v_lines, normal_fg, black_bg);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    print!("\n");
    println!("{} {} \n",tasks.clone().len(), units("task", tasks.len()));
}

pub fn format_report_completed(col_sizes: &Vec<usize>, headers: Vec<&str>, tasks: &Vec<Task>, colors: &Colors, heading: &str) {
    let normal_fg   = colors.clone().color_complete_orphan;
    let tagged_fg   = colors.clone().color_tagged;
    let rperiod_fg  = colors.clone().color_recur_period_fg;
    let rchained_fg = colors.clone().color_recur_chain_fg;
    let black_bg    = colors.clone().color_black_bg;
    let anno_block = make_annotation_block(col_sizes);

    let mut remainder: i64;
    let mut index: i64 = 1;
    let mut v_lines:Vec<String>;

    make_heading(col_sizes, headers, colors, heading);

    for task in tasks {
        index += 1;
        remainder = index % 2;

        v_lines = get_task_lines_completed(col_sizes, &anno_block, &task);

        match remainder {
            // dark black background
            0 => {
                match task.has_recur() {
                    true => {
                        match task.clone().rtype.unwrap() {
                            Rtype::Periodic => {
                                make_dark_print(v_lines, rperiod_fg, black_bg);
                            }
                            Rtype::Chained => {
                                make_dark_print(v_lines, rchained_fg, black_bg);
                            }
                        }
                    }
                    false => {
                        match task.is_tagged() {
                            true => {
                                make_dark_print(v_lines, tagged_fg, black_bg);
                            }
                            false => {
                                make_dark_print(v_lines, normal_fg, black_bg);
                            }
                        }
                    }
                }
            }
            // normal background
            _ => {
                match task.has_recur() {
                    true => {
                        match task.clone().rtype.unwrap() {
                            Rtype::Periodic => {
                                make_print(v_lines, rperiod_fg);
                            }
                            Rtype::Chained => {
                                make_print(v_lines, rchained_fg)
                            }
                        }
                    }
                    false => {
                        match task.is_tagged() {
                            true => {
                                make_print(v_lines, tagged_fg)
                            }
                            false => {
                                make_print(v_lines, normal_fg)
                            }
                        }
                    }
                }
            }
        } // end of match
    } // end of for loop

    print!("\n");
    println!("{} {} \n",tasks.clone().len(), units("task", tasks.len()));
}

pub fn format_report_recurring(col_sizes: &Vec<usize>, headers: Vec<&str>, tasks: &Vec<Task>, colors: &Colors, heading: &str) {
    let rperiod_fg  = colors.clone().color_recur_period_fg;
    let rchained_fg = colors.clone().color_recur_chain_fg;
    let black_bg    = colors.clone().color_black_bg;
    let anno_block = make_annotation_block(col_sizes);

    let mut remainder: i64;
    let mut index: i64 = 1;
    let mut v_lines:Vec<String>;

    make_heading(col_sizes, headers, colors, heading);

    for task in tasks {
        index += 1;
        remainder = index % 2;

        v_lines = get_task_line_recurring(col_sizes, &anno_block, &task);

        match remainder {
            // dark black background
            0 => {
                match task.is_periodic() {
                    true => {
                        make_dark_print(v_lines, rperiod_fg, black_bg);
                    }
                    // assume it is chained
                    false => {
                        make_dark_print(v_lines, rchained_fg, black_bg);
                    }
                }
            }
            // normal background
            _ => {
                match task.is_periodic() {
                    true => {
                        make_print(v_lines, rperiod_fg);
                    }
                    // assume it is chained
                    false => {
                        make_print(v_lines, rchained_fg);
                    }
                }
            }
        }
    }

    print!("\n");
    println!("{} {} \n",tasks.clone().len(), units("task", tasks.len()));
}

pub fn format_report_search(col_sizes: &Vec<usize>, headers: Vec<&str>, tasks: &Vec<Task>, colors: &Colors, heading: &str) {
    let normal_fg   = colors.clone().color_complete_orphan;
    let tagged_fg   = colors.clone().color_tagged;
    let rperiod_fg  = colors.clone().color_recur_period_fg;
    let rchained_fg = colors.clone().color_recur_chain_fg;
    let black_bg    = colors.clone().color_black_bg;
    let anno_block = make_annotation_block(col_sizes);

    let mut remainder: i64;
    let mut index: i64 = 1;
    let mut v_lines:Vec<String>;

    make_heading(col_sizes, headers, colors, heading);

    for task in tasks {
        index += 1;
        remainder = index % 2;

        v_lines = get_task_lines_search(col_sizes, &anno_block, &task);

        match remainder {
            // dark black background
            0 => {
                match task.has_recur() {
                    true => {
                        match task.clone().rtype.unwrap() {
                            Rtype::Periodic => {
                                make_dark_print(v_lines, rperiod_fg, black_bg);
                            }
                            Rtype::Chained => {
                                make_dark_print(v_lines, rchained_fg, black_bg);
                            }
                        }
                    }
                    false => {
                        match task.is_tagged() {
                            true => {
                                make_dark_print(v_lines, tagged_fg, black_bg);
                            }
                            false => {
                                make_dark_print(v_lines, normal_fg, black_bg);
                            }
                        }
                    }
                }
            }
            // normal background
            _ => {
                match task.has_recur() {
                    true => {
                        match task.clone().rtype.unwrap() {
                            Rtype::Periodic => {
                                make_print(v_lines, rperiod_fg);
                            }
                            Rtype::Chained => {
                                make_print(v_lines, rchained_fg)
                            }
                        }
                    }
                    false => {
                        match task.is_tagged() {
                            true => {
                                make_print(v_lines, tagged_fg)
                            }
                            false => {
                                make_print(v_lines, normal_fg)
                            }
                        }
                    }
                }
            }
        } // end of match
    } // end of for loop

    print!("\n");
    println!("{} {} \n",tasks.clone().len(), units("task", tasks.len()));
}

pub fn format_report_single(col_sizes: &Vec<usize>, headers: Vec<&str>, lines: Vec<Vec<String>>, colors: &Colors, task: &Task ) {
    let normal_fg   = colors.clone().color_complete_orphan;
    let tagged_fg   = colors.clone().color_tagged;
    let rperiod_fg  = colors.clone().color_recur_period_fg;
    let rchained_fg = colors.clone().color_recur_chain_fg;
    let active_bg   = colors.clone().color_active_bg;
    let black_bg    = colors.clone().color_black_bg;

    let mut index: i64 = 1;

    // make_heading(col_sizes,headers,settings);
    make_heading(col_sizes,headers,colors, "Single");
    let num_lines = lines.clone()[1].len();

    for i in 0..num_lines {
        if lines[0][i].clone().starts_with("Desc")  {
            // first col 
            print!("{}",color::Fg(normal_fg));
            print!("{}", justify(lines[0][i].clone(), col_sizes[0] + 1, Justify::Left));
            print!("{}",style::Reset);

            // second col
            match task.is_active() {
                true => {
                    print!("{}{}",color::Fg(normal_fg), color::Bg(active_bg));
                    print!("{}", justify(lines[1][i].clone(), col_sizes[1], Justify::Left));
                    print!("{}\n",style::Reset);
                }
                false => {
                    match task.has_recur() {
                        true => {
                            match task.is_periodic() {
                                true => {
                                    print!("{}",color::Fg(rperiod_fg));
                                    print!("{}", justify(lines[1][i].clone(), col_sizes[1], Justify::Left));
                                    print!("{}\n",style::Reset); 
                                }
                                // assume it is chained
                                false => {
                                    print!("{}",color::Fg(rchained_fg));
                                    print!("{}", justify(lines[1][i].clone(), col_sizes[1], Justify::Left));
                                    print!("{}\n",style::Reset); 
                                }
                            }
                        }
                        false => {
                            match task.is_tagged() {
                                true => {
                                    print!("{}",color::Fg(tagged_fg));
                                    print!("{}", justify(lines[1][i].clone(), col_sizes[1], Justify::Left));
                                    print!("{}\n",style::Reset); 
                                }
                                false => {
                                    print!("{}",color::Fg(normal_fg));
                                    print!("{}", justify(lines[1][i].clone(), col_sizes[1], Justify::Left));
                                    print!("{}\n",style::Reset);
                                }
                            }
                        }
                    }
                }
            }

            // we have to reset the index here in order for annotations to follow pattern
            index = 1;
            continue;
        }

        // take care of any annotations
        if lines[0][i].clone().len() == 0 {
            // first col 
            print!("{}",color::Fg(normal_fg));
            print!("{}", justify(lines[0][i].clone(), col_sizes[0] + 1, Justify::Left));
            print!("{}",style::Reset);

            // second col
            match task.is_active() {
                true => {
                    print!("{}{}",color::Fg(normal_fg), color::Bg(active_bg));
                    print!("  {}", justify(lines[1][i].clone(), col_sizes[1]-2, Justify::Left));
                    print!("{}\n",style::Reset);
                }
                false => {
                    match task.has_recur() {
                        true => {
                            match task.is_periodic() {
                                true => {
                                    print!("{}",color::Fg(rperiod_fg));
                                    print!("  {}", justify(lines[1][i].clone(), col_sizes[1]-2, Justify::Left));
                                    print!("{}\n",style::Reset); 
                                }
                                // assume it is chained
                                false => {
                                    print!("{}",color::Fg(rchained_fg));
                                    print!("  {}", justify(lines[1][i].clone(), col_sizes[1]-2, Justify::Left));
                                    print!("{}\n",style::Reset); 
                                }
                            }
                        }
                        false => {
                            match task.is_tagged() {
                                true => {
                                    print!("{}",color::Fg(tagged_fg));
                                    print!("  {}", justify(lines[1][i].clone(), col_sizes[1]-2, Justify::Left));
                                    print!("{}\n",style::Reset); 
                                }
                                false => {
                                    print!("{}",color::Fg(normal_fg));
                                    print!("  {}", justify(lines[1][i].clone(), col_sizes[1]-2, Justify::Left));
                                    print!("{}\n",style::Reset);
                                }
                            }
                        }
                    }
                }
            }

            // we have to reset the index here in order for annotations to follow pattern
            index = 1;
            continue;
        }

        match index % 2 {
            0 => {
                print!("{}",color::Fg(normal_fg));
                print!("{}", justify(lines[0][i].clone(), col_sizes[0] + 1, Justify::Left));
                print!("{}", justify(lines[1][i].clone(), col_sizes[1], Justify::Left));
                print!("{}\n",style::Reset);
            }
            _ => {
                // make dark bg
                print!("{}{}",color::Fg(normal_fg), color::Bg(black_bg));
                print!("{}", justify(lines[0][i].clone(), col_sizes[0] + 1, Justify::Left));
                print!("{}", justify(lines[1][i].clone(), col_sizes[1], Justify::Left));
                print!("{}\n",style::Reset);
            }
        }

        index += 1;
    }

    print!("\n\n");
}

pub fn format_report_waiting(col_sizes: &Vec<usize>, headers: Vec<&str>, tasks: &Vec<Task>, colors: &Colors, heading: &str) {
    let normal_fg   = colors.clone().color_complete_orphan;
    let tagged_fg   = colors.clone().color_tagged;
    let rperiod_fg  = colors.clone().color_recur_period_fg;
    let rchained_fg = colors.clone().color_recur_chain_fg;
    let black_bg    = colors.clone().color_black_bg;
    let anno_block = make_annotation_block(col_sizes);

    let mut remainder: i64;
    let mut index: i64 = 1;
    let mut v_lines:Vec<String>;

    make_heading(col_sizes, headers, colors, heading);

    for task in tasks {
        index += 1;
        remainder = index % 2;

        v_lines = get_task_line_waiting(col_sizes, &anno_block, &task);

        match remainder {
            // dark black background
            0 => {
                match task.is_child() {
                    true => {
                        match task.clone().rtype.unwrap() {
                            Rtype::Periodic => {
                                make_dark_print(v_lines, rperiod_fg, black_bg);
                            }
                            Rtype::Chained => {
                                make_dark_print(v_lines, rchained_fg, black_bg);
                            }
                        }
                    }
                    false => {
                        match task.is_tagged() {
                            true => {
                                make_dark_print(v_lines, tagged_fg, black_bg);
                            }
                            false => {
                                make_dark_print(v_lines, normal_fg, black_bg);
                            }
                        }
                    }
                }
            }
            // normal background
            _ => {
                match task.is_child() {
                    true => {
                        match task.clone().rtype.unwrap() {
                            Rtype::Periodic => {
                                make_print(v_lines, rperiod_fg);
                            }
                            Rtype::Chained => {
                                make_print(v_lines, rchained_fg)
                            }
                        }
                    }
                    false => {
                        match task.is_tagged() {
                            true => {
                                make_print(v_lines, tagged_fg)
                            }
                            false => {
                                make_print(v_lines, normal_fg)
                            }
                        }
                    }
                }
            }
        } // end of match
    }

    print!("\n");
    println!("{} {} \n",tasks.clone().len(), units("task", tasks.len()));
}

pub fn make_active(settings: &SettingsMap) {
    let res_fg = settings.get_color("color_complete_orphan");
    if res_fg.is_err() {
        let message = "Problems retrieving color from settings".to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }
    let res_bg = settings.get_color("color_active_bg");
    if res_bg.is_err() {
        let message = "Problems retrieving color from settings".to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }
    let fg   = res_fg.unwrap();
    let bg   = res_bg.unwrap();

    print!("{}{}", color::Fg(fg), color::Bg(bg));
}

// this function makes a width of spaces to fill when a task has an annotation
pub fn make_annotation_block(col_sizes: &Vec<usize>) -> String {
    let bl = determine_block_width(col_sizes);
    let ret = repeat_char(" ".to_string(), bl);

    return ret;
}

pub fn make_dark_print(v_lines:Vec<String>, fg: color::Rgb, bg: color::Rgb) {
    for line in v_lines {
        print!("{}{}",color::Fg(fg), color::Bg(bg));
        print!("{}",line);
        print!("{}\n",style::Reset);
    }
}

// // make the dark sting (the alternate fro single report)
// pub fn make_dark_print_single(first: &str, second: &str, fg: color::Rgb, bg:color::Rgb){
//     print!("{}{}",color::Fg(fg), color::Bg(bg));
//     print!("{} {}",first,second);
//     print!("{}\n",style::Reset);
// }

// make the header line
pub fn make_heading(col_sizes: &Vec<usize>, headers: Vec<&str>, colors: &Colors, heading: &str) {
    let fg = colors.color_complete_orphan;
    
    print!("\n");
    let message = heading.to_string();
    feedback(Feedback::Info, message);
    print!("\n{}", color::Fg(fg));
    
    for i in 0..headers.len() {
        let h = justify(headers[i].to_string(), col_sizes[i], Justify::Left);
        underline_string(h);
        print!(" ");
    }    
    print!("{}\n", style::Reset);
}

pub fn make_overdue(settings: &SettingsMap){
    let res_fg_overdue = settings.get_color("color_overdue");
    if res_fg_overdue.is_err() {
        let message = "Problems retrieving color from settings".to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }
    let res_bg_overdue = settings.get_color("color_black_bg");
    if res_bg_overdue.is_err() {
        let message = "Problems retrieving color from settings".to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }
    let fgov = res_fg_overdue.unwrap();
    let bgov = res_bg_overdue.unwrap();

    print!("{}{}", color::Fg(fgov), color::Bg(bgov));
}

pub fn make_print(v_lines:Vec<String>, fg: color::Rgb) {
    for line in v_lines {
        print!("{}",color::Fg(fg));
        print!("{}",line);
        print!("{}\n",style::Reset);
    }
}

pub fn print_description_line(col_sizes: &Vec<usize>, task: &Task) {
    let now = lts_now();

    let id = justify(task.id.unwrap().to_string(), col_sizes[0], Justify::Right);
    print!("{} ",id);

    match task.start {
        Some(secs) => {
            print!("{} ",lts_to_date_string(secs));
            let diff = now - secs;
            print!("{} ",align_timeframe(diff));
        }
        None => {
            print!("{} ",repeat_char(" ".to_string(),col_sizes[1]));                              
            print!("{} ",repeat_char(" ".to_string(),col_sizes[2]));                              
        }
    }
    
    // Insert the Recur column here
    match task.clone().recur {
        Some(r) => {
            let recur = justify(r, col_sizes[3], Justify::Center);
            print!("{} ",recur);
        }
        None => {
            print!("{} ",repeat_char(" ".to_string(),col_sizes[3]));                              
        }
    }

    match task.due {
        Some(secs) => {
            print!("{} ",lts_to_date_string(secs));
        }
        None => {
            print!("{} ",repeat_char(" ".to_string(),col_sizes[4]));                              
        }
    }
    
    let desc = justify(task.clone().description, col_sizes[5], Justify::Left);
    print!("{}{}\n",desc, style::Reset);
}

// my active report
pub fn report_active(colors: &Colors, settings: &SettingsMap, pend: &List) -> Result<(),&'static str> {
    let mut col_sizes = vec![3,10,7,5,10];
    let headers = vec!["ID", "Started", "Active", "Recur", "Due", "Description" ];
    let mut tasks: Vec<Task> = Vec::new();
    let mut v_desc: Vec<String> = Vec::new();
    let mut max_col: usize = 0;

    // lets get the set of tasks
    for t in pend.list.clone() {
        if t.is_active() || t.is_overdue()  {
            tasks.push(t.clone());
            v_desc.push(t.description.clone());
            let l1 = t.description.len();
            if l1 > max_col {
                max_col = l1;
            }
            if t.ann.len() > 0 {
                for a in t.ann {
                    let line = lts_to_date_string(a.date) + " " + &a.desc;
                    v_desc.push(line.clone());
                    if line.len() > max_col {
                        max_col = line.len();
                    }
                }
            }
        }
    }

    // add max_col to col_sizes with two spaces
    col_sizes.push(max_col + 2);
    let mut total_width = 0;
    for s in col_sizes.clone() {
        total_width += s;
    }
    // add the separator spaces
    total_width += col_sizes.len() - 1;

    // Width problem
    let width = settings.get_integer("useTerminalWidthOf");
    if total_width > width.unwrap() as usize {
        return Err("We have the width problem");
    }

    // do we have anything
    if tasks.len() == 0 {
        return Err("no matches");
    }

    // lets sort the wanted vector to due date if some
    tasks.sort_by(|a, b| {
        match &a.due {
            Some(secs) => {
                if b.due.is_none() {
                    return Ordering::Less;
                }
                return secs.cmp(&b.due.unwrap())
            }
            None => {
                if b.due.is_some() {
                    return Ordering::Greater;
                }
                return Ordering::Equal;
            }
        }
    });

    format_report_active(&col_sizes, headers, &tasks, colors, &settings);

    Ok(())
}

pub fn report_all_pending(pend: &List, colors: Colors, settings: &SettingsMap) -> Result<(), &'static str> {
    let mut col_sizes = vec![3,7,11,7];
    let headers = vec!["ID", "Age", "Tags", "Due", "Description" ];
    let mut tasks: Vec<Task> = Vec::new();
    let mut v_desc: Vec<String> = Vec::new();
    let mut max_col: usize = 0;

    // check for lengths of description
    for t in pend.list.clone() {
        // we decided not to want templates
        if t.is_parent(){
            continue;
        }
        tasks.push(t.clone());
        v_desc.push(t.description.clone());
        let l1 = t.description.clone().len();
        if l1 > max_col {
            max_col = l1;
        }

        if t.ann.len() > 0 {
            for a in t.ann {
                let line = "  ".to_string() + &lts_to_date_string(a.date) + " " + &a.desc;
                v_desc.push(line.clone());
                if line.len() > max_col {
                    max_col = line.len();
                }
            }
        }
    }

    if tasks.len() == 0 {
        return Err("There are no tasks to be done!");
    }

    //lets sort this vector with the least due date (if it has) on top
    tasks.sort_by(|a,b| {
        match &a.due {
            Some(secs) => {
                if b.due.is_none() {
                    return Ordering::Less;
                }
                return secs.cmp(&b.due.unwrap())
            }
            None => {
                if b.due.is_some() {
                    return Ordering::Greater;
                }
                return Ordering::Equal;
            }
        }
    });

    // add max_col to col_sizes with two spaces
    col_sizes.push(max_col);
    let mut total_width = 0;
    for s in col_sizes.clone() {
        total_width += s;
    }
    // add the separator spaces
    total_width += col_sizes.len() - 1;

    // Width problem
    let width = settings.get_integer("useTerminalWidthOf");
    if total_width > width.unwrap() as usize {
        return Err("We have the width problem");
    }

    // do we have anything
    if tasks.len() == 0 {
        return Err("no matches");
    }

    let heading = "All Pending   (but without templates)";
    format_report_all_pending(&col_sizes, headers, &tasks, &colors, heading);

    Ok(())
}

pub fn report_completed(colors: &Colors, settings: &SettingsMap, comp: &List ) -> Result<(), &'static str> {
    let mut col_sizes = vec![8,2,7,16,11,10];
    let headers = vec!["UUIID", "St", "Age", "Duration", "Tags", "Completed", "Description" ];
    let mut tasks: Vec<Task> = Vec::new();
    let mut v_desc: Vec<String> = Vec::new();
    let mut max_col: usize = 0;


    // check for lengths of description
    for t in comp.list.clone() {
        tasks.push(t.clone());
        v_desc.push(t.description.clone());
        let l1 = t.description.clone().len();
        if l1 > max_col {
            max_col = l1;
        }

        if t.ann.len() > 0 {
            for a in t.ann {
                let line = "  ".to_string() + &lts_to_date_string(a.date) + " " + &a.desc;
                v_desc.push(line.clone());
                if line.len() > max_col {
                    max_col = line.len();
                }
            }
        }
    }

    // add max_col to col_sizes with two spaces
    // col_sizes.push(max_col + 2);
    col_sizes.push(max_col);
    let mut total_width = 0;
    for s in col_sizes.clone() {
        total_width += s;
    }
    // add the separator spaces
    total_width += col_sizes.len() - 1;

    // Width problem
    let width = settings.get_integer("useTerminalWidthOf");
    if total_width > width.unwrap() as usize {
        return Err("We have the width problem");
    }

    // do we have anything
    if tasks.len() == 0 {
        return Err("no matches");
    }

    format_report_completed(&col_sizes, headers, &tasks, colors, "Completed");

    Ok(())
}

pub fn report_recurring(colors: &Colors, settings: &SettingsMap, pend: &List ) -> Result<(), &'static str> {
    let mut col_sizes = vec![3,8,7,4];
    let headers = vec!["ID", "UUIID", "Age", "Tags", "Description" ];
    let mut tasks: Vec<Task> = Vec::new();
    let mut v_desc: Vec<String> = Vec::new();
    let mut max_col: usize = 0;


    // check for lengths of description and only get parents
    for t in pend.list.clone() {
        if t.is_parent() {
            tasks.push(t.clone());
            v_desc.push(t.description.clone());
            let l1 = t.description.clone().len();
            if l1 > max_col {
                max_col = l1;
            }

            if t.ann.len() > 0 {
                for a in t.ann {
                    let line = "  ".to_string() + &lts_to_date_string(a.date) + " " + &a.desc;
                    v_desc.push(line.clone());
                    if line.len() > max_col {
                        max_col = line.len();
                    }
                }
            }
        }
    }

    // add max_col to col_sizes with two spaces
    col_sizes.push(max_col);
    let mut total_width = 0;
    for s in col_sizes.clone() {
        total_width += s;
    }
    // add the separator spaces
    total_width += col_sizes.len() - 1;

    // Width problem
    let width = settings.get_integer("useTerminalWidthOf");
    if total_width > width.unwrap() as usize {
        return Err("We have the width problem");
    }

    // do we have anything
    if tasks.len() == 0 {
        return Err("no matches");
    }

    format_report_recurring(&col_sizes, headers, &tasks, colors, "Recurring");

    Ok(())
}

pub fn report_search(args: &Vec<String>, colors: &Colors, settings: &SettingsMap, all_tasks: &List ) -> Result<(), &'static str> {
    let mut col_sizes = vec![3,2,8,7,4,10];
    let headers = vec!["ID", "St", "UUIID", "Age", "Tags", "Done", "Description" ];
    let mut tasks: Vec<Task> = Vec::new();
    let mut v_desc: Vec<String> = Vec::new();
    let mut max_col: usize = 0;
    let mut found:bool;

    // is there anything
    if args.len() < 3 {
        return Err("No search term provided");
    }
    let term = args.get(2).unwrap().as_str();

    // lets get the set of tasks
    for t in all_tasks.list.clone() {
        found = false;

        // search all descriptions, annotations and tags
        if t.is_tagged() {
            for tag in t.clone().tags {
                if tag.contains(term) {
                    found = true;
                }
            }
        }
        if t.description.contains(term) {
            found = true;
        }
        if t.is_annotated() {
            for anno in t.clone().ann {
                if anno.desc.contains(term) {
                    found = true;
                }
            }
        }

        // add and do some measuring
        if found {
            tasks.push(t.clone());

            let l1 = t.description.len();
            if l1 > max_col {
                max_col = l1;
            }
            if t.ann.len() > 0 {
                for a in t.ann {
                    let line = lts_to_date_string(a.date) + " " + &a.desc;
                    v_desc.push(line.clone());
                    if line.len() > max_col {
                        max_col = line.len();
                    }
                }
            }
        }
    }

    // add max_col to col_sizes with two spaces
    col_sizes.push(max_col + 2);
    let mut total_width = 0;
    for s in col_sizes.clone() {
        total_width += s;
    }
    // add the separator spaces
    total_width += col_sizes.len() - 1;

    // Width problem
    let width = settings.get_integer("useTerminalWidthOf");
    if total_width > width.unwrap() as usize {
        return Err("We have the width problem");
    }

    // do we have anything
    if tasks.len() == 0 {
        return Err("no matches");
    }

    let heading = format!("All Search results for:   {}",term);
    format_report_search(&col_sizes, headers, &tasks, colors, &heading);

    Ok(())
}

// show a single id report 'lets hardcode these variables'
pub fn report_single(settings: &SettingsMap, colors: &Colors, task: &Task ) -> Result<(), &'static str> {
    let mut col_sizes:Vec<usize> = Vec::new();
    let headers = vec![ "Name", "Value" ];
    let mut first_col:Vec<String>  = Vec::new(); 
    let mut second_col:Vec<String> = Vec::new(); 
    let mut diff:i64;
    let mut max_second_col:usize = 0;
    let mut max_first_col:usize  = 0;
    let now = lts_now();

    // ID
    first_col.push("ID".to_string());
    match task.id {
        Some(i) => {
            second_col.push(i.to_string());
        }
        None => {
            second_col.push("-".to_string());
        }
    }
    
    // Description
    first_col.push("Description".to_string());
    second_col.push(task.clone().description.to_string());
    
    // & Annotations
    for anno in task.clone().ann {
        first_col.push("".to_string());
        let line = lts_to_date_time_string(anno.date) + " " + &anno.desc;
        if line.len() + 2 > max_second_col {
            max_second_col = line.len() + 2;
        }
        second_col.push(line);
    }

    // Status
    first_col.push("Status".to_string());
    second_col.push(task.status.text().to_string());

    // Recurrence
    if task.has_recur(){
        first_col.push("Recurrence".to_string());
        second_col.push(task.clone().recur.unwrap().to_string());
    }
    
    // Parent
    if task.is_child(){
        first_col.push("Parent task".to_string());
        second_col.push(task.clone().parent.unwrap().to_string());
    }
    
    // Prodigy
    if task.has_prodigy(){
        first_col.push("Prodigy".to_string());
        second_col.push(task.clone().prodigy.unwrap().to_string());
    }
    
    // Recurrence type
    if task.has_recur(){
        first_col.push("Recurrence type".to_string());
        second_col.push(task.clone().rtype.unwrap().text().to_string());
    }
    
    // Entered
    first_col.push("Entered".to_string());
    diff = now - task.entry;
    let second = lts_to_date_time_string(task.entry.clone()) + format!(" ({})",make_timetracking_timeframe(diff)).as_str(); 
    second_col.push(second);
    

    // Waiting until
    if task.has_wait(){
        first_col.push("Waiting until".to_string());
        second_col.push(lts_to_date_time_string(task.clone().wait.unwrap()));
    }

    // Start
    if task.has_start(){
        first_col.push("Start".to_string());
        second_col.push(lts_to_date_time_string(task.clone().start.unwrap()));
    }
    
    // Due
    if task.has_due(){
        first_col.push("Due".to_string());
        second_col.push(lts_to_date_time_string(task.clone().due.unwrap()));
    }

    // End
    if task.is_complete() {
        first_col.push("End".to_string());
        diff = now - task.end.unwrap();
        let second = lts_to_date_time_string(task.end.clone().unwrap()) + format!(" ({})",make_timetracking_timeframe(diff)).as_str(); 
        second_col.push(second);
    }
    
    // Tags
    if task.is_tagged() {
        first_col.push("Tags".to_string());
        let mut vecco = "".to_string();
        for tag in task.tags.clone() {
            vecco.push_str(&tag);
            vecco.push_str(" ");
        }
        second_col.push(vecco.trim().to_string());
    }
    
    // Virtual tags   -> there should always be virtual tags
    first_col.push("Virtual tags".to_string());
    let mut vecco = "".to_string();
    for tag in task.virtual_tags.clone() {
        let t = tag.text().to_uppercase();
        vecco.push_str(&t);
        vecco.push_str(" ");
    }
    second_col.push(vecco.trim().to_string());
    
    // UUIID
    first_col.push("UUIID".to_string());
    let uuiid_int = format!("{}    ({})",task.uuiid.clone(), task.clone().uuiid_int.to_string());
    second_col.push(uuiid_int);
    
    // Timetracking
    if task.timetrackingseconds > 0 {
        first_col.push("Timetracking".to_string());
        vecco = "   ".to_string() + &make_timetracking_string(task.timetrackingseconds);
        second_col.push(task.timetrackingseconds.to_string() + &vecco);
    }

    // find max of both columns 
    for i in 0..first_col.len() {
        let len1 = first_col[i].len();
        if len1 > max_first_col {
            max_first_col = len1;
        }

        let len2 = second_col[i].len();
        if len2 > max_second_col {
            max_second_col = len2;
        }
    }
    col_sizes.push(max_first_col);
    col_sizes.push(max_second_col);

    // Width problem
    let sum = col_sizes[0] + col_sizes[1] + 1;
    let width = settings.get_integer("useTerminalWidthOf");
    if sum > width.unwrap() as usize {
        return Err("We have the width problem");
    }

    let lines = vec![first_col.clone(), second_col.clone()];
    format_report_single(&col_sizes, headers, lines, &colors, &task);

    Ok(())
}

pub fn report_waiting(colors: &Colors, settings: &SettingsMap, pend: &List ) -> Result<(), &'static str> {
    let mut col_sizes = vec![3,8,7,4,7];
    let headers = vec!["ID", "UUIID", "Age", "Tags", "Wait", "Description" ];
    let mut tasks: Vec<Task> = Vec::new();
    let mut v_desc: Vec<String> = Vec::new();
    let mut max_col: usize = 0;


    // check for lengths of description and only get parents
    for t in pend.list.clone() {
        if t.is_waiting() {
            tasks.push(t.clone());
            v_desc.push(t.description.clone());
            let l1 = t.description.clone().len();
            if l1 > max_col {
                max_col = l1;
            }
    
            if t.ann.len() > 0 {
                for a in t.ann {
                    let line = "  ".to_string() + &lts_to_date_string(a.date) + " " + &a.desc;
                    v_desc.push(line.clone());
                    if line.len() > max_col {
                        max_col = line.len();
                    }
                }
            }
        }
    }

    //lets sort this vector with the least wait date on bottom (basically I have reveresed the order)
    tasks.sort_by(|a,b| {
        let ans =  a.wait.unwrap().cmp(&b.wait.unwrap());
        match ans {
            Ordering::Less => {
                return Ordering::Greater 
            }
            Ordering::Equal => {
                return Ordering::Equal 
            }
            Ordering::Greater => {
                return Ordering::Less 
            }
        }
    });

    // add max_col to col_sizes with two spaces
    col_sizes.push(max_col);
    let mut total_width = 0;
    for s in col_sizes.clone() {
        total_width += s;
    }
    // add the separator spaces
    total_width += col_sizes.len() - 1;

    // Width problem
    let width = settings.get_integer("useTerminalWidthOf");
    if total_width > width.unwrap() as usize {
        return Err("We have the width problem");
    }

    // do we have anything
    if tasks.len() == 0 {
        return Err("no matches");
    }

    format_report_waiting(&col_sizes, headers, &tasks, colors, "Waiting");

    Ok(())
}

// show Nag
pub fn show_nag(settings: &SettingsMap) {
    let show = settings.get_bool("showNag");
    if show.is_err(){
        let message = "Problems retrieving bool 'showNag' from settings".to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }
    if show.unwrap() {
        let line = settings.map.get("nag").unwrap().to_string() + "\n";
        to_orange_feedback(&line);
    }
}

// function to return a message painted with fg and optional background
pub fn to_color_message(fg: color::Rgb, bg: Option<color::Rgb>, line: &str) {
    match bg {
        Some(c) => {
            print!("{}{}",color::Fg(fg), color::Bg(c));
            print!("{}",line);
            print!("{}",style::Reset);
        }

        None => {
            print!("{}",color::Fg(fg));
            print!("{}",line);
            print!("{}",style::Reset);
        }
    }
}

// my to Orange Feedback Message
pub fn to_orange_feedback(line: &str) {
    let fg = COLOR_ORANGE;
    let bg:Option<color::Rgb> = None;
    to_color_message(fg, bg, line);
}














// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {

    
    // // #[ignore]
    // #[test]
    // fn t001_report_single() {
    //     let line = "description:how do i get the konsole that i have now\tdue:1658513756\t\
    //                     entry:1658513756\tstart:1658513756\tstatus:pending\tuuiid:0x0011";
    //     let vec2:Vec<_> = line.split("\t").collect();
    //     let task = make_task(vec2);
    //     // let hexi = task.unwrap().uuiid_int




    // }







}// end of tests



