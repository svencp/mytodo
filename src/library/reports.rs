/*
    Module for everything to do with reports
    2022.08.12      Sven Ponelat

*/




use termion::{color, style};
use std::process::exit;

use crate::library::functions::make_timetracking_string;
use crate::library::functions::make_timetracking_timeframe;
use crate::library::lts::lts_now;
use crate::library::lts::lts_to_date_time_string;
use crate::library::structs::*;
use crate::library::settings::*;
use crate::library::my_utils::*;
use crate::library::task::*;
use crate::library::list::*;













// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
pub fn color_test(colors: Colors) {
    let line1 = "This is my test of the orange_feedback color (123 000)";
    let fg = colors.color_feedback_orange;
    let bg:Option<color::Rgb> = None;
    to_color_message(fg, bg, line1);
}

// get the total of maximum column widths
pub fn get_max_col_widths(big: Vec<Vec<String>>) -> Result<Vec<usize>, &'static str> {
    let num_columns = big[0].len() as i64;
    let mut max:Vec<usize> = vec![0;num_columns as usize];
    // let mut total_max:i64 = 0;
    let mut col_num = 0;

    if big.len() == 0 {
        return Err("Cannot obtain maximum column width");
    }

    for item in big {
        col_num = 0;

        for column in item {
            let len = column.len();
            if len > max[col_num] {
                max[col_num] = len;
            }

            col_num += 1;
        }
    }

    Ok(max)
}

pub fn format_report_single(cols: Vec<usize>, big: Vec<Vec<String>>, desc: Vec<String>, task: Task, colors: Colors) {
    let mut col_num = 0 as i64;
    let mut fg = colors.color_complete_orphan;
    let mut bg:Option<color::Rgb> = None;

    // lets do the header
    let mut first = justify(big[0][0].clone(), cols[0], Justify::Left);
    underline_string(first);
    print!(" ");
    let mut second = justify(big[0][1].clone(), cols[1], Justify::Left);
    underline_string(second);
    print!("\n");
    
    // 222222222222222222222222222222222222222
    first = justify(big[1][0].clone(), cols[0], Justify::Left);
    second = justify(big[1][1].clone(), cols[1], Justify::Left);
    bg = Some(colors.color_black_bg);
    make_dark_print(&first,&second,fg,bg);

    // 3333333333333333333333333333333333333333
    let fgbg: Vec<Option<color::Rgb>> = 











let rr= 99;

    // for item in big {
    //     col_num = 0;

    //     for column in item {
    //         print!("{}",column);
    //         if col_num == 1 {
                
    //             print!("\n");
    //         }
    //         col_num += 1;
    //     }
    // }
    
}

// make the dark sting (the alternate fro single report)
pub fn make_dark_print(first: &str, second: &str, fg: color::Rgb, bg: Option<color::Rgb>){
    print!("{}{}",color::Fg(fg), color::Bg(bg.unwrap()));
    print!("{} {}",first,second);
    print!("{}\n",style::Reset);
}


// show a single id report 'lets hardcode these variables'
pub fn report_single(width: usize, colors: Colors, task: Task ) -> Result<(), &'static str> {
    let mut b_vec:Vec<Vec<String>> = Vec::new();
    let mut first = "Name".to_string();
    let mut second = "Value".to_string();
    let mut diff:i64;
    let now = lts_now();
    let mut desc: Vec<String> = Vec::new();
    let mut date_string = "".to_string();
    let vec = vec![ first , second ];
    b_vec.push(vec);
    
    // ID
    first = "ID".to_string();
    match task.id {
        Some(i) => {
            second = i.to_string();
        }
        None => {
            second = "-".to_string();
        }
    }
    b_vec.push(vec![first,second]);
    
    // lets put description into separate vector
    desc.push(task.description.clone());
    if task.ann.len() > 0 {
        let v_anno = task.ann.clone();
        for a in v_anno {
            let date = lts_to_date_time_string(a.date);
            let pusha = "  ".to_string() + &date + " " + &a.desc;
            desc.push(pusha);
        }
    }


    // // Description
    // first = "Description".to_string();
    // second = task.description.clone();
    // b_vec.push(vec![first,second]);
    
    // Status
    first = "Status".to_string();
    second = task.status.text().to_string();
    b_vec.push(vec![first,second]);
    
    // Recurrence
    match task.recur.clone() {
        Some(i) => {
            first = "Recurrence".to_string();
            second = i.to_string();
            b_vec.push(vec![first,second]);
        }
        None => {
        }
    }
    
    // Parent
    match task.parent.clone() {
        Some(h) => {
            first = "Parent task".to_string();
            second = h.to_string();
            b_vec.push(vec![first,second]);
        }
        None => {
        }
    }
    
    // Prodigy
    match task.prodigy {
        Some(h) => {
            first = "Prodigy".to_string();
            second = h.to_string();
            b_vec.push(vec![first,second]);
        }
        None => {
        }
    }
    
    // Recurrence type
    match task.rtype.clone() {
        Some(h) => {
            first = "Recurrence type".to_string();
            second = h.text().to_string();
            b_vec.push(vec![first,second]);
        }
        None => {
        }
    }
    
    // Entered
    first = "Entered".to_string();
    diff = now - task.entry;
    second = lts_to_date_time_string(task.entry.clone()) + format!(" ({})",make_timetracking_timeframe(diff)).as_str(); 
    b_vec.push(vec![first,second]);
    
    // Waiting until
    match task.wait {
        Some(h) => {
            first = "Waiting until".to_string();
            second = lts_to_date_time_string(h);
            b_vec.push(vec![first,second]);
        }
        None => {
        }
    }
    
    // Start
    match task.start {
        Some(h) => {
            first = "Start".to_string();
            second = lts_to_date_time_string(h);
            b_vec.push(vec![first,second]);
        }
        None => {
        }
    }
    
    // Due
    match task.due {
        Some(h) => {
            first = "Due".to_string();
            second = lts_to_date_time_string(h);
            b_vec.push(vec![first,second]);
        }
        None => {
        }
    }
    
    // End
    match task.end {
        Some(e) => {
            first = "End".to_string();
            diff = now - e;
            second = lts_to_date_time_string(e) + format!(" ({})",make_timetracking_timeframe(diff)).as_str(); 
            b_vec.push(vec![first,second]);
        }
        None => {
        }
    }
    
    // Tags
    match task.tags.len() {
        0 => {
        }
        _ => {
            first = "Tags".to_string();
            let mut vecco = "".to_string();
            for tag in task.tags.clone() {
                vecco.push_str(&tag);
                vecco.push_str(" ");
            }
            second = vecco.trim().to_string();
            b_vec.push(vec![first,second]);
            
        }
    }
    
    // Virtual tags
    match task.virtual_tags.len() {
        0 => {
        }
        _ => {
            first = "Virtual tags".to_string();
            let mut vecco = "".to_string();
            for tag in task.virtual_tags.clone() {
                let t = tag.text().to_uppercase();
                vecco.push_str(&t);
                vecco.push_str(" ");
            }
            second = vecco.trim().to_string();
            b_vec.push(vec![first,second]);
            
        }
    }
    
    // UUIID
    first = "UUIID".to_string();
    second = task.uuiid.clone();
    b_vec.push(vec![first,second]);
    
    // Timetracking
    match task.timetrackingseconds {
        0 => {
        }
        _ => {
            first = "Timetracking".to_string();
            let vecco = "   ".to_string() + &make_timetracking_string(task.timetrackingseconds);
            second = task.timetrackingseconds.to_string() + &vecco;
            b_vec.push(vec![first,second]);
        }
    }
    
    // b_vec is too small
    if b_vec.clone().len() < 4 {
        return Err("Cannot get 4 lines out of the task");
    }

    let res_max  = get_max_col_widths(b_vec.clone()); 
    if res_max.is_err() {
        return Err(res_max.err().unwrap());
    }

    // get total
    let mut total_max = 0;
    for num in res_max.clone().unwrap() {
        total_max += num;
    }

    // add the number of spaces
    let total_len = total_max + 1;

    // Check the width, code later if needed
    if total_len > width {
        return Err("We have the width problem");
    }
    
    format_report_single(res_max.unwrap(), b_vec, desc,  task, colors);

    
    
    println!("report single");
    Ok(())
}


// show Nag
pub fn show_nag(settings: &SettingsMap, colors: Colors) {
    let show = settings.get_bool("showNag");
    if show.is_err(){
        let message = "Problems retrieving bool 'showNag' from settings".to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }
    if show.unwrap() {
        let line = settings.map.get("nag").unwrap().to_string() + "\n";
        to_orange_feedback(&colors, &line);
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
pub fn to_orange_feedback(colors: &Colors, line: &str) {
    let fg = colors.color_feedback_orange;
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



