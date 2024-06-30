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
        PsHashError::TryFromSliceError
    }
}
