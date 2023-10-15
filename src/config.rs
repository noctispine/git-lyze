use clap::Parser;

// Analyze git repo by conventional commits
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Exclude scope
    #[arg(short, long)]
    pub scope: bool,

    /// Exclude commit type
    #[arg(short, long)]
    pub commit_type: bool,

    /// Repository path
    #[arg(short, long)]
    pub path: Option<String>,

    #[arg(short = 't', long, default_value = "type(optional_scope): description")]
    pub convention_style: String
}
