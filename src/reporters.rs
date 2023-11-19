use std::{cmp, collections::HashMap};

use crate::{
    commit::{CommitBucket, FileStatInfo},
    config::{Config, SortType},
};
use colored::Colorize;

pub trait Reporter<'a> {
    fn output(&self, config: &Config, report_info: &ReportStructure<'a>);
    // fn new() -> Self;
}

pub struct ReportStructure<'a> {
    commit_bucket: &'a CommitBucket,
    file_summs: Vec<&'a FileStatInfo>,
}

pub struct BaseReporter<'a> {
    config: &'a Config,
    reporter: Box<dyn Reporter<'a>>,
    report_info: ReportStructure<'a>,
}

impl<'a> BaseReporter<'a> {
    pub fn new(
        config: &'a Config,
        commit_bucket: &'a CommitBucket,
        reporter: Box<(dyn Reporter<'a> + 'static)>,
    ) -> BaseReporter<'a> {
        let report_info = ReportStructure {
            commit_bucket,
            file_summs: Self::map_file_summs(config, &commit_bucket.info.file_summs),
        };

        BaseReporter {
            config,
            reporter,
            report_info,
        }
    }

    pub fn output(&self) {
        self.reporter.output(&self.config, &self.report_info);
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
    fn output(&self, config: &Config, report_info: &ReportStructure<'a>) {
        let scopes = report_info
            .commit_bucket
            .info
            .scopes
            .keys()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let types = report_info
            .commit_bucket
            .info
            .types
            .keys()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        println!("{}: {}", "scopes".cyan().bold(), scopes);
        println!("{}: {}", "types".cyan().bold(), types);

        for file_sum in report_info.file_summs.iter() {
            println!(
                "{:<20}{:<10}{:<10}{:<10}",
                format!("{}:", file_sum.path),
                format!("+{}", file_sum.inserted).green(),
                format!("-{}", file_sum.deleted).red(),
                format!("{}", file_sum.total_changes)
                    .italic()
                    .bright_yellow()
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
