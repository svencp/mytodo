/*
    This is my to-do list
    2020.07.24      Sven Ponelat
*/

mod library;

use library::my_utils::*;
use library::task::*;
use std::collections::{BTreeMap};
use std::process::exit;
use std::fs::copy;
use std::path::Path;
use std::env;
use termion::{color, style};
use thousands::{Separable};
use std::time::{SystemTime};


const VERSION: &str = env!("CARGO_PKG_VERSION");



#[rustfmt::skip]
fn main() {
    let now = SystemTime::now();
    let arguments: Vec<String> = env::args().collect();
    let mut command = None;
    let mut sub1 = None;
    let mut sub2 = None;
    let mut sub3 = None;
    let mut sub4 = None;
    let mut sub5 = None;

    // It seems I need to do this,otherwise temporary variables get dropped
    match arguments.len() {
        2 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
        },
        3 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
        },
        4 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
        }
        5 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
            sub3 = Some(arguments[4].trim().to_owned());
        },
        6 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
            sub3 = Some(arguments[4].trim().to_owned());
            sub4 = Some(arguments[5].trim().to_owned());
        },
        7 => {
            command = Some(arguments[1].to_lowercase().trim().to_owned());
            sub1 = Some(arguments[2].trim().to_owned());
            sub2 = Some(arguments[3].trim().to_owned());
            sub3 = Some(arguments[4].trim().to_owned());
            sub4 = Some(arguments[5].trim().to_owned());
            sub5 = Some(arguments[6].trim().to_owned());
        },

        _ => { () }

    }// end of match






    // let pending = 


















    println!("Hello Svenny!");
    show_response(now)



} // End of Main
