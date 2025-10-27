#![allow(clippy::module_name_repetitions)]

use ps_ecc::{RSDecodeError, RSGenerateParityError};
use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum HashError {
    #[error(transparent)]
    RSGenerateParityError(#[from] RSGenerateParityError),
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum HashValidationError {
    #[error("Invalid Hash length: {0}")]
    InvalidLength(usize),
    #[error(transparent)]
    RSDecodeError(#[from] RSDecodeError),
}
