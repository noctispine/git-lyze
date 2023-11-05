use crate::{commit::CommitBucket, config::Config};
use colored::Colorize;

pub trait Reporter {
    fn output(&self, commit_bucket: &CommitBucket, config: &Config);
    // fn new() -> Self;
}

pub struct BaseReporter<'a> {
    config: &'a Config,
    commit_bucket: &'a CommitBucket,
    reporter: Box<dyn Reporter>,
}

impl<'a> BaseReporter<'a> {
    pub fn new(
        config: &'a Config,
        commit_bucket: &'a CommitBucket,
        reporter: Box<dyn Reporter>,
    ) -> BaseReporter<'a> {
        BaseReporter {
            config,
            reporter: reporter,
            commit_bucket,
        }
    }

    pub fn output(&self) {
        self.reporter.output(&self.commit_bucket, &self.config)
    }
}

pub struct Stdout {}

impl Reporter for Stdout {
    fn output(&self, commit_bucket: &CommitBucket, config: &Config) {
        let scopes = commit_bucket
            .scopes
            .keys()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let types = commit_bucket
            .types
            .keys()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        println!("{}: {}", "scopes".cyan().bold(), scopes);
        println!("{}: {}", "types".cyan().bold(), types);
        for i in commit_bucket.commits.iter() {
            println!("{}: {}", i.summary, i.time);
        }
    }
}

// struct JsonReporter {}

// impl Reporter for JsonReporter {
//     fn output(&self) -> ! {
//         let x = 1;
//     }
// }
