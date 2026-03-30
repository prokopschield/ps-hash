use sha2::{Digest, Sha256};

use crate::DIGEST_SIZE;

#[inline]
#[must_use]
pub fn sha256(data: &[u8]) -> [u8; DIGEST_SIZE] {
    let mut hasher = Sha256::new();

    hasher.update(data);
    hasher.finalize().into()
}

#[inline]
#[must_use]
pub fn blake3(data: &[u8]) -> blake3::Hash {
    blake3::hash(data)
}

#[cfg(test)]
mod tests {
    use sha2::{Digest, Sha256};

    #[test]
    fn sha256_matches_reference_implementation() {
        let data = b"sha256 test";
        let ours = super::sha256(data);
        let expected: [u8; super::DIGEST_SIZE] = Sha256::digest(data).into();

        assert_eq!(ours, expected);
    }

    #[test]
    fn blake3_matches_reference_implementation() {
        let data = b"blake3 test";
        let ours = super::blake3(data);
        let expected = blake3::hash(data);

        assert_eq!(ours.as_bytes(), expected.as_bytes());
    }
}
