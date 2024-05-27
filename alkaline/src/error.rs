pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Other(anyhow::Error),
}
#[derive(Debug, thiserror::Error)]
pub enum AdapterReadError {
    #[error("adapter does not impl read")]
    NotImpl,
    #[error(transparent)]
    Other(anyhow::Error),
}
