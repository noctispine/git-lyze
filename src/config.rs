use crate::defaults::{
    cache_path, convention_style, date_format, date_format_type, file_count, log_level,
    revert_message_pattern, sort_files,
};
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::path;

#[derive(Serialize, Deserialize, Debug, Parser)]
#[serde(rename_all = "camelCase")]
#[command(author, version, about)]
pub struct Config {
    /// Config Path
    #[arg(short = 'c', long, default_value = "lyze.json")]
    pub config_path: path::PathBuf,

    /// Exclude scope
    #[arg(long)]
    pub exclude_scope: bool,

    /// Output Type
    #[arg(short = 'o', long, value_enum, default_value_t = OutputType::Json)]
    pub output_type: OutputType,

    /// Exclude commit type
    #[arg(long)]
    pub exclude_commit_type: bool,

    /// Repository path
    #[arg(long)]
    pub path: Option<String>,

    /// Convetion style
    #[arg(short = 't', long, default_value = "type(optional_scope): description")]
    #[serde(default = "convention_style")]
    pub convention_style: String,

    /// Revert Message Pattern
    #[arg(long, default_value = "revert_indicator \"message\"")]
    #[serde(default = "revert_message_pattern")]
    pub revert_message_pattern: String,

    /// Exclude by changed files' names
    #[arg(long = "exclude-file-patterns", value_parser, num_args=1..)]
    pub exclude_filename_patterns: Option<Vec<String>>,

    /// Filter by filenames
    #[arg(short = 'f', long = "filename-patterns", value_parser, num_args=1..)]
    pub filter_filename_pattern: Option<String>,

    /// Filter by author's username
    #[arg(short = 'u', long = "authors", value_parser, num_args=1..)]
    pub filter_authors: Option<Vec<String>>,

    /// Filter by scope e.g., "utils"
    #[arg(short = 's', long = "scopes", value_parser, num_args=1..)]
    pub filter_scopes: Option<Vec<String>>,

    /// Filter by type e.g., "feat"
    #[arg(short = 'y', long = "types", value_parser, num_args=1..)]
    pub filter_types: Option<Vec<String>>,

    /// Filter by start_date
    #[arg(long)]
    pub start_date: Option<String>,

    /// Filter by end_date
    #[arg(long)]
    pub end_date: Option<String>,

    /// Date format
    #[arg(long, default_value = "%b %e %T %Y %z")]
    #[serde(default = "date_format")]
    pub date_format: String,

    #[arg(long, value_enum, default_value_t = DateFormatType::DateTimeAndTimezone)]
    #[serde(default = "date_format_type")]
    pub date_format_type: DateFormatType,

    /// File Summary, show N files
    #[arg(long, default_value_t = 10)]
    #[serde(default = "file_count")]
    pub file_count: usize,

    /// File Summary, sort files by
    #[arg(long, value_enum, default_value_t = SortType::Desc)]
    #[serde(default = "sort_files")]
    pub sort_files: SortType,

    #[clap(skip)]
    pub ownerships: Option<Vec<OwnershipConfig>>,

    /// Cache path
    #[arg(long, default_value = ".lyze.cache.json")]
    #[serde(default = "cache_path")]
    pub cache_path: String,

    /// Log Level
    #[arg(long, value_enum, default_value_t = LogLevel::Off)]
    #[serde(default = "log_level")]
    pub log_level: LogLevel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipConfig {
    pub name: String,
    pub patterns: Vec<String>,
    pub authors: Vec<String>,
}

#[derive(ValueEnum, Serialize, Deserialize, Clone, Debug)]
pub enum DateFormatType {
    DateOnly,
    DateAndTime,
    DateTimeAndTimezone,
}

#[derive(ValueEnum, Serialize, Deserialize, Clone, Debug)]
pub enum SortType {
    Asc,
    Desc,
}
#[derive(ValueEnum, Serialize, Deserialize, Clone, Debug)]
pub enum OutputType {
    Json,
    Stdout,
}

#[derive(ValueEnum, Serialize, Deserialize, Clone, Debug)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
    Performance,
    Off,
}
