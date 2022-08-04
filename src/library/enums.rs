/*
    Most of my enums are in here.
    2022.07.24      Sven Ponelat

*/

// use serde::{Serialize, Deserialize};
// use field_names::*;


#[derive(Clone  )]
#[allow(non_snake_case)]
pub enum Status {
    Deleted,
    Waiting,
    Pending,
    Completed,
    Recurring,
}


impl Status {
    pub fn text(&self) -> &str{
        match *self {
            Status::Deleted   => "Deleted",
            Status::Waiting   => "Waiting",
            Status::Pending   => "Pending",
            Status::Completed => "Completed",
            Status::Recurring => "Recurring",
        }
    }
}


// enum for recurring type
#[derive(Clone  )]
#[allow(non_snake_case)]
pub enum Rtype {
    Periodic,
    Chained,
}


impl Rtype {
    fn text(&self) -> &str{
        match *self {
            Rtype::Periodic  => "Periodic",
            Rtype::Chained   => "Chained",
        }
    }
}


//enum for argument type
#[derive(Debug, Clone, Eq, PartialEq  )]
#[allow(non_snake_case)]
pub enum ArgType {
    None,
    Integer,
    Hexidecimal,
    Command,
    Unknown,
}


// impl ArgType {
//     fn text(&self) -> &str{
//         match *self {
//             Rtype::Periodic  => "Periodic",
//             Rtype::Chained   => "Chained",
//         }
//     }
// }













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
    fn t001_enum_status1() {
        let e1 = Status::Deleted;
        let text = e1.text();

        assert_eq!(text, "Deleted");
    }


    // #[ignore]
    #[test]
    fn t002_enum_lower() {
        let e1 = Status::Deleted;
        assert_eq!(e1.text().to_lowercase(), "deleted");
    }
























} //end of tests





















