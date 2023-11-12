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

    /// Exclude commit type
    #[arg(long)]
    pub exclude_commit_type: bool,

    /// Repository path
    #[arg(long)]
    pub path: Option<String>,

    // Convetion style
    #[arg(short = 't', long, default_value = "type(optional_scope): description")]
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
    pub date_format: String,

    #[arg(long, value_enum, default_value_t = DateFormatType::DateTimeAndTimezone)]
    pub date_format_type: DateFormatType,

    #[clap(skip)]
    pub ownerships: Option<Vec<OwnershipConfig>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipConfig {
    pub name: String,
    pub pattern: String,
    pub authors: Vec<String>,
}

#[derive(ValueEnum, Serialize, Deserialize, Clone, Debug)]
pub enum DateFormatType {
    DateOnly,
    DateAndTime,
    DateTimeAndTimezone,
}
