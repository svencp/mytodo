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


// show a single id report 'lets hardcode these variables'
pub fn report_single(width: i64, colors: Colors, task: Task ) -> Result<(), &'static str> {
    let mut b_vec:Vec<Vec<String>> = Vec::new();
    let mut first = "Name".to_string();
    let mut second = "Value".to_string();
    let mut diff = 0 as i64;
    let now = lts_now();
    let mut date_string = "".to_string();
    let mut vec = vec![ first , second ];
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
    
    // Description
    first = "Description".to_string();
    second = task.description;
    b_vec.push(vec![first,second]);
    
    // Status
    first = "Status".to_string();
    second = task.status.text().to_string();
    b_vec.push(vec![first,second]);
    
    // Recurrence
    match task.recur {
        Some(i) => {
            first = "Recurrence".to_string();
            second = i.to_string();
            b_vec.push(vec![first,second]);
        }
        None => {
        }
    }
    
    // Parent
    match task.parent {
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
    match task.rtype {
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
    second = lts_to_date_time_string(task.entry) + format!(" ({})",make_timetracking_timeframe(diff)).as_str(); 
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
            for tag in task.tags {
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
            for tag in task.virtual_tags {
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
    second = task.uuiid;
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
    
    // format_report_single(width, colors);
    


    
    
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



