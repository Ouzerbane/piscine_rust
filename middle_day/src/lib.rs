use chrono::{NaiveDate, Datelike, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    let year = year as i32;

    let is_leap = NaiveDate::from_ymd_opt(year, 2, 29).is_some();

    if is_leap {
        return None;
    }
    
    let middle_date = NaiveDate::from_yo_opt(year, 183)?; 
    Some(middle_date.weekday())
}
