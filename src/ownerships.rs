use crate::commit::{CommitBucket, CommitInfo};
use crate::config::OwnershipConfig;
use regex::Regex;

pub struct OwnershipBucket<'a> {
    pub cm_bucket: CommitBucket,
    pub config: &'a OwnershipConfig,
}

pub struct Ownerships<'a> {
    pub ow_buckets: Vec<OwnershipBucket<'a>>,
}

impl<'a> Ownerships<'a> {
    pub fn build(config: &'a Vec<OwnershipConfig>, bucket: &'a CommitBucket) -> Self {
        let mut ow_buckets: Vec<OwnershipBucket> = vec![];

        for conf in config {
            let mut commits: Vec<CommitInfo> = vec![];
            for cm in bucket.commits.iter() {
                for pattern in conf.patterns.iter() {
                    let rpattern = format!(r"{}", pattern);

                    let regex = Regex::new(&rpattern).unwrap();
                    if !regex.is_match(&cm.summary) {
                        continue;
                    }
                    commits.push(cm.clone());
                    break;
                }
            }
            let info = CommitBucket::collect_bucket_info(&commits);
            ow_buckets.push(OwnershipBucket {
                cm_bucket: CommitBucket { commits, info },
                config: conf,
            })
        }

        Ownerships { ow_buckets }
    }
}
