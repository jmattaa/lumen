use crate::{
    git_entity::{git_commit::GitCommitError, git_diff::GitDiffError},
    provider::ProviderError,
};
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LumenError {
    #[error("{0}")]
    GitCommitError(#[from] GitCommitError),

    #[error("{0}")]
    GitDiffError(#[from] GitDiffError),

    #[error("Missing API key for {0}, use --api-key or LUMEN_API_KEY env variable")]
    MissingApiKey(String),

    #[error("Missing Model for {0}, use --model or LUMEN_MODEL env variable")]
    MissingModel(String),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("{0}")]
    CommandError(String),

    #[error(transparent)]
    ProvderError(#[from] ProviderError),
}
