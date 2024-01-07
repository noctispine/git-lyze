use std::{cmp, collections::HashMap, env::set_var};

use crate::{
    commit::FileStatInfo,
    config::{Config, DateFormatType, LogLevel, SortType},
};
use chrono::{DateTime, NaiveDate, NaiveDateTime};

pub fn parse_date(value: &String, format: &String, format_type: &DateFormatType) -> Option<i64> {
    match format_type {
        DateFormatType::DateOnly => NaiveDate::parse_from_str(&value, &format)
            .ok()
            .map(|date| date.and_hms_opt(0, 0, 0).unwrap_or_default())
            .map(|date| date.timestamp()),

        DateFormatType::DateAndTime => NaiveDateTime::parse_from_str(&value, &format)
            .ok()
            .map(|date| date.timestamp()),

        DateFormatType::DateTimeAndTimezone => DateTime::parse_from_str(&value, format)
            .ok()
            .map(|date| date.timestamp()),
    }
}

pub fn map_file_summs<'a>(
    conf: &Config,
    summ_map: &'a HashMap<String, FileStatInfo>,
) -> Vec<&'a FileStatInfo> {
    let mut summ = summ_map.values().collect::<Vec<&FileStatInfo>>();

    summ.sort_by(|a, b| match conf.sort_files {
        SortType::Asc => a.total_changes.abs().cmp(&b.total_changes.abs()),
        SortType::Desc => b.total_changes.abs().cmp(&a.total_changes.abs()),
    });

    let boundry = match conf.file_count.cmp(&summ.len()) {
        cmp::Ordering::Greater => summ.len(),
        cmp::Ordering::Less => conf.file_count,
        _ => summ.len(),
    };

    summ[..boundry].to_vec()
}

pub fn set_log_env(log_level: &LogLevel) {
    let key = "RUST_LOG";
    match log_level {
        LogLevel::Error => set_var(key, "error"),
        LogLevel::Warn => set_var(key, "warn"),
        LogLevel::Info => set_var(key, "info"),
        LogLevel::Debug => set_var(key, "debug"),
        LogLevel::Trace => set_var(key, "trace"),
        _ => set_var(key, "off"),
    };
}
