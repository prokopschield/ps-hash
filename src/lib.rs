mod error;
pub use error::PsHashError;
use ps_pint16::PackedInt;
use sha2::{Digest, Sha256};
use std::fmt::Write;

pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    return result.into();
}

pub fn blake3(data: &[u8]) -> [u8; 32] {
    return *blake3::hash(data).as_bytes();
}

pub fn xor(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let mut result = [0; 32];

    for i in 0..32 {
        result[i] = a[i] ^ b[i];
    }

    return result;
}

pub fn checksum_u32(data: &[u8], length: u32) -> u32 {
    let mut hash: u32 = length;

    for c in data.iter() {
        hash = (*c as u32)
            .wrapping_add(hash << 6)
            .wrapping_add(hash << 16)
            .wrapping_sub(hash);
    }

    return hash;
}

pub fn checksum(data: &[u8], length: u32) -> [u8; 4] {
    checksum_u32(data, length).to_le_bytes()
}

pub type HashParts = ([u8; 32], [u8; 4], PackedInt);

pub fn hash_to_parts(data: &[u8]) -> HashParts {
    let length = data.len();
    let shasum = sha256(data);
    let blasum = blake3(data);
    let xored = xor(shasum, blasum);
    let checksum = checksum(&xored, length as u32);

    return (xored, checksum, PackedInt::from_usize(length));
}

/// a 50-byte ascii string representing a Hash
#[derive(Clone, Copy, Eq)]
#[repr(transparent)]
pub struct Hash {
    inner: [u8; 50],
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

        return left.0.cmp(&right.0);
    }
}

impl PartialOrd for Hash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Hash> for [u8; 50] {
    fn from(hash: Hash) -> [u8; 50] {
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
    if str.len() != 50 {
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
    pub fn as_slice(&self) -> &[u8; 50] {
        &self.inner
    }

    pub fn as_bytes(&self) -> &[u8; 50] {
        &self.inner
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            // safe because Hash is guaranteed to be valid ASCII
            std::str::from_utf8_unchecked(&self.inner)
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            // safe because Hash is guaranteed to be valid ASCII
            String::from_utf8_unchecked(self.inner.to_vec())
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.to_vec()
    }

    pub fn hash(data: &[u8]) -> Self {
        encode_parts(hash_to_parts(data))
    }

    /// This should tell you how large a vector to allocate if you want to copy the hashed data.
    pub fn data_max_len(&self) -> Result<usize, PsHashError> {
        let bits = &self.inner[48..50];
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

    return Hash {
        inner: ps_base64::sized_encode::<50>(&vec),
    };
}

pub fn hash(data: &[u8]) -> Hash {
    encode_parts(hash_to_parts(data))
}

pub fn decode_parts(hash: &[u8]) -> Result<HashParts, PsHashError> {
    if hash.len() < 50 {
        return Err(PsHashError::InputTooShort);
    }

    let bytes = ps_base64::decode(hash);

    return Ok((
        bytes[0..32].try_into()?,
        bytes[32..36].try_into()?,
        PackedInt::from_12_bits(&bytes[36..38].try_into()?),
    ));
}

pub fn verify_hash_integrity(hash: &[u8]) -> bool {
    let parts = match decode_parts(hash) {
        Ok(parts) => parts,
        Err(_) => return false,
    };

    for i in 0..4 {
        if parts.1 == checksum(&parts.0, parts.2.to_u32() + i << 12) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use ps_pint16::PackedInt;

    #[test]
    pub fn hash() {
        let test_str = b"Hello, world!";
        let test_value = test_str.as_slice();
        let hash_value = super::hash(test_value).to_string();

        assert_eq!(
            "3Lqbann~vFOn43UpL64ukdU4TlKXU4nFFvUZCL1iFF5E1IlNDQ",
            hash_value
        );

        let parts = super::decode_parts(hash_value.as_bytes()).unwrap();

        assert_eq!(parts.2.to_usize(), test_value.len());
    }

    #[test]
    pub fn hash_length() {
        for input_length in 0..10000 {
            let input = b"F".repeat(input_length);
            let hash = super::hash(input.as_slice());
            let (_, _, length) = super::decode_parts(hash.as_bytes()).unwrap();

            assert_eq!(
                PackedInt::from_usize(input_length),
                length,
                "{}",
                input_length
            );
        }
    }
}
