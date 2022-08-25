/*
Most of my odd structs are in here.
2022.08.09      Sven Ponelat

*/

use std::collections::{BTreeSet, BTreeMap};
use termion::{color, style};
use crate::library::settings::*;
use crate::library::my_utils::*;
use crate::library::lts::*;
use std::process::exit;
use std::time::{UNIX_EPOCH, Duration};
use chrono::*;
use chronoutil::*;


pub const DAY_SECS: i64          = 86400;
pub const WEEK_SECS: i64         = 604800;



// Colors
pub struct Colors {
    pub color_active_bg: color::Rgb,                            // Orange
    pub color_black_bg: color::Rgb,                             // Black
    pub color_complete_orphan: color::Rgb,                      // White
    pub color_feedback_orange: color::Rgb,                      // Orange
    pub color_recur_chain_fg: color::Rgb,                       // Light Blue
    pub color_recur_period_fg: color::Rgb,                      // Dark Blue
    pub color_tagged: color::Rgb,                               // Dark Green
    pub color_overdue: color::Rgb,                               // Dark Red
}

impl Colors {
    pub fn new() -> Colors {
        Colors { 
            color_active_bg: color::Rgb (255,255,255),
            color_black_bg: color::Rgb (255,255,255),               
            color_complete_orphan: color::Rgb (255,255,255),        
            color_feedback_orange: color::Rgb (255,255,255),
            color_recur_chain_fg: color::Rgb (255,255,255),
            color_recur_period_fg: color::Rgb (255,255,255),
            color_tagged: color::Rgb (255,255,255),
            color_overdue: color::Rgb (255,255,255),
        }
    }

    



} // end of impl Colors




pub struct Hdeci {
    pub set: BTreeSet<i64>,
}


impl Hdeci {
    pub fn new() -> Hdeci {
        Hdeci { 
            set: BTreeSet::new(),
        }
    }


    pub fn add(&mut self, num: i64) {
        self.set.insert(num);
    } 


    // gets the next avaialable integer
    pub fn get_next_hexidecimal(&self) -> i64 {
        let mut index = 0;
        let mut found = false;
    
        for _i in 0..self.set.len() {
            index += 1;
            if ! self.set.contains(&index){
                found = true;
                break;
            } 
        }
    
        if ! found {
            let ret = index + 1;
            return ret;
        }
    
        return index;
    }

} // end of impl


pub struct RecurTerm {
    pub term: String,
    pub numeric: i64,
    pub duration: char,
    pub is_relative: bool,
}

impl RecurTerm {

    fn instant() -> RecurTerm {
        RecurTerm {
            term: "".to_string(),
            numeric: 0,
            duration: 'd',
            is_relative: false,
        }
    }

    pub fn multiply_from_timestring(&self, ts: i64, multiplier: i64) -> i64 {
        let ret:i64;
        let mut addition:i64 = 0;

        let product = (self.numeric * multiplier) as i32;
        let time_ndt = NaiveDateTime::from_timestamp(ts, 0);

        match self.duration {
            'd' => {
                addition = product as i64 * DAY_SECS;
            }
            'w' => {
                addition = product as i64 * WEEK_SECS;
            }
            'm' => {
                let delta = RelativeDuration::months(product);
                let ndt = time_ndt + delta;
                return ndt.timestamp();
            }
            'y' => {
                let delta = RelativeDuration::years(product);
                let ndt = time_ndt + delta;
                return ndt.timestamp();
            }
            _ => {
                // should never get here
            }
        }

        ret = ts + addition;
        return ret
    }

    pub fn new(term: &str) -> Result<RecurTerm, &'static str> {
        let mut ret = RecurTerm::instant();

        if ! term.starts_with("+") {
            return Err("Term doesn't start with + ");
        }
    
        let str = term.replace("+", "");
        let mut n_arr:Vec<char> = Vec::new();
        let mut c_arr:Vec<char> = Vec::new();
        let str_arr: Vec<char> = str.chars().collect();
        for c in str_arr {
            if c.is_numeric() {
                n_arr.push(c);
                continue;
            }
            c_arr.push(c);
        }
    
        // is it a number
        let s_num: String = n_arr.iter().collect();
        let res_num = s_num.parse::<i64>();
        if res_num.is_err() {
            return Err("recur_term number could not be parsed");
        }
        ret.numeric = res_num.unwrap();
        
        // has the term got the right chars (only d,w,m,y)
        if c_arr.len() > 1 {
            return Err("Too many characters in duration");
        }
        if c_arr.len() < 1 {
            return Err("No duration symbol given");
        }
    
        // assign duration
        ret.duration = c_arr[0];

        match c_arr[0] {
            'd' => {
                ret.term = "+".to_string() + &ret.numeric.to_string() + "d";
                ret.is_relative = false;
            }
            'w' => {
                ret.term = "+".to_string() + &ret.numeric.to_string() + "w";
                ret.is_relative = false;
            }
            'm' => {
                ret.term = "+".to_string() + &ret.numeric.to_string() + "m";
                ret.is_relative = true;
            }
            'y' => {
                ret.term = "+".to_string() + &ret.numeric.to_string() + "y";
                ret.is_relative = true;
            }
            _ => {
                return Err("Illegal duration symbol");
            }
        }

        Ok(ret)
    } // end of new()

    pub fn text(&self) -> String {
        return  self.term.to_string()
    }








} // end of impl



// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

pub fn hexidecimal_to_string(num: i64) -> String {
        // make it hex
        let str = format!("{:x}",num);

        // pad with leading zeros with up to six places
        let lead = format!("{:0>6}",str);
    
        // add the 0x
        let ret = "0x".to_string() + lead.as_str();
    
        return ret;
}

// load and return all my colors
pub fn load_colors(settings: &SettingsMap) -> Colors {
    let mut ret = Colors::new();

    let active_bg = settings.get_color("color_active_bg");
    if active_bg.is_err(){
        let message = format!("Error in retrieving color from settings.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    ret.color_active_bg = active_bg.unwrap();
    
    let black = settings.get_color("color_black_bg");
    if black.is_err(){
        let message = format!("Error in retrieving color from settings.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    ret.color_black_bg = black.unwrap();
    
    let orange_feedback = settings.get_color("color_feedback_orange");
    if orange_feedback.is_err(){
        let message = format!("Error in retrieving color from settings.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    ret.color_feedback_orange = orange_feedback.unwrap();
    
    let white = settings.get_color("color_complete_orphan");
    if white.is_err(){
        let message = format!("Error in retrieving color from settings.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    ret.color_complete_orphan = white.unwrap();
    
    let light_blue = settings.get_color("color_recur_chain_fg");
    if light_blue.is_err(){
        let message = format!("Error in retrieving color from settings.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    ret.color_recur_chain_fg = light_blue.unwrap();
    
    let dark_blue = settings.get_color("color_recur_period_fg");
    if dark_blue.is_err(){
        let message = format!("Error in retrieving color from settings.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    ret.color_recur_period_fg = dark_blue.unwrap();
    
    let dark_green = settings.get_color("color_tagged");
    if dark_green.is_err(){
        let message = format!("Error in retrieving color from settings.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    ret.color_tagged = dark_green.unwrap();
    
    let overdue = settings.get_color("color_overdue");
    if overdue.is_err(){
        let message = format!("Error in retrieving color from settings.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    ret.color_overdue = overdue.unwrap();

    return ret;
}





// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use crate::library::{enums::Status, lts::lts_to_date_time_string};

    use super::*;
    use std::{fs::copy};
    use substring::Substring;
    use std::fs::remove_file;

    
    // #[ignore]
    #[test]
    fn t001_new() {
        let mut seto = Hdeci::new();
        seto.add(7);
        seto.add(3);
        seto.add(1);
        seto.add(2);
        
        let next = seto.get_next_hexidecimal();
        assert_eq!(next,4);
        
        seto.add(4);
        let str1 = hexidecimal_to_string(seto.get_next_hexidecimal());
        assert_eq!(str1,"0x000005");
    }
    
    // #[ignore]
    #[test]
    fn t002_recur_term() {
        let term = RecurTerm::new("+3m").unwrap();
        assert_eq!(term.text(),"+3m");
        
        let term2 = RecurTerm::new("+3mo");
        assert_eq!(term2.is_err(),true);
        
        let date_str = "2000-01-01";
        let ts = lts_date_string_to_timestamp(date_str);
        
        // add 12 months
        let dts = term.multiply_from_timestring(ts.unwrap(), 4);
        let str = lts_to_date_time_string(dts);
        assert_eq!(str.substring(0, 10), "2001-01-01");
        
        let term3 = RecurTerm::new("+7d").unwrap();
        let date_str = "2000-01-01";
        let ts = lts_date_string_to_timestamp(date_str);
        let dts = term3.multiply_from_timestring(ts.unwrap(), 10);
        let str = lts_to_date_time_string(dts);
        assert_eq!(str.substring(0, 10), "2000-03-11");
        
    }






















}//end of tests
