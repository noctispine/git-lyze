use crate::commit::{BucketInfo, CommitBucket};
use crate::config::OwnershipConfig;
use regex::Regex;

pub struct OwnershipBucket<'a> {
    pub cm_bucket: &'a CommitBucket,
    pub config: &'a OwnershipConfig,
    pub info: BucketInfo,
}

pub struct Ownerships<'a> {
    pub buckets: Vec<OwnershipBucket<'a>>,
}

impl<'a> Ownerships<'a> {
    pub fn build(config: &'a Vec<OwnershipConfig>, bucket: &'a CommitBucket) -> Ownerships<'a> {
        let mut ownership_buckets: Vec<OwnershipBucket> = vec![];

        for conf in config {
            for cm in bucket.commits.iter() {
                for pattern in conf.patterns.iter() {
                    let rpattern = format!(r"{}", pattern);

                    let regex = Regex::new(&rpattern).unwrap();
                    if !regex.is_match(&cm.summary) {
                        continue;
                    }

                    ownership_buckets.push(OwnershipBucket {
                        cm_bucket: bucket,
                        config: conf,
                        info: CommitBucket::collect_bucket_info(bucket.commits.iter().collect()),
                    });
                }
            }
        }

        Ownerships {
            buckets: ownership_buckets,
        }
    }
}

// fn get_ownerships(&self, configs: &Vec<OwnershipConfig>) -> Vec<Ownerships> {
//     let mut ownerhips: Vec<Ownerships> = vec![];
//     for owner_ship_config in configs.iter() {
//         let mut commits: Vec<&CommitInfo> = vec![];
//         for cm in self.commits.iter() {
//             let pattern = format!(r"{}", owner_ship_config.pattern);
//             let regex = Regex::new(&pattern).unwrap();

//             if !regex.is_match(&cm.summary) {
//                 continue;
//             }

//             commits.push(cm);
//         }

//         // let (total, types, scopes, file_summs) = Self::collect_bucket_info(commits);
//         // ownerhips.push(Ownerships {
//         //     info: owner_ship_config,
//         //     cm_bucket: CommitBucket {
//         //         commits: commits.iter().cloned(),
//         //         types,
//         //         scopes,
//         //         file_summs,
//         //         total,
//         //     },
//         // })
//     }

//     ownerhips
// }
