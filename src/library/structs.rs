/*
Most of my odd structs are in here.
2022.08.09      Sven Ponelat

*/




use std::collections::{BTreeSet, BTreeMap};
use termion::{color, style};
use crate::library::settings::*;
use crate::library::my_utils::*;
use chrono::prelude::*;
use substring::Substring;
use std::path::Path;
use std::fs::*;
// use std::fs::remove_file;
use std::io::{Write, LineWriter};
use std::process::exit;
use std::fmt::{Debug};
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use std::time::{UNIX_EPOCH, Duration};
use std::io::{BufRead, BufReader};



// Colors
pub struct Colors {
    pub orange_feedback: color::Rgb,
}

impl Colors {
    pub fn new() -> Colors {
        Colors { 
            orange_feedback: color::Rgb (255,255,255) 
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
    let orange_feedback = settings.get_color("color_general_orange");
    if orange_feedback.is_err(){
        let message = format!("Error in retrieving color from settings.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    ret.orange_feedback = orange_feedback.unwrap();

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






}//end of tests
