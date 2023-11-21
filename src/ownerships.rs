use crate::commit::{CommitBucket, CommitInfo};
use crate::config::OwnershipConfig;
use regex::Regex;
use std::marker::PhantomData;

pub struct OwnershipBucket<'a> {
    pub cm_bucket: &'a CommitBucket,
    pub config: &'a OwnershipConfig,
}

pub struct Ownerships<'a, T> {
    p: PhantomData<&'a T>,
}

impl<'a, T> Ownerships<'a, T> {
    pub fn build(
        config: &'a Vec<OwnershipConfig>,
        bucket: &'a CommitBucket,
    ) -> Vec<OwnershipBucket<'a>> {
        let mut ownership_buckets: Vec<OwnershipBucket> = vec![];

        for conf in config {
            let mut commits: Vec<CommitInfo> = vec![];
            for cm in bucket.commits.iter() {
                for pattern in conf.patterns.iter() {
                    let rpattern = format!(r"{}", pattern);

                    let regex = Regex::new(&rpattern).unwrap();
                    if !regex.is_match(&cm.summary) {
                        continue;
                    }
                    // add commits
                    commits.push(cm.clone());
                    break;
                }
            }
            // create commit bucket with info and ownership bucket along with the config
            let info = CommitBucket::collect_bucket_info(&commits);
            ownership_buckets.push(OwnershipBucket {
                cm_bucket: &CommitBucket { commits, info },
                config: conf,
            })
        }
        ownership_buckets
    }
}
