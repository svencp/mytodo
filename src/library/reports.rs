/*
    Module for everything to do with reports
    2022.08.12      Sven Ponelat

*/




use termion::{color, style};
use std::process::exit;
use std::cmp;
use crate::library::functions::make_timetracking_string;
use crate::library::functions::make_timetracking_timeframe;
use crate::library::lts::lts_now;
use crate::library::lts::lts_to_date_string;
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

pub fn format_report_single(cols: Vec<usize>, big: Vec<Vec<String>>, desc: Vec<Vec<String>>, task: Task, colors: Colors) {
    let fg = colors.color_complete_orphan;
    let bg = colors.color_black_bg;

    // lets do the header
    print!("\n\n");
    let mut first = justify(big[0][0].clone(), cols[0], Justify::Left);
    underline_string(first);
    print!(" ");
    let mut second = justify(big[0][1].clone(), cols[1], Justify::Left);
    underline_string(second);
    print!("\n");
    
    // 222222222222222222222222222222222222222
    first = justify(big[1][0].clone(), cols[0], Justify::Left);
    second = justify(big[1][1].clone(), cols[1], Justify::Left);
    // bg = Some(colors.color_black_bg);
    make_dark_print(&first,&second,fg,bg);

    // 3333333333333333333333333333333333333333
    let fgbg = get_colour_scheme(task,colors);

    for i in 0..desc.len() {
        match i {
            0 => {
                match fgbg.get(1).unwrap() {
                    Some(bg) => {
                        let d = justify("Description".to_string(), cols[0], Justify::Left);
                        print!("{} ",d);
                        let value = desc[i][0].clone();        
                        let v = justify(value, cols[1], Justify::Left);
                        print!("{}{}{}",color::Fg(fgbg[0].unwrap()), color::Bg(*bg),v);
                        print!("{}\n",style::Reset);
                    }
                    None => {
                        let d = justify("Description".to_string(), cols[0], Justify::Left);
                        print!("{} ",d);
                        let value = desc[i][0].clone();  
                        let v = justify(value, cols[1], Justify::Left);
                        print!("{}{}",color::Fg(fgbg[0].unwrap()),v);
                        print!("{}\n",style::Reset);
                    }
                }
            }

            _ => {
                match fgbg.get(1).unwrap() {
                    Some(bg) => {
                        let d = repeat_char(" ".to_string(), cols[0]);
                        print!("{} ",d);
                        let value = desc[i][0].clone(); 
                        // remember to take 2 spaces away again for the tab
                        let v = justify(value, cols[1]-2, Justify::Left);
                        print!("{}{}  {}",color::Fg(fgbg[0].unwrap()), color::Bg(*bg),v);
                        print!("{}\n",style::Reset);
                    }
                    None => {
                        let d = repeat_char(" ".to_string(), cols[0]);
                        print!("{} ",d);
                        let value = desc[i][0].clone(); 
                        // remember to take 2 spaces away again for the tab
                        let v = justify(value, cols[1]-2, Justify::Left);
                        print!("{}  {}",color::Fg(fgbg[0].unwrap()),v);
                        print!("{}\n",style::Reset);
                    }
                }
            }
        }
    }

    // 4444444444444444444444444444444444444444444444444444444444444444444444444 onwards
    for i in 2..big.len() {
        first = justify(big[i][0].clone(), cols[0], Justify::Left);
        second = justify(big[i][1].clone(), cols[1], Justify::Left);
        
        let remainder = i % 2;
        match remainder {
            0 => {
                make_dark_print(&first,&second,fg,bg);
            }
            _ => {
                make_print(&first, &second, fg);
            }
        }
    }
    print!("\n\n");
}

// make the dark sting (the alternate fro single report)
pub fn make_dark_print(first: &str, second: &str, fg: color::Rgb, bg:color::Rgb){
    print!("{}{}",color::Fg(fg), color::Bg(bg));
    print!("{} {}",first,second);
    print!("{}\n",style::Reset);
}

// make the dark sting (the alternate fro single report)
pub fn make_print(first: &str, second: &str, fg: color::Rgb){
    print!("{}",color::Fg(fg));
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
    let mut desc: Vec<Vec<String>> = Vec::new();
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
    
    // lets put description into separate vector  && only give the date (not the time as well)
    desc.push(vec![task.description.clone()]);
    if task.ann.len() > 0 {
        let v_anno = task.ann.clone();
        for a in v_anno {
            let date = lts_to_date_string(a.date);
            // let date = lts_to_date_time_string(a.date);
            let pusha = "".to_string() + &date + " " + &a.desc;
            desc.push(vec![pusha]);
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

    // get max column widths
    let result_max_desc = get_max_col_widths(desc.clone()); 
    if result_max_desc.is_err() {
        return Err(result_max_desc.err().unwrap());
    }
    let result_max_bvec = get_max_col_widths(b_vec.clone()); 
    if result_max_bvec.is_err() {
        return Err(result_max_bvec.err().unwrap());
    }
    // and combine totals; if annotated and a tab of 2 spaces
    let first_col = result_max_bvec.clone().unwrap()[0];
    let mut desc_2nd_col = result_max_desc.clone().unwrap()[0];
    if task.is_annotated(){
        desc_2nd_col += 2;
    }
    let second_col = cmp::max(desc_2nd_col,result_max_bvec.clone().unwrap()[1]);
    let col_sizes = vec![first_col,second_col];

    // get total
    let mut total_max = 0;
    for num in col_sizes.clone() {
        total_max += num;
    }

    // add the number of spaces
    let total_len = total_max + 1;

    // Check the width, code later if needed
    if total_len > width {
        return Err("We have the width problem");
    }
    
    format_report_single(col_sizes, b_vec, desc.clone(),  task, colors);

    
    
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



