use std::vec;

use clap::Parser;
use git2::{Repository, Signature};
use tempfile::TempDir;

use crate::{cache::Cache, config::Config, repo::Repo};

pub fn mock_config(args: Option<Vec<&str>>) -> Config {
    if let Some(args) = args {
        return Config::try_parse_from(args).unwrap();
    }

    Config::try_parse_from(vec![""]).unwrap()
}

pub fn setup_repo(temp_dir: &TempDir) -> (Repo, String) {
    let git_repo = Repository::init(temp_dir.path()).expect("Failed to create repository");
    let mut oid = git_repo
        .index()
        .expect("Failed to get index")
        .write_tree()
        .expect("Failed to get oid");
    let tree = git_repo.find_tree(oid).expect("Failed to get the tree");

    let signature = Signature::now("erencam", "erencam.dev@gmail.com").unwrap();
    let signature_second = Signature::now("unknown", "unknown@unknown.unknown").unwrap();

    let commit_messages = [
        "init",
        "test(commit): base tests",
        "test(main): idk",
        "feat(main): test",
        "feat(repo): idk",
    ];

    for (indx, commit_message) in commit_messages.iter().enumerate() {
        if indx > 0 {
            oid = git_repo
                .commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    commit_message,
                    &tree,
                    &[&git_repo.find_commit(oid).unwrap()],
                )
                .expect(format!("Failed to commit with message: {}", commit_message).as_str());
        } else {
            oid = git_repo
                .commit(
                    Some("HEAD"),
                    &signature_second,
                    &signature_second,
                    commit_message,
                    &tree,
                    &[],
                )
                .expect(format!("Failed to commit with message: {}", commit_message).as_str());
        }
    }

    (
        Repo::init(temp_dir.path()).expect("Failed to init repo"),
        "type(optional_scope): description".to_string(),
    )
}

pub fn setup_cache_dir(path: &String) {
    let mut cache = Cache::new(&path);
    cache.set("foo".to_string(), "bar".to_string());
    drop(cache);
}
