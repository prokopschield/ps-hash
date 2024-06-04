use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PsHashError {
    #[error("The input was too short and could not be decoded.")]
    InputTooShort,
    #[error("Reading from a slice failed.")]
    TryFromSliceError,
}

impl From<std::array::TryFromSliceError> for PsHashError {
    fn from(_error: std::array::TryFromSliceError) -> Self {
        PsHashError::TryFromSliceError
    }
}
