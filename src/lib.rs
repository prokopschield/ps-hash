mod error;
pub use error::PsHashError;
use error::{HashError, HashValidationError};
use ps_base64::{base64, sized_encode};
use ps_buffer::Buffer;
use ps_ecc::ReedSolomon;
use ps_pint16::PackedInt;
use sha2::{Digest, Sha256};
use std::fmt::Write;

#[cfg(test)]
pub mod tests;

pub const HASH_SIZE_BIN: usize = 32;
pub const HASH_SIZE: usize = 64;
pub const PARITY: usize = 7;
pub const PARITY_SIZE: usize = 14;
pub const SIZE_SIZE: usize = std::mem::size_of::<u16>();

pub const RS: ReedSolomon = match ReedSolomon::new(PARITY as u8) {
    Ok(rs) => rs,
    Err(_) => panic!("Failed to construct Reed-Solomon codec."),
};

#[inline(always)]
#[must_use]
pub fn sha256(data: &[u8]) -> [u8; HASH_SIZE_BIN] {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    result.into()
}

#[inline(always)]
#[must_use]
pub fn blake3(data: &[u8]) -> blake3::Hash {
    blake3::hash(data)
}

pub type HashParts = ([u8; HASH_SIZE_BIN], [u8; PARITY_SIZE], PackedInt);

/// a 64-byte ascii string representing a Hash
#[derive(Clone, Copy, Eq)]
#[repr(transparent)]
pub struct Hash {
    inner: [u8; HASH_SIZE],
}

impl Hash {
    /// Calculated the [`Hash`] of `data`.
    ///
    /// # Errors
    ///
    /// - [`HashError::BufferError`] is returned if an allocation fails.
    /// - [`HashError::RSGenerateParityError`] is returned if generating parity fails.
    pub fn hash<T: AsRef<[u8]>>(data: T) -> Result<Self, HashError> {
        let data = data.as_ref();
        let mut buffer = Buffer::with_capacity(HASH_SIZE)?;

        buffer.extend_from_slice(sha256(data))?;
        buffer ^= &blake3(data).as_bytes()[..];
        buffer.extend_from_slice(PackedInt::from_usize(data.len()).to_16_bits())?;
        buffer.extend_from_slice(RS.generate_parity(&buffer)?)?;

        let hash = Self {
            inner: sized_encode::<HASH_SIZE>(&buffer),
        };

        Ok(hash)
    }

    /// Validates and corrects a [`Hash`].
    ///
    /// # Errors
    ///
    /// - [`HashValidationError::RSDecodeError`] is returned if the hash is unrecoverable.
    pub fn validate<T: AsRef<[u8]>>(hash: T) -> Result<Self, HashValidationError> {
        let mut hash = base64::decode(hash.as_ref());
        let (data, parity) = hash.split_at_mut(36);

        ReedSolomon::correct_detached_in_place(parity, data)?;

        let hash = Self {
            inner: sized_encode::<HASH_SIZE>(&hash),
        };

        Ok(hash)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for Hash {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &b in &self.inner {
            if b.is_ascii_graphic() {
                f.write_char(b as char)
            } else {
                f.write_str(&format!("<0x{b:02X?}>"))
            }?;
        }

        Ok(())
    }
}

impl core::hash::Hash for Hash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match decode_parts(&self.inner) {
            Ok((hash, checksum, length)) => {
                state.write(&hash);
                state.write(&checksum);
                state.write_u16(length.to_inner_u16());
            }
            Err(_) => state.write(&self.inner),
        }
    }
}

impl std::ops::Deref for Hash {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl std::ops::Index<usize> for Hash {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.inner.len() {
            &self.inner[index]
        } else {
            &0
        }
    }
}

impl std::ops::Index<std::ops::Range<usize>> for Hash {
    type Output = str;

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        let start = std::cmp::min(index.start, self.inner.len());
        let end = std::cmp::min(index.end, self.inner.len());
        let range = start..end;

        &self.as_str()[range]
    }
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for Hash {
    type Output = str;

    fn index(&self, index: std::ops::RangeFrom<usize>) -> &Self::Output {
        self.index(index.start..HASH_SIZE)
    }
}

impl std::ops::Index<std::ops::RangeTo<usize>> for Hash {
    type Output = str;

    fn index(&self, index: std::ops::RangeTo<usize>) -> &Self::Output {
        self.index(0..index.end)
    }
}

impl std::ops::Index<std::ops::RangeToInclusive<usize>> for Hash {
    type Output = str;

    fn index(&self, index: std::ops::RangeToInclusive<usize>) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl std::ops::Index<std::ops::RangeFull> for Hash {
    type Output = str;

    fn index(&self, _: std::ops::RangeFull) -> &Self::Output {
        return self.as_str();
    }
}

impl std::ops::Index<std::ops::RangeInclusive<usize>> for Hash {
    type Output = str;

    fn index(&self, index: std::ops::RangeInclusive<usize>) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {
        let left = match decode_parts(&self.inner) {
            Ok(parts) => parts,
            Err(_) => return self.inner == other.inner,
        };

        let right = match decode_parts(&other.inner) {
            Ok(parts) => parts,
            Err(_) => return false,
        };

        left.0 == right.0 && left.1 == right.1 && left.2 == right.2
    }
}

impl Ord for Hash {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = match decode_parts(&self.inner) {
            Ok(left) => left,
            Err(_) => return self.inner.cmp(&other.inner),
        };

        let right = match decode_parts(&other.inner) {
            Ok(right) => right,
            Err(_) => return self.inner.cmp(&other.inner),
        };

        left.0.cmp(&right.0)
    }
}

impl PartialOrd for Hash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Hash> for [u8; HASH_SIZE] {
    fn from(hash: Hash) -> [u8; HASH_SIZE] {
        hash.inner
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
    pub const fn as_bytes(&self) -> &[u8; HASH_SIZE] {
        &self.inner
    }

    #[must_use]
    pub const fn as_str(&self) -> &str {
        unsafe {
            // safe because Hash is guaranteed to be valid ASCII
            std::str::from_utf8_unchecked(&self.inner)
        }
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.to_vec()
    }

    /// This should tell you how large a vector to allocate if you want to copy the hashed data.
    pub fn data_max_len(&self) -> Result<usize, PsHashError> {
        let bits = &self.inner[48..HASH_SIZE];
        let bits = ps_base64::decode(bits);
        let bits = bits[0..2].try_into()?;
        let size = PackedInt::from_12_bits(bits).to_usize();

        Ok(size)
    }
}

#[must_use]
pub fn encode_parts(parts: HashParts) -> Hash {
    let (xored, checksum, length) = parts;

    let mut vec: Vec<u8> = Vec::with_capacity(38);

    vec.extend_from_slice(&xored);
    vec.extend_from_slice(&checksum);
    vec.extend_from_slice(&length.to_12_bits());

    Hash {
        inner: ps_base64::sized_encode::<HASH_SIZE>(&vec),
    }
}

#[inline]
pub fn hash<T: AsRef<[u8]>>(data: T) -> Result<Hash, HashError> {
    Hash::hash(data)
}

pub fn decode_parts(hash: &[u8]) -> Result<HashParts, PsHashError> {
    if hash.len() < HASH_SIZE {
        return Err(PsHashError::InputTooShort);
    }

    let bytes = ps_base64::decode(hash);

    Ok((
        bytes[0..32].try_into()?,
        bytes[34..48].try_into()?,
        PackedInt::from_16_bits(&bytes[32..34].try_into()?),
    ))
}
