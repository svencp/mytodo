/*
    Module for everything all main functions
    2022.07.31      Sven Ponelat

*/








// Function to determine whether the first argument is an ineteger
pub fn is_arg_integer(first: &str) -> Result<(Vec<i64>), String> {


    Err("Not an inetger".to_string())
}


// Function to determine whether the first argument is hexidecimal
pub fn is_arg_hexidecimal(first: &str) -> Result<(Vec<i64>), String> {


    Err("Not a hexidecimal".to_string())
}


// Show the task given by integer id
pub fn report_single_id(){

}


// Show the task given by hexi uuiid
pub fn report_single_uuiid(){

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















} //end of tests