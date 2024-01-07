pub mod cache;
pub mod commit;
pub mod config;
pub mod convention;
pub mod customerror;
pub mod defaults;
pub mod ownerships;
pub mod repo;
pub mod reporters;
pub mod test_utils;
pub mod tracker;
pub mod utils;

use crate::utils::set_log_env;
use std::{env::current_dir, fs::File, path::Path};

use clap::Parser;
use colored::Color;
use reporters::{BaseReporter, Stdout};
use tracker::Tracker;

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

    set_log_env(&config.log_level);
    env_logger::init();
    let mut perf_tracker = Tracker::new(&config, Color::Magenta, None);

    let path = config.path.clone().unwrap_or(
        (current_dir().expect("there should be a path"))
            .to_string_lossy()
            .to_string(),
    );

    perf_tracker.start("init repo");
    let repo = Repo::init(Path::new(&path)).unwrap();
    perf_tracker.stop();

    perf_tracker.start("collect commit bucket");
    let commit_bucket = CommitBucket::build(&repo, &config.convention_style, &config).unwrap();
    perf_tracker.stop();

    perf_tracker.start("create reporter");
    let base_reporter = BaseReporter::new(&config, &commit_bucket, Box::new(Stdout {}));
    perf_tracker.stop();

    base_reporter.output();
}
