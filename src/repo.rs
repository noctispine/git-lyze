use crate::customerror::{
    Error,
    Result
};
use std::path::Path;
use git2::{
    Repository,
    Commit, Sort
};
use std::io;


pub struct Repo {
    dot_git: Repository
}

impl Repo {
    pub fn init(path: &Path) -> Result<Self> {
        if path.exists() {
            Ok(Self {
                dot_git: Repository::open(path)?
            })
        } else {
            Err(Error::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                "repository path not found",
            )))
        }
    }

    pub fn get_commits(&self) -> Result<Vec<Commit>> {
        let mut walk = self.dot_git.revwalk()?;
		walk.set_sorting(Sort::TOPOLOGICAL)?;

        walk.push_head()?;

        let commits: Vec<Commit> = walk
            .filter_map(|oid| oid.ok())
            .filter_map(|oid| self.dot_git.find_commit(oid).ok())
            .collect();

        Ok(commits)
    }

    pub fn find_last_commit(&self) -> Result<Commit> {
        let obj = self.dot_git.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
        obj.into_commit().map_err(|_| Error::GitError(git2::Error::from_str("Couldn't find the commit")))
        
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use tempfile::TempDir;
    use std::cell::RefCell;


    #[test]
    fn test_init_existing_path() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        Repository::init(temp_dir.path()).expect("Failed to create temp repository");

        println!("created tempdir: {:?}", temp_dir.path());
        let result = Repo::init(temp_dir.path());

        assert!(result.is_ok());

        temp_dir.close().expect("Failed to close temp dir");
    }

    #[test]
    fn test_init_nonexistent_path() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let non_existent_path = temp_dir.path().join(PathBuf::from("/nonexistentdir"));

        let result = Repo::init(&non_existent_path);

        assert!(result.is_err());
        temp_dir.close().expect("Failed to close temp dir")
    }

    #[test]
    fn can_get_commits() {

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        Repository::init(temp_dir.path()).expect("Failed to create temp repository");
        let repo = RefCell::new(Repo::init(temp_dir.path()).expect("Failed to get the repo"));
        let b_repo = repo.borrow_mut();


        let mut index = b_repo.dot_git.index().unwrap();
        let oid = index.write_tree().unwrap();
        let signature = git2::Signature::now("Eren", "erencam.dev@gmail.com").unwrap();
        let message = "feat(repo): add get_commits";
        let tree = b_repo.dot_git.find_tree(oid).unwrap();


        b_repo.dot_git.commit(
            Some("HEAD"), 
            &signature, 
            &signature, 
            message, 
            &tree, 
            &[]
        ).unwrap();

        let parent = b_repo.find_last_commit().unwrap();

        b_repo.dot_git.commit(
            Some("HEAD"), 
            &signature, 
            &signature, 
            message, 
            &tree, 
            &[&parent]
        ).unwrap();
        


        let commits = b_repo.get_commits();
        assert!(commits.is_ok());
        assert_eq!(commits.unwrap().len(), 2);
        

    }
}