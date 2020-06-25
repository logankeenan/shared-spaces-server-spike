use chrono::{NaiveDate, Datelike, Duration};
use handlebars::{Handlebars};
use crate::factories::date_factory::{beginning_of_week_date, end_of_the_week_date};
use std::ops::{Sub, Add};
use titlecase::titlecase as titlecase_lib;


pub fn register_helpers(handlebars: &mut Handlebars) {

    handlebars_helper!(fmt_date_short_day: | date_string: str| {
        let result = NaiveDate::parse_from_str(date_string, "%Y-%m-%d");

        match result {
            Ok(date) => {
                date.format("%a").to_string()
            },
            Err(_) => "".to_string(),
        }
    });
    handlebars.register_helper("fmt_date_short_day", Box::new(fmt_date_short_day));

    handlebars_helper!(fmt_date_day_digit: | date_string: str| {
        let result = NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
        match result {
            Ok(date) => {
                date.format("%d").to_string()
            },
            Err(_) => "".to_string(),
        }
    });
    handlebars.register_helper("fmt_date_day_digit", Box::new(fmt_date_day_digit));

    handlebars_helper!(fmt_date_mm_dd_yy: | date_string: str| {
        let result = NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
        match result {
            Ok(date) => {
                date.format("%D").to_string()
            },
            Err(_) => "".to_string(),
        }
    });
    handlebars.register_helper("fmt_date_mm_dd_yy", Box::new(fmt_date_mm_dd_yy));

    handlebars_helper!(fmt_date_mm_dd_yyyy: | date_string: str| {
        let result = NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
        match result {
            Ok(date) => {
                date.format("%m/%d/%Y").to_string()
            },
            Err(_) => "".to_string(),
        }
    });
    handlebars.register_helper("fmt_date_mm_dd_yyyy", Box::new(fmt_date_mm_dd_yyyy));

    handlebars_helper!(fmt_date_mm_dd: | date_string: str| {
        let result = NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
        match result {
            Ok(date) => {
                date.format("%m/%d").to_string()
            },
            Err(_) => "".to_string(),
        }
    });
    handlebars.register_helper("fmt_date_mm_dd", Box::new(fmt_date_mm_dd));

    handlebars_helper!(add_days: | date_string: str, days: i64| {
        let result = NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
        match result {
            Ok(date) => {
                date.add(Duration::days(days)).to_string()
            },
            Err(_) => "".to_string(),
        }
    });
    handlebars.register_helper("add_days", Box::new(add_days));

    handlebars_helper!(titlecase: | string_to_titlecase: str| {
        titlecase_lib(&string_to_titlecase.to_string())
    });
    handlebars.register_helper("titlecase", Box::new(titlecase));

    handlebars_helper!(fmt_date_full_month: | date_string: str| {
        let result = NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
        match result {
            Ok(date) => {
                date.format("%B").to_string()
            },
            Err(_) => "".to_string(),
        }
    });
    handlebars.register_helper("fmt_date_full_month", Box::new(fmt_date_full_month));

    handlebars_helper!(date_subtract_one_week: | date_string: str| {
        let result = NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
        match result {
            Ok(date) => {
                date.sub(Duration::weeks(1)).to_string()
            },
            Err(_) => "".to_string(),
        }
    });
    handlebars.register_helper("date_subtract_one_week", Box::new(date_subtract_one_week));

    handlebars_helper!(date_add_one_week: | date_string: str| {
        let result = NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
        match result {
            Ok(date) => {
                date.add(Duration::weeks(1)).to_string()
            },
            Err(_) => "".to_string(),
        }
    });
    handlebars.register_helper("date_add_one_week", Box::new(date_add_one_week));

    handlebars_helper!(date_greater_than: | left_date_string: str, right_date_string: str| {
        let left_date = NaiveDate::parse_from_str(&left_date_string, "%Y-%m-%d").unwrap();
        let right_date = NaiveDate::parse_from_str(&right_date_string, "%Y-%m-%d").unwrap();

        left_date.gt(&right_date)
    });
    handlebars.register_helper("date_greater_than", Box::new(date_greater_than));

    handlebars_helper!(bool_eq: | bool_left_string: bool, bool_right_string: bool| {
        println!("left: {}", bool_left_string);
        println!("right: {}", bool_right_string);
        //
        // let left_bool: bool = bool_left_string.parse().unwrap();
        // let right_bool: bool = bool_right_string.parse().unwrap();

        bool_left_string.eq(&bool_right_string)
    });
    handlebars.register_helper("bool_eq", Box::new(bool_eq));

    handlebars_helper!(date_less_than: | left_date_string: str, right_date_string: str| {
        let left_date = NaiveDate::parse_from_str(&left_date_string, "%Y-%m-%d").unwrap();
        let right_date = NaiveDate::parse_from_str(&right_date_string, "%Y-%m-%d").unwrap();

        left_date.lt(&right_date)
    });
    handlebars.register_helper("date_less_than", Box::new(date_less_than));

    handlebars_helper!(concat_3: | first: str, second: str, third: str| {
            format!("{}{}{}", first, second, third)
    });
    handlebars.register_helper("concat_3", Box::new(concat_3));

    handlebars_helper!(num_string: | first: i64| {
            format!("{}", first)
    });
    handlebars.register_helper("num_string", Box::new(num_string));
}