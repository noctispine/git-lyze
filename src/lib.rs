pub mod commit;
pub mod config;
pub mod convention;
pub mod customerror;
pub mod repo;
pub mod reporters;
pub mod test_utils;

use std::{env::current_dir, fs::File, path::Path};

use clap::Parser;

use crate::{commit::CommitBucket, config::Config, repo::Repo};

pub fn run() {
    let config = Config::parse();

    let config = if let Ok(config_file) = File::open(&config.config_path) {
        match serde_json::from_reader(config_file) {
            Ok(args) => args,
            Err(e) => panic!("Error in configuration file:\n{}", e),
        }
    } else {
        config
    };

    let path = config.path.clone().unwrap_or(
        (current_dir().expect("there should be a path"))
            .to_string_lossy()
            .to_string(),
    );

    let repo = Repo::init(Path::new(&path)).unwrap();

    let commit_bucket = CommitBucket::build(&repo, &config.convention_style, &config).unwrap();

    for cm in commit_bucket.commits {
        println!("{}: {}", cm.author.name, cm.summary);
        if cm.stats.is_err() {
            continue;
        }
        for file in cm.stats.unwrap().file_stat_infos {
            println!("{}: {} {}\n", file.path, file.inserted, file.deleted)
        }
    }
}
