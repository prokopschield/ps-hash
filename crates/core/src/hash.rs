use std::fmt::{Debug, Display};

use ps_base64::{base64, encode, sized_decode};
use ps_ecc::ReedSolomon;
use ps_pint16::PackedInt;
use ps_util::subarray;

use crate::{
    hash_inner, HashError, HashValidationError, DIGEST_SIZE, HASH_SIZE, HASH_SIZE_BIN,
    HASH_SIZE_COMPACT, MIN_RECOVERABLE, MIN_RECOVERABLE_BIN, PARITY, PARITY_OFFSET, PARITY_SIZE,
};

pub const RS: ReedSolomon = match ReedSolomon::new(PARITY) {
    Ok(rs) => rs,
    Err(_) => panic!("Failed to construct Reed-Solomon codec."),
};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Hash {
    inner: [u8; HASH_SIZE_BIN],
}

impl Hash {
    #[allow(clippy::self_named_constructors)]
    pub fn hash(data: impl AsRef<[u8]>) -> Result<Self, HashError> {
        let inner = hash_inner(data.as_ref())?;
        Ok(Self { inner })
    }

    pub fn validate_bin_vec(hash: &mut Vec<u8>) -> Result<Self, HashValidationError> {
        hash.resize(HASH_SIZE_BIN, 0xF4);

        let (data, parity) = hash.split_at_mut(PARITY_OFFSET);
        ReedSolomon::correct_detached_in_place(parity, data)?;

        let mut inner = [0u8; HASH_SIZE_BIN];
        inner.copy_from_slice(hash);

        Ok(Self { inner })
    }

    pub fn validate(bytes: impl AsRef<[u8]>) -> Result<Self, HashValidationError> {
        let bytes = bytes.as_ref();

        let mut hash = Self {
            inner: match bytes.len() {
                MIN_RECOVERABLE_BIN..=HASH_SIZE_BIN => {
                    let mut inner = [0xF4; HASH_SIZE_BIN];
                    inner[..bytes.len()].copy_from_slice(bytes);
                    inner
                }
                MIN_RECOVERABLE..=HASH_SIZE => sized_decode::<HASH_SIZE_BIN>(bytes),
                len => Err(HashValidationError::InvalidLength(len))?,
            },
        };

        let (data, parity) = hash.inner.split_at_mut(PARITY_OFFSET);
        ReedSolomon::correct_detached_in_place(parity, data)?;

        Ok(hash)
    }

    #[inline]
    #[must_use]
    pub fn compact(&self) -> &[u8] {
        &self.inner[..HASH_SIZE_COMPACT]
    }

    #[must_use]
    pub fn digest(&self) -> &[u8; DIGEST_SIZE] {
        subarray(&self.inner, 0)
    }

    #[must_use]
    pub fn data_max_len(&self) -> PackedInt {
        PackedInt::from_16_bits(&[self.inner[DIGEST_SIZE], self.inner[DIGEST_SIZE + 1]])
    }

    #[must_use]
    pub fn parity(&self) -> &[u8; PARITY_SIZE] {
        subarray(&self.inner, PARITY_OFFSET)
    }

    #[must_use]
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        encode(&self.inner)
    }
}

#[inline]
pub fn hash(data: impl AsRef<[u8]>) -> Result<Hash, HashError> {
    Hash::hash(data)
}

impl From<Hash> for [u8; HASH_SIZE] {
    fn from(hash: Hash) -> [u8; HASH_SIZE] {
        base64::sized_encode(&hash.inner)
    }
}

impl From<Hash> for String {
    fn from(value: Hash) -> Self {
        value.to_string()
    }
}

impl From<&Hash> for String {
    fn from(value: &Hash) -> Self {
        value.to_string()
    }
}

impl From<Hash> for Vec<u8> {
    fn from(value: Hash) -> Self {
        value.to_string().into_bytes()
    }
}

impl From<&Hash> for Vec<u8> {
    fn from(value: &Hash) -> Self {
        value.to_string().into_bytes()
    }
}

impl TryFrom<&[u8]> for Hash {
    type Error = HashValidationError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::validate(value)
    }
}

impl TryFrom<&str> for Hash {
    type Error = HashValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.as_bytes().try_into()
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        base64::encode_into(&self.inner, f)
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for Hash {}

impl PartialOrd for Hash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hash {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl std::hash::Hash for Hash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.inner);
    }
}
