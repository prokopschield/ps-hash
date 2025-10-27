#![allow(clippy::module_name_repetitions)]

use ps_ecc::{RSDecodeError, RSGenerateParityError};
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PsHashError {
    #[error("The input was too short and could not be decoded.")]
    InputTooShort,
    #[error("Reading from a slice failed.")]
    TryFromSliceError,
    #[error("When converting &[u8] to Hash: incorrect length")]
    BadInputLength,
    #[error("When converting &[u8] to Hash: invalid byte")]
    BadInputByte,
}

impl From<std::array::TryFromSliceError> for PsHashError {
    fn from(_error: std::array::TryFromSliceError) -> Self {
        Self::TryFromSliceError
    }
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum HashError {
    #[error(transparent)]
    RSGenerateParityError(#[from] RSGenerateParityError),
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum HashValidationError {
    #[error(transparent)]
    RSDecodeError(#[from] RSDecodeError),
}
