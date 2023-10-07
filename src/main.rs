use clap::Parser;
use std::env::current_dir;

// Analyze git repo by conventional commits
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Exclude scope
    #[arg(short, long)]
    scope: bool,

    /// Exclude commit type
    #[arg(short, long)]
    commit_type: bool,

    /// Repository path
    #[arg(short, long)]
    path: Option<String>
}

fn main() {
    let args = Args::parse();

    let path = args.path
        .unwrap_or((current_dir()
            .expect("there should be a path"))
                .to_string_lossy().to_string()
        );
    
    
    println!("path {}", path);
}
