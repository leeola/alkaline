pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Other(anyhow::Error),
}
