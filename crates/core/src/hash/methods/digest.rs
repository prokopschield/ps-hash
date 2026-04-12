use ps_util::subarray;

use crate::DIGEST_SIZE;

use super::super::Hash;

impl Hash {
    #[must_use]
    pub fn digest(&self) -> &[u8; DIGEST_SIZE] {
        subarray(&self.inner, 0)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::{Hash, DIGEST_SIZE};

    #[test]
    fn digest_returns_correct_size() {
        let h = Hash::hash(b"test").expect("hashing should succeed");
        assert_eq!(h.digest().len(), DIGEST_SIZE);
    }

    #[test]
    fn digest_is_32_bytes() {
        let h = Hash::hash(b"32 bytes").expect("hashing should succeed");
        assert_eq!(h.digest().len(), 32);
    }

    #[test]
    fn digest_is_prefix_of_inner() {
        let h = Hash::hash(b"prefix").expect("hashing should succeed");
        let digest = h.digest();
        assert_eq!(digest.as_slice(), &h.inner[..DIGEST_SIZE]);
    }

    #[test]
    fn digest_is_deterministic() {
        let h = Hash::hash(b"deterministic").expect("hashing should succeed");
        let d1 = h.digest();
        let d2 = h.digest();
        assert_eq!(d1, d2);
    }

    #[test]
    fn digest_different_for_different_data() {
        let h1 = Hash::hash(b"data1").expect("hashing should succeed");
        let h2 = Hash::hash(b"data2").expect("hashing should succeed");
        assert_ne!(h1.digest(), h2.digest());
    }

    #[test]
    fn digest_same_for_same_data() {
        let h1 = Hash::hash(b"same").expect("hashing should succeed");
        let h2 = Hash::hash(b"same").expect("hashing should succeed");
        assert_eq!(h1.digest(), h2.digest());
    }

    #[test]
    fn digest_changes_with_single_bit_difference() {
        let h1 = Hash::hash([0u8]).expect("hashing should succeed");
        let h2 = Hash::hash([1u8]).expect("hashing should succeed");
        assert_ne!(h1.digest(), h2.digest());
    }

    #[test]
    fn digest_returns_array_reference() {
        let h = Hash::hash(b"array ref").expect("hashing should succeed");
        let digest: &[u8; DIGEST_SIZE] = h.digest();
        assert_eq!(digest.len(), DIGEST_SIZE);
    }

    #[test]
    fn digest_preserved_after_validation() {
        let original = Hash::hash(b"preserved").expect("hashing should succeed");
        let validated = Hash::validate(original.to_string())
            .expect("validation of an uncorrupted hash should succeed");
        assert_eq!(original.digest(), validated.digest());
    }
}
