/*
    Most of my enums are in here.
    2022.07.24      Sven Ponelat

*/

// use serde::{Serialize, Deserialize};
// use field_names::*;
use std::str::FromStr;



#[derive(Clone, Debug, PartialEq  )]
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

impl FromStr for Status {
    type Err = ();
    
    fn from_str(input: &str) -> Result<Status, Self::Err> {
        let lower = input.trim().to_lowercase();
        let matcho = lower.as_str();

        match matcho {
            "deleted"   => Ok(Status::Deleted),
            "waiting"   => Ok(Status::Waiting),
            "pending"   => Ok(Status::Pending),
            "completed" => Ok(Status::Completed),
            "recurring" => Ok(Status::Recurring),
            _           => Err(()),
        }
    }
    
}



// enum for recurring type
#[derive(Clone, Debug, PartialEq  )]
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

impl FromStr for Rtype {
    type Err = ();
    
    fn from_str(input: &str) -> Result<Rtype, Self::Err> {
        let lower = input.trim().to_lowercase();
        let matcho = lower.as_str();

        match matcho {
            "periodic"  => Ok(Rtype::Periodic),
            "chained"   => Ok(Rtype::Chained),
            _           => Err(()),
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
    
    // #[ignore]
    #[test]
    fn t003_from_string() {
        let s1 = "DELETED"; 
        let sta = Status::from_str(s1);
        assert_eq!(sta.is_err(), false);
        
        let s2 = "something".to_lowercase();
        let sta2 = Status::from_str(&s2);
        assert_eq!(sta2.is_err(), true);
    }
    
    // #[ignore]
    #[test]
    fn t003_from_string_chained() {
        let s1 = "chained"; 
        let sta = Rtype::from_str(s1);
        assert_eq!(sta.is_err(), false);
        
        let s2 = "something".to_lowercase();
        let sta2 = Rtype::from_str(&s2);
        assert_eq!(sta2.is_err(), true);
    }






















} //end of tests





















