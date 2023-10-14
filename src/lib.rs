pub mod customerror;
pub mod repo;
pub mod config;
pub mod commit;

use std::env::current_dir;
use clap::Parser;

use crate::config::Config;


pub fn run() {
    let args = Config::parse();

    let path = args.path
        .unwrap_or((current_dir()
            .expect("there should be a path"))
                .to_string_lossy().to_string()
        );
    
    
    println!("path {}", path);    
}