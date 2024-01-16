use crate::cache::Cache;
use crate::config::Config;
use crate::convention::ConventionBuilder;
use crate::customerror::Result;
use crate::repo::Repo;
use crate::tracker::{Tracker, TrackerOpts};
use crate::utils::parse_date;
use colored::Color;
use git2::{Commit, DiffOptions, DiffStatsFormat};
use log::info;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::vec;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stats {
    pub cm_id: String,
    pub file_stat_infos: Vec<FileStatInfo>,
    pub changed_files_count: usize,
    pub insertions: usize,
    pub deletions: usize,
    pub total_changes: usize,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileStatInfo {
    pub path: String,
    pub inserted: usize,
    pub deleted: usize,
    pub total_changes: i64,
}

#[derive(Serialize, Clone, Debug)]
pub struct CommitInfo {
    pub author: Author,
    pub summary: String,
    pub type_: String,
    pub scope: String,
    pub stats: Option<Stats>,
    pub time: i64,
}

#[derive(Serialize, Clone)]
pub struct CommitBucket {
    pub commits: Vec<CommitInfo>,
    pub info: BucketInfo,
}

pub type FileSumms = HashMap<String, FileStatInfo>;

#[derive(Serialize, Debug, Clone)]
pub struct FreqInfo {
    pub count: u32,
    pub freq: f64,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Freq {
    pub total: u32,
    pub info: HashMap<String, FreqInfo>,
}

#[derive(Serialize, Clone)]
pub struct BucketInfo {
    pub types: Freq,
    pub scopes: Freq,
    pub file_summs: FileSumms,
    pub total: usize,
}

impl CommitBucket {
    pub fn build(
        repo: &Repo,
        example_commit_message: &str,
        config: &Config,
    ) -> Result<CommitBucket> {
        let mut perf_tracker = Tracker::new(&config, Color::Cyan, None);
        let mut cache = Cache::new(&format!(
            "{}/{}",
            &config.path.clone().unwrap_or("./".to_string()),
            &config.cache_path
        ));

        perf_tracker.start("commit :: get commits from repo");
        let g_commits = repo.get_commits()?;
        perf_tracker.stop();

        perf_tracker.start("commit :: filter commits by date");
        let g_commits =
            g_commits
                .iter()
                .filter(|commit| {
                    let is_after_start = config.start_date.as_ref().map_or(true, |start_date| {
                        parse_date(&start_date, &config.date_format, &config.date_format_type)
                            .map_or(true, |parsed_start_date| {
                                return commit.time().seconds().cmp(&parsed_start_date).is_ge();
                            })
                    });

                    let is_before_end =
                        config.end_date.as_ref().map_or(true, |end_date| {
                            parse_date(&end_date, &config.date_format, &config.date_format_type)
                                .map_or(true, |parsed_end_date| {
                                    return commit.time().seconds().cmp(&parsed_end_date).is_le();
                                })
                        });

                    is_after_start && is_before_end
                })
                .collect::<Vec<&Commit>>();

        perf_tracker.stop();

        let mut commits: Vec<CommitInfo> = vec![];

        perf_tracker.start("commit :: convention builder");
        let convention_builder = ConventionBuilder::build(example_commit_message);
        perf_tracker.stop();

        perf_tracker.start("commit :: parse commit wrt convention builder");
        for g_commit in g_commits {
            let parsed_message_info = convention_builder
                .construct_info(g_commit.summary().unwrap_or("").to_string())
                .unwrap_or_default();

            let commit_info: CommitInfo = CommitInfo {
                author: Author {
                    name: g_commit.author().name().unwrap_or("").to_string(),
                    email: g_commit.author().email().unwrap_or("").to_string(),
                },
                summary: g_commit.summary().unwrap_or("").to_string(),
                type_: parsed_message_info.type_,
                scope: parsed_message_info.optional_scope.unwrap_or("".to_string()),
                stats: Self::get_stats(&repo, &mut cache, &config, &g_commit),
                time: g_commit.time().seconds(),
            };

            commits.push(commit_info);
        }
        perf_tracker.stop();

        perf_tracker.start("commit :: filter commits by config");
        let commits: Vec<CommitInfo> = commits
            .into_iter()
            .filter(|info| {
                config
                    .filter_authors
                    .as_ref()
                    .map_or(true, |author| author.contains(&info.author.name))
            })
            .filter(|info| {
                config
                    .filter_scopes
                    .as_ref()
                    .map_or(true, |scope| scope.contains(&info.scope))
            })
            .filter(|info| {
                config
                    .filter_types
                    .as_ref()
                    .map_or(true, |type_| type_.contains(&info.type_))
            })
            .filter(|info| {
                config
                    .exclude_filename_patterns
                    .as_ref()
                    .map_or(true, |filename_patterns| {
                        for pattern in filename_patterns {
                            let pattern = format!(r"{}", pattern);
                            let regex = Regex::new(&pattern).unwrap();
                            match &info.stats {
                                Some(stats) => {
                                    for file_stat_info in &stats.file_stat_infos {
                                        if regex.is_match(&file_stat_info.path) {
                                            return false;
                                        }
                                        return true;
                                    }
                                }
                                None => return true,
                            }
                        }
                        true
                    })
            })
            .filter(|info| {
                config
                    .filter_filename_pattern
                    .as_ref()
                    .map_or(true, |filename_pattern| {
                        let regex = Regex::new(&format!(r"{}", filename_pattern)).unwrap();
                        if let Some(stats) = &info.stats {
                            for file_stat_info in &stats.file_stat_infos {
                                if !regex.is_match(file_stat_info.path.as_str()) {
                                    return false;
                                }
                                return true;
                            }
                        }
                        false
                    })
            })
            .collect();
        perf_tracker.stop();

        perf_tracker.start("commit :: collect bucket info");
        let bucket_info = Self::collect_bucket_info(&commits);
        perf_tracker.stop();

        Ok(CommitBucket {
            commits,
            info: bucket_info,
        })
    }

    fn get_stats(
        repo: &Repo,
        cache: &mut Cache,
        config: &Config,
        commit: &Commit,
    ) -> Option<Stats> {
        let mut file_stat_infos: Option<Vec<FileStatInfo>> = None;

        let mut perf_tracker = Tracker::new(
            &config,
            Color::Cyan,
            Some(TrackerOpts {
                write_once: Some(true),
            }),
        );
        perf_tracker.start("commit :: get_stats :: get commit stat from cache");
        if let Some(stats) = cache.get(commit.id().to_string()) {
            info!("get commit {} stats from cache", commit.id());
            if let Ok(stats) = serde_json::from_str(stats) {
                file_stat_infos = Some(stats);
            }
        };
        perf_tracker.stop();

        perf_tracker.start("commit :: get_stats :: compute commit diff");
        if file_stat_infos.is_none() {
            let mut diff_opts = DiffOptions::new();

            let raw_diff = match repo.get_diff(commit, Some(&mut diff_opts)) {
                Some(diff) => diff,
                None => return None,
            };

            let diff_total: git2::DiffStats = match raw_diff.stats() {
                Ok(diff) => diff,
                Err(_) => return None,
            };

            let mutated_diff_total = diff_total
                .to_buf(DiffStatsFormat::NUMBER, 1)
                .map_or(String::new(), |f| f.as_str().unwrap_or("").to_string())
                .lines()
                .map(|line| {
                    let normalized_line = line
                        .split(" ")
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                        .join(" ");
                    let normalized_line = normalized_line.split(" ").collect::<Vec<_>>();
                    let deleted = normalized_line[1].parse::<usize>().unwrap_or(0);
                    let inserted = normalized_line[0].parse::<usize>().unwrap_or(0);
                    FileStatInfo {
                        path: normalized_line[2].to_string(),
                        inserted,
                        deleted,
                        total_changes: inserted as i64 + deleted as i64,
                    }
                })
                .collect::<Vec<FileStatInfo>>();
            if let Ok(file_stat_infos_str) = serde_json::to_string(&mutated_diff_total) {
                info!("add {} to cache", commit.id().clone());
                cache.set(commit.id().clone().to_string(), file_stat_infos_str);
            }

            file_stat_infos = Some(mutated_diff_total);
        }
        perf_tracker.stop();

        perf_tracker.start("commit :: get_stats :: filter file info by file patterns");
        let filtered_file_stat_infos = file_stat_infos
            .unwrap()
            .into_iter()
            .filter(|file_stat_info| {
                match &config.filter_filename_pattern {
                    Some(pattern) => {
                        let pattern = format!(r"{}", pattern);
                        let regex = Regex::new(&pattern).unwrap();
                        if regex.is_match(&file_stat_info.path) {
                            return true;
                        }
                        return false;
                    }

                    None => return true,
                };
            })
            .collect::<Vec<FileStatInfo>>();
        perf_tracker.stop();

        let deletions = filtered_file_stat_infos
            .iter()
            .fold(0, |acc, stat| acc + stat.deleted);

        let insertions = filtered_file_stat_infos
            .iter()
            .fold(0, |acc, stat| acc + stat.inserted);

        let stats = Stats {
            cm_id: commit.id().to_string(),
            file_stat_infos: filtered_file_stat_infos.clone(),
            changed_files_count: filtered_file_stat_infos.len(),
            deletions,
            insertions,
            total_changes: insertions + deletions,
        };

        Some(stats)
    }

    pub fn collect_bucket_info(commits: &Vec<CommitInfo>) -> BucketInfo {
        let mut file_summs: HashMap<String, FileStatInfo> = HashMap::new();
        let mut types_count: HashMap<String, u32> = HashMap::new();
        let mut scopes_count: HashMap<String, u32> = HashMap::new();
        let total = commits.len();

        for commit in commits.iter() {
            if !commit.type_.is_empty() {
                let c_commit_type = commit.type_.clone();
                let new_count = types_count.get(&c_commit_type).unwrap_or(&0) + 1;
                types_count.insert(c_commit_type, new_count);
            }

            if !commit.scope.is_empty() {
                let c_scope = commit.scope.clone();
                let new_count = scopes_count.get(&c_scope).unwrap_or(&0) + 1;
                scopes_count.insert(c_scope, new_count);
            }

            match &commit.stats {
                Some(com_stat) => {
                    for stat in com_stat.file_stat_infos.iter() {
                        let prev_stat = file_summs.get(&stat.path);
                        match prev_stat {
                            Some(prev_stat) => {
                                file_summs.insert(
                                    prev_stat.path.clone(),
                                    FileStatInfo {
                                        path: prev_stat.path.clone(),
                                        inserted: prev_stat.inserted + stat.inserted,
                                        deleted: prev_stat.deleted + stat.deleted,
                                        total_changes: prev_stat.total_changes + stat.total_changes,
                                    },
                                );
                            }
                            None => {
                                file_summs.insert(
                                    stat.path.clone(),
                                    FileStatInfo {
                                        path: stat.path.clone(),
                                        inserted: stat.inserted,
                                        deleted: stat.deleted,
                                        total_changes: stat.total_changes,
                                    },
                                );
                            }
                        }
                    }
                }
                None => {}
            };
        }

        let types_total = types_count
            .clone()
            .into_iter()
            .fold(0, |acc, (_, value)| acc + value);

        let mut types = Freq {
            info: HashMap::new(),
            total: types_total,
        };

        let scopes_total = scopes_count
            .clone()
            .into_iter()
            .fold(0, |acc, (_, value)| acc + value);

        let mut scopes = Freq {
            total: scopes_total,
            info: HashMap::new(),
        };

        for (key, value) in types_count.into_iter() {
            types.info.insert(
                key.to_string(),
                FreqInfo {
                    count: value.to_owned(),
                    freq: value as f64 / types_total as f64,
                },
            );
        }

        for (key, value) in scopes_count.into_iter() {
            scopes.info.insert(
                key.to_string(),
                FreqInfo {
                    count: value.to_owned(),
                    freq: value as f64 / types_total as f64,
                },
            );
        }

        BucketInfo {
            types,
            scopes,
            file_summs,
            total,
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use crate::test_utils::mock_config;

    use super::*;
    use crate::test_utils::setup_repo;

    #[test]
    fn can_parse_commits() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let (repo, example_commit_message) = setup_repo(&temp_dir);
        let bucket =
            CommitBucket::build(&repo, example_commit_message.as_str(), &mock_config(None))
                .expect("Failed to build commit bucket");

        assert_eq!(bucket.commits.len(), 5);
        assert_eq!(bucket.info.types.total, 4);
        assert!(
            bucket.info.types.info.contains_key("feat")
                && bucket.info.types.info.contains_key("test")
        );

        let feat_val = bucket.info.types.info.get("feat");
        assert!(feat_val.is_some());
        assert_eq!(feat_val.unwrap().count, 2);

        let main_val = bucket.info.scopes.info.get("main");
        assert!(main_val.is_some());
        assert_eq!(main_val.unwrap().count, 2);
        assert_eq!(bucket.info.scopes.total, 4);
        assert!(
            bucket.info.scopes.info.contains_key("main")
                && bucket.info.scopes.info.contains_key("commit")
                && bucket.info.scopes.info.contains_key("repo")
        );
        assert_eq!(bucket.info.total, 5);
    }

    #[test]
    fn can_filter_author() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let (repo, example_commit_message) = setup_repo(&temp_dir);
        let bucket = CommitBucket::build(
            &repo,
            example_commit_message.as_str(),
            &mock_config(Some(vec!["", "--authors", "erencam"])),
        )
        .expect("Failed to build bucket");

        for commit in bucket.commits {
            assert_eq!(commit.author.name, "erencam");
        }
        assert_eq!(bucket.info.total, 4);
    }

    #[test]
    fn can_filter_multiple() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let (repo, example_commit_message) = setup_repo(&temp_dir);
        let bucket = CommitBucket::build(
            &repo,
            example_commit_message.as_str(),
            &mock_config(Some(vec![
                "",
                "--authors",
                "erencam",
                "--types",
                "feat",
                "-s",
                "main",
            ])),
        )
        .expect("Failed to build bucket");

        assert_eq!(bucket.info.total, 1);
        assert_eq!(bucket.commits[0].author.name, "erencam");
        assert_eq!(bucket.commits[0].type_, "feat");
        assert_eq!(bucket.commits[0].scope, "main");
    }
}
