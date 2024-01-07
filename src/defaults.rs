use crate::config::{DateFormatType, LogLevel, OutputType, SortType};

pub fn convention_style() -> String {
    "type(optional_scope): description".to_string()
}

pub fn date_format() -> String {
    "%a %b %e %T %Y %z".to_string()
}

pub fn date_format_type() -> DateFormatType {
    DateFormatType::DateTimeAndTimezone
}

pub fn file_count() -> usize {
    10
}

pub fn sort_files() -> SortType {
    SortType::Desc
}

pub fn output_type() -> OutputType {
    OutputType::Json
}

pub fn cache_path() -> String {
    ".lyze.config.json".to_string()
}

pub fn revert_message_pattern() -> String {
    "revert_indicator \"message\"".to_string()
}

pub fn log_level() -> LogLevel {
    LogLevel::Off
}
