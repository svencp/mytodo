/*
    Trying to make a more generic settings file.

    2022.08.02      Sven Ponelat
*/


use crate::library::my_utils::*;
use chrono::prelude::*;
use std::path::Path;
use std::fs::{ OpenOptions };
use std::fs::remove_file;
use std::io::Write;
use std::process::exit;
use std::fmt::{Debug};
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::env;
use std::time::{UNIX_EPOCH, Duration};





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

// Reads the settings (settings.json) file into a treemap, returning a result
pub fn import(path: &str) -> Result<SettingsMap, &str> {
    let str_file  = std::fs::read_to_string(path );
    let content = match str_file {
        Ok(content) => { content },
        Err(_) => { return Err("Problem reading to String in Settings"); }
    };
    
    let m: SettingsMap  = match serde_json::from_str(&content){
        Ok(map) => map,
        Err(_) => { return Err("Problem converting to json in Settings"); }
    };

    Ok(m)
}


// Writes the settings to disk in local folder
pub fn export( map: &BTreeMap<String,String>,  path: &str) -> Result<(), String> {
    let path = Path::new(path);
    
    if remove_file(path).is_err() {
        let message = format!("No worries: old settings file was not found, a new one will be created.");
        feedback(Feedback::Info, message)
    }

    let serialized = serde_json::to_string_pretty(map);
    let mut file = match OpenOptions::new()
                            .read(false)
                            .write(true)
                            .create(true)
                            .open(path)  {
        
        Err(_) => { return Err("Problems opening json file in 'write_settings'".to_string()); } 
        Ok(file)   => { file }
    };
    
    match file.write_all(serialized.unwrap().as_bytes()) {
        Err(_) => { return Err("Problems writing json file in 'write_settings'".to_string()); } 
        Ok(file)   => { file } 
    }
    
    Ok(())
} 











// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::copy};
    use substring::Substring;
    use std::fs::remove_file;

    
    // // #[ignore]
    // #[test]
    // fn t001_task_new() {
    //     let mut t1 = Task::new();
    //     t1.id = Some(23);
    //     t1.description = "This is a description".to_string();
    //     t1.status = Status::Pending;
        

    //     let yebo: bool = t1.entry > 1650000000;
    //     assert_eq!(yebo, true);
    // }























} // end of tests


