pub mod commit;
pub mod config;
pub mod convention;
pub mod customerror;
pub mod ownerships;
pub mod repo;
pub mod reporters;
pub mod test_utils;
pub mod utils;

use std::{env::current_dir, fs::File, path::Path};

use clap::Parser;
use reporters::{BaseReporter, Stdout};

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

    let base_reporter = BaseReporter::new(&config, &commit_bucket, Box::new(Stdout {}));

    base_reporter.output();
}
