use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::path;
use crate::defaults::{
    convention_style,
    date_format,
    date_format_type,
    file_count,
    sort_files
};

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

    /// Exclude commit type
    #[arg(long)]
    pub exclude_commit_type: bool,

    /// Repository path
    #[arg(long)]
    pub path: Option<String>,

    // Convetion style
    #[arg(short = 't', long, default_value = "type(optional_scope): description")]
    #[serde(default = "convention_style")]
    pub convention_style: String,

    // filter by changed file patterns
    #[arg(short = 'f', long = "file-patterns", value_parser, num_args=1..)]
    pub filter_filenames: Option<Vec<String>>,

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
    #[arg(long, default_value = "%a %b %e %T %Y %z")]
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
