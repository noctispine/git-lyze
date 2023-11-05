use crate::config::DateFormatType;
use chrono::{DateTime, NaiveDate, NaiveDateTime};

pub fn parse_date(value: &String, format: &String, format_type: &DateFormatType) -> Option<i64> {
    match format_type {
        DateFormatType::DateOnly => NaiveDate::parse_from_str(&value, &format)
            .ok()
            .map(|date| date.and_hms_opt(12, 0, 0).unwrap_or_default())
            .map(|date| date.timestamp()),

        DateFormatType::DateAndTime => NaiveDateTime::parse_from_str(&value, &format)
            .ok()
            .map(|date| date.timestamp()),

        DateFormatType::DateTimeAndTimezone => DateTime::parse_from_str(&value, format)
            .ok()
            .map(|date| date.timestamp()),
    }
}
