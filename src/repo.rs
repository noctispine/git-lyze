use crate::customerror::{
    Error,
    Result
};
use std::path::Path;
use git2::Repository;
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
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use tempfile::TempDir;

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
}