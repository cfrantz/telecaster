use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown type")]
    UnknownType,
}
