use chrono::{NaiveDateTime, NaiveDate};
use serde::Deserializer;

pub mod user;

use serde::de::{Deserialize, Error};
use std::str::FromStr;

pub fn current_date_time() -> NaiveDateTime {
    chrono::Utc::now().naive_local()
}

pub fn deserialize_i16_empty_string_as_zero<'de, D>(deserializer: D) -> Result<i16, D::Error>
    where
        D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;

    if string.trim().eq("") {
        return Ok(0);
    } else {
        return i16::from_str(&string).map_err(Error::custom);
    }
}

pub fn date_format_yyyy_mm_dd<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
{
    const FORMAT: &'static str = "%Y-%m-%d";
    let string = String::deserialize(deserializer)?;

    println!("string: {}", string);

    let date = chrono::NaiveDate::parse_from_str(string.as_str(), FORMAT).unwrap();

    Ok(date)
}

pub fn boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;

    println!("boolean string: {}", string);

    if string == "true" || string == "on" {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn default_as_false() -> bool {
    false
}