use chrono::{NaiveDate, Duration, Datelike};
use std::ops::{Sub, Add};

pub fn beginning_of_week_date(current_date: NaiveDate) -> NaiveDate {
    let number_of_days_from_sun = current_date.weekday().num_days_from_sunday();
    let start_of_week = current_date.clone().sub(Duration::days(number_of_days_from_sun as i64));

    start_of_week
}

pub fn end_of_the_week_date(current_date: NaiveDate) -> NaiveDate {
    beginning_of_week_date(current_date).clone().add(Duration::days(6))
}