use std::env::current_dir;
use clap::Parser;
use git_lyze::config::Config;

fn main() {
    let args = Config::parse();

    let path = args.path
        .unwrap_or((current_dir()
            .expect("there should be a path"))
                .to_string_lossy().to_string()
        );
    
    
    println!("path {}", path);
}
