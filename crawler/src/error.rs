#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("fetch: {0}")]
    Fetch(#[from] FetchError),
    #[error("extract: {0}")]
    Extract(#[from] ExtractError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum FetchError {
    #[error("not found")]
    NotFound,
}

#[derive(thiserror::Error, Debug)]
pub enum ExtractError {}

pub type Result<T, E = Error> = std::result::Result<T, E>;
