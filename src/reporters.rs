use std::ops::Mul;

use crate::{
    commit::{CommitBucket, FileSumms},
    config::Config,
    ownerships::Ownerships,
    utils::map_file_summs,
    window::{app::App, screen::render_screen},
};
use ratatui::widgets::{Row, TableState};

pub trait Reporter<'a> {
    fn output(
        &self,
        config: &Config,
        report_info: &CommitBucket,
        ownerships_info: &Option<Ownerships<'a>>,
    );
    // fn output_commit_bucket(&self, bucket: &CommitBucket);
    // fn output_file_summs(&self, file_summs: &FileSumms);
}

pub struct BaseReporter<'a> {
    config: &'a Config,
    reporter: Box<dyn Reporter<'a>>,
    bucket: &'a CommitBucket,
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
            bucket: commit_bucket,
            ownerships_info,
        }
    }

    pub fn output(&self) {
        self.reporter
            .output(&self.config, &self.bucket, &self.ownerships_info);
    }
}

pub struct Stdout {}

impl<'a> Reporter<'a> for Stdout {
    fn output(
        &self,
        config: &Config,
        report_info: &CommitBucket,
        _ownerships_info: &Option<Ownerships<'a>>,
    ) {
        // let (mut scopes, mut types, mut files_summs) = (
        //     vec![vec![String::from("scopes"), String::from("count")]],
        //     vec![vec![String::from("types"), String::from("count")]],
        //     vec![vec![
        //         String::from("file path"),
        //         String::from("total changes"),
        //         String::from("insertions"),
        //         String::from("deletions"),
        //     ]],
        // );

        // for (_type, count) in report_info.info.types.iter() {
        //     types.push(vec![_type.clone(), count.clone().to_string()]);
        // }

        // let mut type_table = Builder::from(types).build();
        // type_table
        //     .with(Style::modern())
        //     .with(ColumnNames::default());
        //
        // for (scope, count) in report_info.info.scopes.iter() {
        //     scopes.push(vec![scope.clone(), count.clone().to_string()]);
        // }
        //
        // let mut scope_table = Builder::from(scopes).build();
        // scope_table
        //     .with(Style::modern())
        //     .with(ColumnNames::default());
        //
        let file_summs = map_file_summs(&config, &report_info.info.file_summs);
        // for file_summ in file_summs.iter() {
        //     files_summs.push(vec![
        //         file_summ.path.to_string(),
        //         file_summ.total_changes.to_string(),
        //         file_summ.inserted.to_string(),
        //         file_summ.deleted.to_string(),
        //     ]);
        // }
        // let mut files_summs_table = Builder::from(files_summs).build();
        // files_summs_table
        //     .with(Style::modern())
        //     .with(ColumnNames::default());
        //
        // println!("{type_table}\n{scope_table}\n{files_summs_table}");
        //
        //

        let mut types: Vec<(String, u64)> = vec![];

        for (_type, info) in &report_info.info.types.info {
            let type_pct = (info.freq * 100.0).round() as u64;
            types.push((_type.to_string(), type_pct));
        }
        render_screen(&mut App {
            counter: 0,
            should_quit: false,
            active_tab: 0,
            titles: vec!["Stats", "Files"],
            file_summs,
            types,
            sort_file_summs: config.sort_files.clone(),
            file_table_state: TableState::default().with_selected(Some(0)),
        });
    }
}
