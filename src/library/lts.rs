/*
    My attempt at a local timestamp
    2022-08-08     Sven Ponelat
*/



use chrono::*;
use chronoutil::*;




pub const DATE_FORMAT: &str      = "%Y-%m-%d";
pub const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub const DAY_SECS: i64          = 86400;
pub const WEEK_SECS: i64         = 604800;












// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

// a function that returns the utc timestamp as a local timestamp;
pub fn lts_now() -> i64 {
    let ts = Local::now().timestamp();
    let offset = Local.timestamp(ts, 0).offset().fix().local_minus_utc() as i64;
    let ret = ts + offset;

    return ret;
}

// a function that takes a lts timestamp and converts it to a date string
pub fn lts_to_date_string(ts: i64) -> String {
    let ndt = NaiveDateTime::from_timestamp(ts, 0);
    let ret = ndt.format(DATE_FORMAT).to_string();
    return ret;
}

// a function that takes a lts timestamp and converts it to a date and time string
pub fn lts_to_date_time_string(ts: i64) -> String {
    let ndt = NaiveDateTime::from_timestamp(ts, 0);
    let ret = ndt.format(DATE_TIME_FORMAT).to_string();
    return ret;
}


// a function that takes a date string and converts it to a timestamp (dont do local here)
pub fn lts_date_string_to_timestamp(date_str: &str) ->  Result<i64, &'static str> {
    let nd = NaiveDate::parse_from_str(date_str, DATE_FORMAT);
    if nd.is_err() {
        return Err("NaiveDate parse error");
    }
    let ndt = nd.unwrap().and_hms(0, 0, 0);
    // let local_ts = lts_timestamp_to_local_timestamp(ndt.timestamp());
    return Ok(ndt.timestamp());
}

// a function that takes a date string and converts it to a timestamp (dont do local here)
pub fn lts_date_time_string_to_timestamp(date_time_str: &str) ->  Result<i64, &'static str> {
    let ndt = NaiveDateTime::parse_from_str(date_time_str, DATE_TIME_FORMAT);
    if ndt.is_err() {
        return Err("NaiveDateTime parse error");
    }
    // let local_ts = lts_timestamp_to_local_timestamp(ndt.unwrap().timestamp());
    return Ok(ndt.unwrap().timestamp());
}

// function to parse string to timestamp
pub fn lts_from_str64_to_timestamp(str_64: &str) -> Result<i64, &'static str> {
    let res = str_64.parse::<i64>();
    if res.is_err(){
        return Err("i64 parsing error")
    }
    return Ok(res.unwrap());
}

// function to add a timestamp to recur_term
pub fn lts_add_timestamp_to_recur_term(ts: i64, term: &str) -> Result<i64, &'static str> {
    if ! term.starts_with("+") {
        return Err("term doesn't start with +")
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
    let num = res_num.unwrap();
    
    // has the term got the right chars (only d,w,m,y)
    // let s_char: String = c_arr.iter().collect();
    if c_arr.len() > 1 {
        return Err("Too many characters in duration");
    }
    if c_arr.len() < 1 {
        return Err("No duration symbol given");
    }

    let time_ndt = NaiveDateTime::from_timestamp(ts, 0);

    match c_arr[0] {
        'd' => {
            let addition = num * DAY_SECS;
            let ret = time_ndt.timestamp() + addition;
            return Ok(ret);
        }
        'w' => {
            let addition = num * WEEK_SECS;
            let ret = time_ndt.timestamp() + addition;
            return Ok(ret);
        }
        'm' => {
            let delta = RelativeDuration::months(num as i32);
            let ndt = time_ndt + delta;
            return Ok(ndt.timestamp());
        }
        'y' => {
            let delta = RelativeDuration::years(num as i32);
            let ndt = time_ndt + delta;
            return Ok(ndt.timestamp());
        }
        _ => {
            return Err("Illegal duration symbol");
        }
    }

} // end of function



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
    fn t001_lts1() {
        let date_str = "2000-01-01";
        let ts = lts_date_string_to_timestamp(date_str);
        let date_time_str =  lts_to_date_time_string(ts.unwrap());
        assert_eq!(date_time_str,"2000-01-01 00:00:00");

        let now = Utc::now().timestamp();
        let offset = Local.timestamp(now, 0).offset().fix().local_minus_utc() as i64; 
        let utc_adjusted = now + offset;
        let utc_adjusted_str = lts_to_date_time_string(utc_adjusted);
        let new_now = lts_now();
        let new_now_str = lts_to_date_time_string(new_now);
        assert_eq!(new_now_str,utc_adjusted_str);

        let date_str2 = "2000-01-01";
        let ts2 = lts_date_string_to_timestamp(date_str2);
        let date_time_str2 =  lts_to_date_string(ts2.unwrap());
        assert_eq!(date_time_str2,"2000-01-01");
        assert_eq!(1,1);
    }

    // #[ignore]
    #[test]
    fn t002_recur_term() {
        let date_str = "2000-01-01";
        let term = "+3m";
        let ts = lts_date_string_to_timestamp(date_str);
        let res = lts_add_timestamp_to_recur_term(ts.unwrap(),term);
        let res_time = lts_to_date_time_string(res.unwrap());
        assert_eq!(res_time,"2000-04-01 00:00:00".to_string());

        let date_str2 = "2000-01-01";
        let term2 = "+17d";
        let ts2 = lts_date_string_to_timestamp(date_str2);
        let res2 = lts_add_timestamp_to_recur_term(ts2.unwrap(),term2);
        let res_time2 = lts_to_date_time_string(res2.unwrap());
        assert_eq!(res_time2,"2000-01-18 00:00:00".to_string());

        let date_str3 = "2000-01-01";
        let term3 = "+6w";
        let ts3 = lts_date_string_to_timestamp(date_str3);
        let res3 = lts_add_timestamp_to_recur_term(ts3.unwrap(),term3);
        let res_time3 = lts_to_date_time_string(res3.unwrap());
        assert_eq!(res_time3,"2000-02-12 00:00:00".to_string());

        let date_str = "2000-01-27";
        let term = "+17y";
        let ts = lts_date_string_to_timestamp(date_str);
        let res = lts_add_timestamp_to_recur_term(ts.unwrap(),term);
        let res_time = lts_to_date_time_string(res.unwrap());
        assert_eq!(res_time,"2017-01-27 00:00:00".to_string());
        
        // something with now
        let now_utc = Utc::now().timestamp();
        let offset = Local.timestamp(now_utc, 0).offset().fix().local_minus_utc() as i64;
        let add = now_utc + offset;
        assert_eq!(add,lts_now());
    }


















} //end of tests