use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[cfg(feature = "repo")]
    #[error("Git error: `${0}`")]
    GitError(#[from] git2::Error),

    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: `{0}`")]
    ParseError(String),
}

pub type Result<T> = core::result::Result<T, Error>;
