/*
    Trying to make a more generic settings file.

    2022.08.02      Sven Ponelat
*/


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
use std::collections::BTreeMap;
use std::env;
use std::time::{UNIX_EPOCH, Duration};
use std::io::{BufRead, BufReader};




#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize, derivative::Derivative)]
pub struct SettingsMap {
    pub map: BTreeMap< String, String >
}


impl SettingsMap {
    
    pub fn new() -> SettingsMap {
        let mut map = BTreeMap::new();
        SettingsMap::init_map(&mut map);
        SettingsMap { map: map }
    }
    
    fn init_map(map: &mut BTreeMap<String,String>) {
        map.insert("dataDir".to_string(), "/DATA/myToDo".to_string());
        map.insert("lastSpeciesViewed".to_string(), "0".to_string());
    
    } 
    
    
    
    
    



} //end of impl
















// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// pub fn load_settings(file: &str) -> SettingsMap {
pub fn load_settings(file: &str)  -> SettingsMap {
    let result_path = env::current_exe();
    if result_path.is_err() {
        let message = format!("Error in executable file path name.");
        feedback(Feedback::Error, message);
        exit(17);
    }
    
    let working_dir = result_path.unwrap().parent().unwrap().to_str().unwrap().to_owned();
    let path_to_trial = working_dir.clone() + "/trial";
    let path_to_settings = working_dir + "/" + file;
    
    // can we write to this directory
    let result_system = file_system_ok(&path_to_trial);
    if result_system.is_err() {
        let message = format!("Error in file_system_ok().");
        feedback(Feedback::Error, message);
        exit(17);
    }
    
    // if the settings file does NOT exist
    if ! Path::new(&path_to_settings).exists() {
        let default = SettingsMap::new();
        let res_save = export(&default.map, &path_to_settings);
        if res_save.is_err() {
            let message = format!("Error in saving settings file");
            feedback(Feedback::Error, message);
            exit(17);
        }
    }
    
    // load the settings file
    let res_import = import(&path_to_settings);
    if res_import.is_err() {
        let message = format!("Error in importing settings file");
        feedback(Feedback::Error, message);
        exit(17);
    }

    let ret = res_import.unwrap();

    // test to see if one can write to the data directory
    let path_to_data = ret.map.get("dataDir").unwrap().to_string() + "/trial"  ;
    let result_data = file_system_ok(&path_to_data);
    if result_data.is_err() {
        let message = format!("Error in writing to the data directory.");
        feedback(Feedback::Error, message);
        exit(17);
    }

    return ret;
}

// This functions checks if one can read and write to the directory.
// Again for testing puposes I have to input a file with a directory.
pub fn file_system_ok(test: &str) -> Result<(), &str> {
    let path = Path::new(test);

    // Lets open a file
    let mut file = match OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .open(path){
        Ok(content) => content,
        Err(_) => { return Err("Problem opening any file in file_system_ok"); }
    };

    // Lets write to a file
    match file.write_all("Hello Sven".as_bytes()){
        Ok(content) => content,
        Err(_) => { return Err("Problem writing any file in file_system_ok"); }   
    }

    // Lets delete a file
    match remove_file(&path){
        Ok(_) => (),
        Err(_) => { return Err("Problem removing any file in file_system_ok"); }   
    }

    Ok(())
}

// Reads the settings (settings.text) file into a treemap, returning a result
// again decided not to make this a json file
pub fn import(path: &str) -> Result<SettingsMap, &'static str> {
    let mut ret = SettingsMap::new();

    let res_file = File::open(path);
    if res_file.is_err() {
        return Err("Problem opening settings.txt");
    }
    
    let reader = BufReader::new(res_file.unwrap());
    
    for line in reader.lines() {
        if line.is_err(){
            return Err("Problem reading line in settings");
        }
        let read = Some(line.unwrap());
        if read.clone().is_some() {
            // let sub = read.unwrap().substring(0, 1);
            let s = read.clone().unwrap();
            let sub = s.substring(0, 1);
            if sub == " " || sub == "#" || sub == "" {
                continue;
            }
            let l = read.clone().unwrap();
            let split = l.split("\t");
            let vecs:Vec<&str> = split.collect();
            if vecs.len() >= 2 {
                ret.map.insert(vecs[0].to_string(), vecs[1].to_string());
            }
        }
    }

    if ret.map.len() == 0 {
        return Err("No settings were loaded");
    }

    return Ok(ret);
}


// Writes the settings to disk in local folder
// I have decided to make this text and not json
pub fn export( map: &BTreeMap<String,String>,  path: &str) -> Result<(), &'static str> {
    let path = Path::new(path);
    
    if remove_file(path).is_err() {
        let message = format!("No worries: old settings file was not found, a new one will be created.");
        feedback(Feedback::Info, message)
    }

    let vec = make_file_string(map.clone());
    // let serialized = serde_json::to_string_pretty(map);
    let mut file = match OpenOptions::new()
                            .read(false)
                            .write(true)
                            .create(true)
                            .open(path)  {
        
        Err(_) => { return Err("Problems opening text file in 'write_settings'"); } 
        Ok(file)   => { file }
    };

    for line in vec {
        let res = file.write(line.as_bytes());
        if res.is_err() {
            return Err("Error in writing settings file")
        }
    }
    
    Ok(())
} 


// make a string that will be written to a text file
pub fn make_file_string(map: BTreeMap<String,String>) -> Vec<String> {

    let mut vec:Vec<String> = Vec::new();
    let date = chrono::offset::Local::now();
    let gg = date.format("%Y-%m-%d");

    vec.push("##################\n".to_string());
    vec.push("#                #\n".to_string());
    let str = format!("#   {}   #\n",gg);
    vec.push(str);
    vec.push("#                #\n".to_string());
    vec.push("##################\n".to_string());
    vec.push("\n".to_string());
    vec.push("\n".to_string());

    for (k,v) in map {
        let element = k + "\t" +  &v + "\n";
        vec.push(element);
    }

    return vec;
}











// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use super::*;
    use substring::Substring;
    use std::fs::*;


    // #[ignore]
    #[test]
    fn t001_date() {
        let date = Utc.ymd(2022, 8, 3);
        let str = date.format("%Y-%m-%d");
        
        assert_eq!(str.to_string(), "2022-08-03".to_string());
    }
    
    // #[ignore]
    #[test]
    fn t002_make_file_string() {
        let path = "./tempo.txt";
        let mut map: BTreeMap<String,String> = BTreeMap::new();
        map.insert("dataDir".to_string(), "/DATA/myToDo".to_string());
        map.insert("lastSpeciesViewed".to_string(), "0".to_string());
        
        let res = make_file_string(map.clone());
        let len = res.len();
        assert_eq!(len, 9);
        
        let _res_wri = export(&map, path);
        let file = File::open(path);
        let x = file.unwrap().metadata().unwrap().len();
        remove_file(path).expect("Cleanup test failed");
        assert_eq!(x, 138);


    }























} // end of tests

