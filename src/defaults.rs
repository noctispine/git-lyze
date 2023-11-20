use crate::config::{DateFormatType, SortType};

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


