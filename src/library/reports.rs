/*
    Module for everything to do with reports
    2022.08.12      Sven Ponelat

*/




use termion::{color, style};
use std::cmp::Ordering;
use std::process::exit;
use std::cmp;
use crate::library::enums::Rtype;
use crate::library::functions::*;
use crate::library::lts::*;
use crate::library::structs::*;
use crate::library::settings::*;
use crate::library::my_utils::*;
use crate::library::task::*;
use crate::library::list::*;













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

// get the color scheme according to virtual tags
pub fn get_colour_scheme(task: Task, colors: Colors) -> Vec<Option<color::Rgb>> {
    match task.is_active() {
        true => {
            let fg = Some(colors.color_complete_orphan);
            let bg = Some(colors.color_active_bg);
            return vec![fg,bg];
        }
        false => {
            match task.is_recurring() {
                true => {
                    match task.is_periodic() {
                        true => {
                            let fg = Some(colors.color_recur_period_fg);
                            let bg = None;
                            return vec![fg,bg];
                        }
                        false => {
                            let fg = Some(colors.color_recur_chain_fg);
                            let bg = None;
                            return vec![fg,bg];
                        }
                    }
                }
                false => {
                    match task.is_tagged() {
                        true => {
                            let fg = Some(colors.color_tagged);
                            let bg = None;
                            return vec![fg,bg];
                        }
                        false => {
                            let fg = Some(colors.color_complete_orphan);
                            let bg = None;
                            return vec![fg,bg];
                        }
                    }
                }
            }
        }
    }
}

// get the total of maximum column widths
pub fn get_max_col_widths(big: Vec<Vec<String>>) -> Result<Vec<usize>, &'static str> {
    let num_lines = big.len() as i64;
    let num_columns = big[0].len();
    let mut v_max:Vec<usize> = vec![0;num_columns];

    if num_lines == 0 {
        return Err("Cannot obtain maximum column width");
    }

    for lines in 0..big.len() {
        for col in 0..big[lines].len() {
            let len = big[lines][col].len();
            let prev = v_max[col];
            v_max[col] = cmp::max(len, prev);
        }
    }

    Ok(v_max)
}

pub fn format_report_active(col_sizes: &Vec<usize>, headers: Vec<&str>, tasks: &Vec<Task>, colors: &Colors, settings: &SettingsMap) {
    let fg   = colors.clone().color_complete_orphan;
    let bg   = colors.clone().color_active_bg;
    let fgov = colors.clone().color_overdue;
    let bgov = colors.clone().color_black_bg;
    let anno_block = make_annotation_block(col_sizes);

    // make_heading(col_sizes,headers,settings);
    make_heading(col_sizes,headers,colors);
    
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
                    let d = justify(desc.clone(), col_sizes[4]-2, Justify::Left);
                    print!("{}   {}",anno_block, d);
                    print!("{}\n",style::Reset);
                }
            }
        }
    }

    print!("\n\n");
}

pub fn format_report_completed(col_sizes: &Vec<usize>, headers: Vec<&str>, tasks: &Vec<Task>, colors: &Colors ,settings: &SettingsMap) {
    let normal_fg   = colors.clone().color_complete_orphan;
    let tagged_fg   = colors.clone().color_tagged;
    let rperiod_fg  = colors.clone().color_recur_period_fg;
    let rchained_fg = colors.clone().color_recur_chain_fg;
    let black_bg    = colors.clone().color_black_bg;
    let anno_block = make_annotation_block(col_sizes);

    let mut remainder: i64;
    let mut index: i64 = 1;
    let mut v_lines:Vec<String>;

    make_heading(col_sizes, headers, colors);

    for task in tasks {
        index += 1;
        remainder = index % 2;

        v_lines = get_task_lines_completed(col_sizes, &anno_block, &task);

        match remainder {
            // dark black background
            0 => {
                match task.is_recurring() {
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
                match task.is_recurring() {
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
        }
    }




    // for i in 0..tasks.len() {
    //     let remainder = i % 2;
    //     match remainder {
    //         // dark black background
    //         0 => {
    //             match tasks[i].cloned().is_recurring() {
    //                 true => {
    //                     match tasks.cloned()[i].rtype.unwrap() {
    //                         Rtype::Periodic => {

    //                         }
    //                         Rtype::Chained => {

    //                         }
    //                     }
    //                 }
    //                 false => {

    //                 }
    //             }
    //         }
    //         // normal background
    //         _ => {

    //         }
    //     }
    // }






    print!("\n\n")
}

pub fn format_report_single(col_sizes: &Vec<usize>, headers: Vec<&str>, lines: Vec<Vec<String>>, colors: &Colors, task: &Task ) {
    let normal_fg   = colors.clone().color_complete_orphan;
    let tagged_fg   = colors.clone().color_tagged;
    let rperiod_fg  = colors.clone().color_recur_period_fg;
    let rchained_fg = colors.clone().color_recur_chain_fg;
    let active_bg    = colors.clone().color_active_bg;
    let black_bg    = colors.clone().color_black_bg;
    let anno_block = make_annotation_block(col_sizes);
    let mut index: usize = 0;

    let mut remainder: i64;
    let mut index: i64 = 1;
    let mut v_lines:Vec<String>;

    // make_heading(col_sizes,headers,settings);
    make_heading(col_sizes,headers,colors);
    let num_lines = lines.clone()[1].len();

    for i in 0..num_lines {
        if lines[0][i].clone().starts_with("Desc") || lines[0][i].clone().len() == 0 {
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








    
    // let fg = colors.color_complete_orphan;
    // let bg = colors.color_black_bg;

    // // lets do the header
    // print!("\n\n");
    // let mut first = justify(big[0][0].clone(), cols[0], Justify::Left);
    // underline_string(first);
    // print!(" ");
    // let mut second = justify(big[0][1].clone(), cols[1], Justify::Left);
    // underline_string(second);
    // print!("\n");
    
    // // 222222222222222222222222222222222222222
    // first = justify(big[1][0].clone(), cols[0], Justify::Left);
    // second = justify(big[1][1].clone(), cols[1], Justify::Left);
    // make_dark_print_single(&first,&second,fg,bg);

    // // 3333333333333333333333333333333333333333
    // let fgbg = get_colour_scheme(task,colors);

    // for i in 0..desc.len() {
    //     match i {
    //         0 => {
    //             match fgbg.get(1).unwrap() {
    //                 Some(bg) => {
    //                     let d = justify("Description".to_string(), cols[0], Justify::Left);
    //                     print!("{} ",d);
    //                     let value = desc[i][0].clone();        
    //                     let v = justify(value, cols[1], Justify::Left);
    //                     print!("{}{}{}",color::Fg(fgbg[0].unwrap()), color::Bg(*bg),v);
    //                     print!("{}\n",style::Reset);
    //                 }
    //                 None => {
    //                     let d = justify("Description".to_string(), cols[0], Justify::Left);
    //                     print!("{} ",d);
    //                     let value = desc[i][0].clone();  
    //                     let v = justify(value, cols[1], Justify::Left);
    //                     print!("{}{}",color::Fg(fgbg[0].unwrap()),v);
    //                     print!("{}\n",style::Reset);
    //                 }
    //             }
    //         }

    //         _ => {
    //             match fgbg.get(1).unwrap() {
    //                 Some(bg) => {
    //                     let d = repeat_char(" ".to_string(), cols[0]);
    //                     print!("{} ",d);
    //                     let value = desc[i][0].clone(); 
    //                     // remember to take 2 spaces away again for the tab
    //                     let v = justify(value, cols[1]-2, Justify::Left);
    //                     print!("{}{}  {}",color::Fg(fgbg[0].unwrap()), color::Bg(*bg),v);
    //                     print!("{}\n",style::Reset);
    //                 }
    //                 None => {
    //                     let d = repeat_char(" ".to_string(), cols[0]);
    //                     print!("{} ",d);
    //                     let value = desc[i][0].clone(); 
    //                     // remember to take 2 spaces away again for the tab
    //                     let v = justify(value, cols[1]-2, Justify::Left);
    //                     print!("{}  {}",color::Fg(fgbg[0].unwrap()),v);
    //                     print!("{}\n",style::Reset);
    //                 }
    //             }
    //         }
    //     }
    // }

    // // 4444444444444444444444444444444444444444444444444444444444444444444444444 onwards
    // for i in 2..big.len() {
    //     first = justify(big[i][0].clone(), cols[0], Justify::Left);
    //     second = justify(big[i][1].clone(), cols[1], Justify::Left);
        
    //     let remainder = i % 2;
    //     match remainder {
    //         0 => {
    //             make_dark_print_single(&first,&second,fg,bg);
    //         }
    //         _ => {
    //             make_print2(&first, &second, fg);
    //         }
    //     }
    // }
    // print!("\n\n");

    print!("\n\n");
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

// make the dark sting (the alternate fro single report)
pub fn make_dark_print_single(first: &str, second: &str, fg: color::Rgb, bg:color::Rgb){
    print!("{}{}",color::Fg(fg), color::Bg(bg));
    print!("{} {}",first,second);
    print!("{}\n",style::Reset);
}

// make the header line
pub fn make_heading(col_sizes: &Vec<usize>, headers: Vec<&str>, colors: &Colors) {
    let fg = colors.color_complete_orphan;
    print!("\n\n{}", color::Fg(fg));
    
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

// make the dark string (the alternate fro single report)
pub fn make_print2(first: &str, second: &str, fg: color::Rgb){
    print!("{}",color::Fg(fg));
    print!("{} {}",first,second);
    print!("{}\n",style::Reset);
}

// pub fn print_annotation_lines(block: String, col_sizes: &Vec<usize>, task: &Task) {





//     for ann in task.ann.clone() {
//         let desc = lts_to_date_string(ann.date) + " " + &ann.desc;
//         let d = justify(desc.clone(), col_sizes[4]-2, Justify::Left);
//         print!("{}   {}\n",block, d);
//     }
//     print!("{}",style::Reset);
// }

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
    
    match task.due {
        Some(secs) => {
            print!("{} ",lts_to_date_string(secs));
        }
        None => {
            print!("{} ",repeat_char(" ".to_string(),col_sizes[3]));                              
        }
    }
    
    let desc = justify(task.clone().description, col_sizes[4], Justify::Left);
    print!("{}{}\n",desc, style::Reset);
    // print!("{}\n",desc);
}

// my active report
pub fn report_active(colors: &Colors, settings: &SettingsMap, pend: &List) -> Result<(),&'static str> {
    let mut col_sizes = vec![2,10,7,10];
    let headers = vec!["ID", "Started", "Active", "Due", "Description" ];
    let mut tasks: Vec<Task> = Vec::new();
    let mut v_desc: Vec<String> = Vec::new();
    let mut max_col: usize = 0;

    // lets get the set of tasks
    for t in pend.list.clone() {
        if t.is_active() || t.is_overdue() {
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

pub fn report_completed(colors: &Colors, settings: &SettingsMap, comp: &List ) -> Result<(), &'static str> {
    let mut col_sizes = vec![8,7,16,11,10];
    let headers = vec!["UUIID", "Age", "Duration", "Tags", "Completed", "Description" ];
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

    format_report_completed(&col_sizes, headers, &tasks, colors, &settings);

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
    let fg = crate::COLOR_ORANGE;
    let bg:Option<color::Rgb> = None;
    to_color_message(fg, bg, line);
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
    fn t001_report_single() {
        let line = "description:how do i get the konsole that i have now\tdue:1658513756\t\
                        entry:1658513756\tstart:1658513756\tstatus:pending\tuuiid:0x0011";
        let vec2:Vec<_> = line.split("\t").collect();
        let task = make_task(vec2);
        // let hexi = task.unwrap().uuiid_int




    }







}// end of tests



