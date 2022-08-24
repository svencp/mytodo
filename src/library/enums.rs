/*
    Most of my enums are in here.
    2022.07.24      Sven Ponelat

*/

// use serde::{Serialize, Deserialize};
// use field_names::*;
use std::str::FromStr;

use substring::Substring;


//enum for which modifications are allowed
#[derive(Debug, Clone, Eq, PartialEq  )]
#[allow(non_snake_case)]
pub enum AllowMods {
    // description
    Des,
    // due
    Due,
    // recur
    Rec,
    // rtype
    Rty,
    // start
    Sta,
    // tags
    Tag,
    // wait
    Wai,
}

impl AllowMods {
    pub fn text(&self) -> &str{
        match *self {
            AllowMods::Des  => "des",
            AllowMods::Due  => "due",
            AllowMods::Rec  => "rec",
            AllowMods::Rty  => "rty",
            AllowMods::Sta  => "sta",
            AllowMods::Tag  => "tag",
            AllowMods::Wai  => "wai",
        }
    }
}


impl FromStr for AllowMods {
    type Err = ();
    
    fn from_str(input: &str) -> Result<AllowMods, Self::Err> {
        let lower = input.trim().to_lowercase();
        if lower.len() < 3 {
            return Err(())
        }
        let matcho = lower.substring(0, 3);

        match matcho {
            "des"  => Ok(AllowMods::Des),
            "due"  => Ok(AllowMods::Due),
            "rec"  => Ok(AllowMods::Rec),
            "rty"  => Ok(AllowMods::Rty),
            "sta"  => Ok(AllowMods::Sta),
            "tag"  => Ok(AllowMods::Tag),
            "wai"  => Ok(AllowMods::Wai),
            _      => Err(()),
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


// enum for recurring type
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq  )]
#[allow(non_snake_case)]
pub enum Rtype {
    Periodic,
    Chained,
}


impl Rtype {
    pub fn text(&self) -> &str{
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



#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq )]
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












            
            
            
            

//enum for argument type
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq  )]
#[allow(non_snake_case)]
pub enum VirtualTags {
    Active,
    Annotated,
    Child,
    Completed,
    Deleted, 
    Overdue,
    Parent,
    Pending,
    Tagged,
    Waiting,
}

impl VirtualTags {
    pub fn text(&self) -> &str{
        match *self {
            VirtualTags::Active    => "Active",
            VirtualTags::Annotated => "Annotated",
            VirtualTags::Child     => "Child",
            VirtualTags::Completed => "Completed",
            VirtualTags::Deleted   => "Deleted",
            VirtualTags::Overdue   => "Overdue",
            VirtualTags::Parent    => "Parent",
            VirtualTags::Pending   => "Pending",
            VirtualTags::Tagged    => "Tagged",
            VirtualTags::Waiting   => "Waiting",
        }
    }


} //end of impl





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
    
    // #[ignore]
    #[test]
    fn t004_allow_mods() {
        let s1 = "wait:2022-09-01"; 
        let res = AllowMods::from_str(s1);
        assert_eq!(res.unwrap(), AllowMods::Wai);
        
        let s1 = "waxt:2022-09-01"; 
        let res = AllowMods::from_str(s1);
        assert_eq!(res.is_err(), true);
    }





















} //end of tests





















