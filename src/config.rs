use std::path;

use clap::Parser;
use serde::{Deserialize, Serialize};

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
}
