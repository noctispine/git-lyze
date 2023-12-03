use crate::{
    commit::{CommitBucket, FileStatInfo},
    config::Config,
    ownerships::Ownerships,
    utils::map_file_summs,
};
use colored::Colorize;

pub trait Reporter<'a> {
    fn output(
        &self,
        config: &Config,
        report_info: &CommitBucket,
        ownerships_info: &Option<Ownerships<'a>>,
    );
    fn output_commit_bucket(&self, bucket: &CommitBucket);
    fn output_file_summs(&self, file_stat_infos: Vec<&FileStatInfo>);
}

pub struct BaseReporter<'a> {
    config: &'a Config,
    reporter: Box<dyn Reporter<'a>>,
    general_info: &'a CommitBucket,
    ownerships_info: Option<Ownerships<'a>>,
}

impl<'a> BaseReporter<'a> {
    pub fn new(
        config: &'a Config,
        commit_bucket: &'a CommitBucket,
        reporter: Box<(dyn Reporter<'a> + 'static)>,
    ) -> BaseReporter<'a> {
        let ownerships_info = match &config.ownerships {
            Some(conf) => Some(Ownerships::build(&conf, commit_bucket)),
            None => None,
        };

        BaseReporter {
            config,
            reporter,
            general_info: commit_bucket,
            ownerships_info,
        }
    }

    pub fn output(&self) {
        self.reporter
            .output(&self.config, &self.general_info, &self.ownerships_info);
    }
}

pub struct Stdout {}

impl<'a> Reporter<'a> for Stdout {
    fn output(
        &self,
        config: &Config,
        general_info: &CommitBucket,
        ownerships_info: &Option<Ownerships<'a>>,
    ) {
        println!("\n=====================");
        println!("|| General Summary ||");
        println!("=====================\n");
        self.output_commit_bucket(&general_info);
        self.output_file_summs(map_file_summs(
            &config,
            &general_info.info.file_summs,
        ));

        if let Some(ow_info) = ownerships_info {
            let ow_buckets = &ow_info.ow_buckets;
            println!("\n========================");
            println!("|| Ownerships Summary ||");
            println!("========================\n");
            println!("length: {}", ow_buckets.len());
            for info in ow_buckets.iter() {
                println!("\n{}: {}", "owner".cyan(), info.config.name.to_string());
                println!("{}: {}", "authors".green(), info.config.authors.join(""));
                self.output_commit_bucket(&info.cm_bucket);
                let file_summs = map_file_summs(&config, &info.cm_bucket.info.file_summs);
                self.output_file_summs(file_summs);
            }
        }
    }

    fn output_commit_bucket(&self, bucket: &CommitBucket) {
        let scopes = bucket
            .info
            .scopes
            .keys()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let types = bucket
            .info
            .types
            .keys()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        println!("{}: {}", "scopes".cyan().bold(), scopes);
        println!("{}: {}", "types".cyan().bold(), types);
    }

    fn output_file_summs(&self, file_stat_infos: Vec<&FileStatInfo>) {
        for file_sum in file_stat_infos.iter() {
            println!(
                "{:<30}{:<10}{:<10}{:<10}",
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
