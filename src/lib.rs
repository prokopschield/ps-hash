mod error;
pub use error::PsHashError;
use ps_pint16::PackedInt;
use sha2::{Digest, Sha256};
use std::fmt::Write;

#[cfg(test)]
pub mod tests;

pub const HASH_SIZE_BIN: usize = 32;
pub const HASH_SIZE: usize = 50;

pub fn sha256(data: &[u8]) -> [u8; HASH_SIZE_BIN] {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    result.into()
}

pub fn blake3(data: &[u8]) -> [u8; HASH_SIZE_BIN] {
    return *blake3::hash(data).as_bytes();
}

#[inline(always)]
pub fn xor<const S: usize>(a: [u8; S], b: [u8; S]) -> [u8; S] {
    let mut result = [0; S];

    for i in 0..S {
        result[i] = a[i] ^ b[i];
    }

    result
}

pub fn checksum_u32(data: &[u8], length: u32) -> u32 {
    let mut hash: u32 = length;

    for c in data.iter() {
        hash = (*c as u32)
            .wrapping_add(hash << 6)
            .wrapping_add(hash << 16)
            .wrapping_sub(hash);
    }

    hash
}

pub fn checksum(data: &[u8], length: u32) -> [u8; 4] {
    checksum_u32(data, length).to_le_bytes()
}

pub type HashParts = ([u8; HASH_SIZE_BIN], [u8; 4], PackedInt);

pub fn hash_to_parts(data: &[u8]) -> HashParts {
    let length = data.len();
    let shasum = sha256(data);
    let blasum = blake3(data);
    let xored = xor(shasum, blasum);
    let checksum = checksum(&xored, length as u32);

    (xored, checksum, PackedInt::from_usize(length))
}

/// a 50-byte ascii string representing a Hash
#[derive(Clone, Copy, Eq)]
#[repr(transparent)]
pub struct Hash {
    inner: [u8; HASH_SIZE],
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
            if validate_hash_char(b) {
                f.write_char(b as char)
            } else {
                f.write_str(&format!("<0x{:02X?}>", b))
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
        self.index(0..index.end + 1)
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
        self.index(0..index.end() + 1)
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
    fn from(hash: &Hash) -> String {
        hash.to_string()
    }
}

impl From<&Hash> for Vec<u8> {
    fn from(hash: &Hash) -> Vec<u8> {
        hash.to_vec()
    }
}

#[inline(always)]
/// validates that a byte is non-whitespace printable ascii
pub fn validate_hash_char(b: u8) -> bool {
    b > 0x20 && b < 0x7F
}

#[inline]
/// validates that a byte slice contains only non-whitespace printable ascii
pub fn validate_hash_str_bytes(str: &[u8]) -> bool {
    str.iter().all(|&b| validate_hash_char(b))
}

#[inline]
/// validates that a byte slice could be a valid hash
pub fn validate_hash_str(str: &[u8]) -> Result<&str, PsHashError> {
    if str.len() != HASH_SIZE {
        return Err(PsHashError::BadInputLength);
    }

    if !validate_hash_str_bytes(str) {
        return Err(PsHashError::BadInputByte);
    }

    let validated = unsafe {
        // safe bacause str is checked to be valid ascii
        std::str::from_utf8_unchecked(str)
    };

    Ok(validated)
}

impl TryFrom<&[u8]> for Hash {
    type Error = PsHashError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let validated = validate_hash_str(value)?.as_bytes();

        let hash = Self {
            inner: validated.try_into()?,
        };

        Ok(hash)
    }
}

impl TryFrom<&str> for Hash {
    type Error = PsHashError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.as_bytes().try_into()
    }
}

impl Hash {
    pub fn as_bytes(&self) -> &[u8; HASH_SIZE] {
        &self.inner
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            // safe because Hash is guaranteed to be valid ASCII
            std::str::from_utf8_unchecked(&self.inner)
        }
    }

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

pub fn hash(data: &[u8]) -> Hash {
    encode_parts(hash_to_parts(data))
}

pub fn decode_parts(hash: &[u8]) -> Result<HashParts, PsHashError> {
    if hash.len() < HASH_SIZE {
        return Err(PsHashError::InputTooShort);
    }

    let bytes = ps_base64::decode(hash);

    Ok((
        bytes[0..HASH_SIZE_BIN].try_into()?,
        bytes[HASH_SIZE_BIN..36].try_into()?,
        PackedInt::from_12_bits(&bytes[36..38].try_into()?),
    ))
}

pub fn verify_hash_integrity(hash: &[u8]) -> bool {
    let parts = match decode_parts(hash) {
        Ok(parts) => parts,
        Err(_) => return false,
    };

    for i in 0..4 {
        if parts.1 == checksum(&parts.0, (parts.2.to_u32() + i) << 12) {
            return true;
        }
    }

    false
}
