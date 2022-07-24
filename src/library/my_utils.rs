/*  A file to keep functions that I use.

    2022.06.30   Sven Ponelat

*/


use termion::{color, style};
use std::time::SystemTime;


pub const MY_NORMAL_GRAY: color::Rgb =  color::Rgb (177, 177, 177);
pub const MY_YELLOW: color::Rgb      =  color::Rgb (253, 185, 73);
pub const MY_WHITE: color::Rgb       =  color::Rgb (230, 230, 230);
pub const MY_WHITER: color::Rgb      =  color::Rgb (160, 160, 160);





#[allow(dead_code)]
pub enum Feedback{
    Info,
    Warning,
    Error
}


#[allow(dead_code)]
pub enum Justify {
    Left,
    Center,
    Right
}


// A function to give command line feedback to situations such as errors or warnings
#[allow(dead_code)]
pub fn feedback(status: Feedback, message: String){
    
    match status {
        Feedback::Info    => { print!("{}{}{}",color::Fg(termion::color::LightYellow),"Info:",style::Reset);}
        Feedback::Warning => { print!("{}{}{}",color::Fg(termion::color::Yellow),"Warning:",style::Reset);}
        Feedback::Error   => { print!("{}{}{}",color::Fg(termion::color::Red),"Error:",style::Reset);}
    }
    print!("  {}\n",message);
}


// Function to show response times
pub fn show_response(now: SystemTime){
    // let my_normal_gray = color::Rgb (177, 177, 177);
    let duration = now.elapsed().unwrap().as_millis();
    let message = format!("Program runtime is: {:?}ms", duration);
    print!("{}{}{}", color::Fg(MY_NORMAL_GRAY), message, style::Reset); 
}  


/*
    A function that returns a string with repeated char (although
    in this function it is a string).
*/
pub fn repeat_char(ch: String, num: usize) -> String {
    let mut ret = String::new();
    for i in 0..num {
        match i {
            _ => { ret.push_str(&ch) }
        }
    }
    ret
}


#[allow(dead_code)]
// A function that justifies a phrase in a given number of characters
pub fn justify(phrase: String, num: usize, which: Justify) -> String {
    let ret: String = phrase.trim().to_string();
    let p_len = ret.len() as usize;

    if p_len >= num {
        return ret
    }

    let spare = num - p_len;
    let padding = repeat_char(" ".to_string(), spare);

    match which {
        Justify::Left   => { return format!("{}{}",ret, padding) }
        Justify::Right  => { return format!("{}{}",padding, ret) }
        Justify::Center => {
                        if let 0 = spare % 2 {        // if spare is even
                            let front_len = spare / 2;
                            let front = repeat_char(" ".to_string(), front_len);
                            let back = front.clone();
                            return format!("{}{}{}", front, ret, back) 

                        }                             // else spare is odd
                        let back_len = spare / 2 ;
                        let back  = repeat_char(" ".to_string(), back_len);
                        let front_len = back_len + 1;
                        let front = repeat_char(" ".to_string(), front_len);
                        return format!("{}{}{}", front, ret, back) 
        }
    }
}




/*
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
                                    ALL TESTS ARE RUN:  ONE AT A TIME   
                                    
    Running concurrent tests in the same directory with reading and writing has unpredictable results                                    
*/
#[warn(unused_assignments)]
#[cfg(test)]
mod tests {                   //     DONT RUN THE TESTS ABOVE THIS LINE
    use super::*;
    

    // #[ignore]
    #[test]
    fn t001_make_10x_int_1() {

    }






}  // End of tests