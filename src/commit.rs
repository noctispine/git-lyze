use std::collections::HashSet;

use crate::convention::ConventionBuilder;
use crate::repo::Repo;
use crate::customerror::Result;
struct Author {
    name: String,
    email: String
}

struct CommitInfo {
    author: Author,
    summary: String,
    type_: String,
    scope: String
}

struct CommitBucket {
    commits: Vec<CommitInfo>,
    types: HashSet<String>,
    scopes: HashSet<String>,
    total: usize
}

impl CommitBucket {
    pub fn new(
        repo: Repo,
        example_commit_message: String
    ) -> Result<CommitBucket> {
        let g_commits = repo.get_commits()?;
        let mut commits: Vec<CommitInfo> = vec![];
        let mut types: HashSet<String> = HashSet::new();
        let mut scopes: HashSet<String> = HashSet::new();

        let convention_builder = ConventionBuilder::build(example_commit_message);
        
        for g_commit in g_commits {
            let parsed_message_info = convention_builder
                .construct_info(g_commit
                .summary()
                .unwrap_or("")
                .to_string())
                .unwrap_or_default();

            types.insert(parsed_message_info.type_.clone());

            scopes.insert(parsed_message_info.optional_scope
                    .clone()
                    .unwrap_or(""
                    .to_string())
                );

            commits.push(
                CommitInfo { 
                    author: Author { 
                        name: g_commit.author().name().unwrap_or("").to_string(),
                        email: g_commit.author().email().unwrap_or("").to_string(),
                    }, 
                    summary: g_commit.summary().unwrap_or("").to_string(),
                    type_: parsed_message_info.type_,
                    scope: parsed_message_info.optional_scope.unwrap_or("".to_string())
                },
            )
        }

        let total = commits.len();

        Ok(CommitBucket { commits, total, types, scopes })
    }
}
