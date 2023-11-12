use std::{cmp, collections::HashMap};

use crate::{
    commit::{CommitBucket, FileStatInfo},
    config::{Config, SortType},
};
use colored::Colorize;

pub trait Reporter<'a> {
    fn output(
        &self,
        config: &Config,
        commit_bucket: &CommitBucket,
        file_summs: &Vec<&'a FileStatInfo>,
    );
    // fn new() -> Self;
}

pub struct BaseReporter<'a> {
    config: &'a Config,
    commit_bucket: &'a CommitBucket,
    reporter: Box<dyn Reporter<'a>>,
    file_summs: Vec<&'a FileStatInfo>,
}

impl<'a> BaseReporter<'a> {
    pub fn new(
        config: &'a Config,
        commit_bucket: &'a CommitBucket,
        reporter: Box<(dyn Reporter<'a> + 'static)>,
    ) -> BaseReporter<'a> {
        BaseReporter {
            config,
            reporter: reporter,
            commit_bucket,
            file_summs: Self::map_file_summs(config, &commit_bucket.file_summs),
        }
    }

    pub fn output(&self) {
        self.reporter
            .output(&self.config, &self.commit_bucket, &self.file_summs)
    }

    fn map_file_summs(
        conf: &Config,
        summ_map: &'a HashMap<String, FileStatInfo>,
    ) -> Vec<&'a FileStatInfo> {
        let mut summ = summ_map.values().collect::<Vec<&FileStatInfo>>();

        summ.sort_by(|a, b| match conf.sort_files {
            SortType::Asc => a.total_changes.abs().cmp(&b.total_changes.abs()),
            SortType::Desc => b.total_changes.abs().cmp(&a.total_changes.abs()),
        });

        let boundry = match conf.file_count.cmp(&summ.len()) {
            cmp::Ordering::Greater => summ.len(),
            cmp::Ordering::Less => conf.file_count,
            _ => summ.len(),
        };

        summ[..boundry].to_vec()
    }
}

pub struct Stdout {}

impl<'a> Reporter<'a> for Stdout {
    fn output(
        &self,
        config: &Config,
        commit_bucket: &CommitBucket,
        file_summs: &Vec<&'a FileStatInfo>,
    ) {
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

        for file_sum in file_summs.iter() {
            println!(
                "{}: {} {} | total: {}",
                file_sum.path,
                format!("+{}", file_sum.inserted).green(),
                format!("-{}", file_sum.deleted).red(),
                file_sum.total_changes
            );
        }
    }
}

// struct JsonReporter {}

// impl Reporter for JsonReporter {
//     fn output(&self) -> ! {
//         let x = 1;
//     }
// }
