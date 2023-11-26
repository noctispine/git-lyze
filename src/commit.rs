use crate::config::Config;
use crate::convention::ConventionBuilder;
use crate::customerror::Result;
use crate::repo::Repo;
use crate::utils::parse_date;
use git2::{Commit, DiffStatsFormat};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::vec;

#[derive(Serialize, Deserialize, Clone)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Stats {
    pub file_stat_infos: Vec<FileStatInfo>,
    pub changed_files_count: usize,
    pub insertions: usize,
    pub deletions: usize,
    pub total_changes: usize,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct FileStatInfo {
    pub path: String,
    pub inserted: usize,
    pub deleted: usize,
    pub total_changes: i64,
}

#[derive(Clone)]
pub struct CommitInfo {
    pub author: Author,
    pub summary: String,
    pub type_: String,
    pub scope: String,
    pub stats: Option<Stats>,
    pub time: i64,
}

#[derive(Clone)]
pub struct CommitBucket {
    pub commits: Vec<CommitInfo>,
    pub info: BucketInfo,
}

#[derive(Clone)]
pub struct BucketInfo {
    pub types: HashMap<String, u32>,
    pub scopes: HashMap<String, u32>,
    pub file_summs: HashMap<String, FileStatInfo>,
    pub total: usize,
}

impl CommitBucket {
    pub fn build(
        repo: &Repo,
        example_commit_message: &str,
        config: &Config,
    ) -> Result<CommitBucket> {
        let g_commits = repo.get_commits()?;

        let mut commits: Vec<CommitInfo> = vec![];

        let convention_builder = ConventionBuilder::build(example_commit_message);

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
                stats: Self::get_stats(&repo, &g_commit),
                time: g_commit.time().seconds(),
            };

            commits.push(commit_info);
        }

        let commits: Vec<CommitInfo> =
            commits
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
                        .filter_filenames
                        .as_ref()
                        .map_or(true, |filename_patterns| {
                            for pattern in filename_patterns {
                                let pattern = format!(r"{}", pattern);
                                let regex = Regex::new(&pattern).unwrap();
                                match &info.stats {
                                    Some(stats) => {
                                        for file_stat_info in &stats.file_stat_infos {
                                            println!(
                                                "{}: {} || is match: {}",
                                                pattern,
                                                file_stat_info.path,
                                                regex.is_match(&file_stat_info.path).to_string()
                                            );
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
                    config.start_date.as_ref().map_or(true, |start_date| {
                        parse_date(&start_date, &config.date_format, &config.date_format_type)
                            .map_or(true, |parsed_start_date| {
                                return info.time.cmp(&parsed_start_date).is_gt();
                            })
                    })
                })
                .filter(|info| {
                    config.end_date.as_ref().map_or(true, |end_date| {
                        parse_date(&end_date, &config.date_format, &config.date_format_type).map_or(
                            true,
                            |parsed_end_date| {
                                return info.time.cmp(&parsed_end_date).is_lt();
                            },
                        )
                    })
                })
                .collect();

        let bucket_info = Self::collect_bucket_info(&commits);

        Ok(CommitBucket {
            commits,
            info: bucket_info,
        })
    }

    fn get_stats(repo: &Repo, commit: &Commit) -> Option<Stats> {
        let raw_diff = match repo.get_diff(commit) {
            Some(diff) => diff,
            None => return None,
        };

        let diff_total: git2::DiffStats = match raw_diff.stats() {
            Ok(diff) => diff,
            Err(_) => return None,
        };

        let file_stat_infos = diff_total
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
                    total_changes: inserted as i64 - deleted as i64,
                }
            })
            .collect::<Vec<FileStatInfo>>();

        Some(Stats {
            file_stat_infos,
            changed_files_count: diff_total.files_changed(),
            deletions: diff_total.deletions(),
            insertions: diff_total.insertions(),
            total_changes: diff_total.deletions() + diff_total.insertions(),
        })
    }

    pub fn collect_bucket_info(commits: &Vec<CommitInfo>) -> BucketInfo {
        let mut file_summs: HashMap<String, FileStatInfo> = HashMap::new();
        let mut types: HashMap<String, u32> = HashMap::new();
        let mut scopes: HashMap<String, u32> = HashMap::new();
        let total = commits.len();

        for commit in commits.iter() {
            if !commit.type_.is_empty() {
                let c_commit_type = commit.type_.clone();
                let new_count = types.get(&c_commit_type).unwrap_or(&0) + 1;
                types.insert(c_commit_type, new_count);
            }

            if !commit.scope.is_empty() {
                let c_scope = commit.scope.clone();
                let new_count = scopes.get(&c_scope).unwrap_or(&0) + 1;
                scopes.insert(c_scope, new_count);
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
        assert_eq!(bucket.info.types.len(), 2);
        assert!(bucket.info.types.contains_key("feat") && bucket.info.types.contains_key("test"));

        let feat_val = bucket.info.types.get("feat");
        assert!(feat_val.is_some());
        assert_eq!(feat_val.unwrap(), &2);

        let main_val = bucket.info.scopes.get("main");
        assert!(main_val.is_some());
        assert_eq!(main_val.unwrap(), &2);
        assert_eq!(bucket.info.scopes.len(), 3);
        assert!(
            bucket.info.scopes.contains_key("main")
                && bucket.info.scopes.contains_key("commit")
                && bucket.info.scopes.contains_key("repo")
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
