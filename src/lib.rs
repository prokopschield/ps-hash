#![allow(clippy::missing_errors_doc)]
mod error;
mod implementations;
mod methods;
pub use error::*;
use ps_base64::base64;
use ps_ecc::ReedSolomon;
use ps_pint16::PackedInt;
use sha2::{Digest, Sha256};

#[cfg(test)]
pub mod tests;

pub const DIGEST_SIZE: usize = 32;
pub const HASH_SIZE_BIN: usize = 48;
pub const HASH_SIZE_COMPACT: usize = 42;
pub const HASH_SIZE: usize = 64;
pub const PARITY: u8 = 7;
pub const PARITY_OFFSET: usize = 34;
pub const PARITY_SIZE: usize = 14;
pub const SIZE_SIZE: usize = std::mem::size_of::<u16>();
/// The minimum number of characters for a Hash to still be safely recoverable.
pub const MIN_RECOVERABLE: usize = HASH_SIZE - (PARITY as usize * 8 / 6);
/// The minimum number of bytes for a Hash to still be safely recoverable.
pub const MIN_RECOVERABLE_BIN: usize = HASH_SIZE_BIN - (PARITY as usize);

pub const RS: ReedSolomon = match ReedSolomon::new(PARITY) {
    Ok(rs) => rs,
    Err(_) => panic!("Failed to construct Reed-Solomon codec."),
};

#[inline]
#[must_use]
pub fn sha256(data: &[u8]) -> [u8; DIGEST_SIZE] {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    result.into()
}

#[inline]
#[must_use]
pub fn blake3(data: &[u8]) -> blake3::Hash {
    blake3::hash(data)
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Hash {
    inner: [u8; HASH_SIZE_BIN],
}

impl Hash {
    /// Calculates the [`Hash`] of `data`.
    ///
    /// # Errors
    ///
    /// - [`HashError::RSGenerateParityError`] is returned if generating parity fails.
    #[allow(clippy::self_named_constructors)]
    pub fn hash(data: impl AsRef<[u8]>) -> Result<Self, HashError> {
        let data = data.as_ref();
        let mut inner = [0u8; HASH_SIZE_BIN];

        let sha = sha256(data);
        let blake = blake3(data);

        // XOR digests
        for i in 0..DIGEST_SIZE {
            inner[i] = sha[i] ^ blake.as_bytes()[i];
        }

        // Copy length
        inner[DIGEST_SIZE..PARITY_OFFSET]
            .copy_from_slice(&PackedInt::from_usize(data.len()).to_16_bits());

        // Generate and copy parity
        let parity = RS.generate_parity(&inner[..PARITY_OFFSET])?;
        inner[PARITY_OFFSET..].copy_from_slice(&parity);

        Ok(Self { inner })
    }

    /// Validates and corrects a binary-encoded [`Hash`].\
    /// The correction happens on the provided [`Vec`].
    ///
    /// # Errors
    ///
    /// - [`HashValidationError::RSDecodeError`] is returned if the hash is unrecoverable.
    pub fn validate_bin_vec(hash: &mut Vec<u8>) -> Result<Self, HashValidationError> {
        // The constant 0xF4 is chosen arbitrarily.
        // Using 0x00 would produce Ok(AAA...AAA) for all short inputs.
        hash.resize(HASH_SIZE_BIN, 0xF4);

        let (data, parity) = hash.split_at_mut(PARITY_OFFSET);

        ReedSolomon::correct_detached_in_place(parity, data)?;

        let mut inner = [0u8; HASH_SIZE_BIN];

        inner.copy_from_slice(hash);

        let hash = Self { inner };

        Ok(hash)
    }
}
impl From<Hash> for [u8; HASH_SIZE] {
    fn from(hash: Hash) -> [u8; HASH_SIZE] {
        base64::sized_encode(&hash.inner)
    }
}

impl From<&Hash> for String {
    fn from(hash: &Hash) -> Self {
        hash.to_string()
    }
}

impl From<&Hash> for Vec<u8> {
    fn from(hash: &Hash) -> Self {
        hash.to_vec()
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

impl Hash {
    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

#[inline]
pub fn hash(data: impl AsRef<[u8]>) -> Result<Hash, HashError> {
    Hash::hash(data)
}
