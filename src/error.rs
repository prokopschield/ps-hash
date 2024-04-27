#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PsHashError {
    InputTooShort,
    TryFromSliceError,
}

impl From<std::array::TryFromSliceError> for PsHashError {
    fn from(_error: std::array::TryFromSliceError) -> Self {
        PsHashError::TryFromSliceError
    }
}
