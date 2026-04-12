use ps_util::subarray;

use crate::{PARITY_OFFSET, PARITY_SIZE};

use super::super::Hash;

impl Hash {
    #[must_use]
    pub fn parity(&self) -> &[u8; PARITY_SIZE] {
        subarray(&self.inner, PARITY_OFFSET)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Hash, HASH_SIZE_BIN, PARITY_OFFSET, PARITY_SIZE};

    #[test]
    fn parity_returns_correct_size() {
        let h = Hash::hash(b"test").unwrap();
        assert_eq!(h.parity().len(), PARITY_SIZE);
    }

    #[test]
    fn parity_is_14_bytes() {
        let h = Hash::hash(b"14 bytes").unwrap();
        assert_eq!(h.parity().len(), 14);
    }

    #[test]
    fn parity_is_suffix_of_inner() {
        let h = Hash::hash(b"suffix").unwrap();
        let parity = h.parity();
        assert_eq!(parity.as_slice(), &h.inner[PARITY_OFFSET..]);
    }

    #[test]
    fn parity_offset_plus_size_equals_inner_size() {
        assert_eq!(PARITY_OFFSET + PARITY_SIZE, HASH_SIZE_BIN);
    }

    #[test]
    fn parity_is_deterministic() {
        let h = Hash::hash(b"deterministic").unwrap();
        let p1 = h.parity();
        let p2 = h.parity();
        assert_eq!(p1, p2);
    }

    #[test]
    fn parity_different_for_different_data() {
        let h1 = Hash::hash(b"data1").unwrap();
        let h2 = Hash::hash(b"data2").unwrap();
        assert_ne!(h1.parity(), h2.parity());
    }

    #[test]
    fn parity_same_for_same_data() {
        let h1 = Hash::hash(b"same").unwrap();
        let h2 = Hash::hash(b"same").unwrap();
        assert_eq!(h1.parity(), h2.parity());
    }

    #[test]
    fn parity_returns_array_reference() {
        let h = Hash::hash(b"array ref").unwrap();
        let parity: &[u8; PARITY_SIZE] = h.parity();
        assert_eq!(parity.len(), PARITY_SIZE);
    }

    #[test]
    fn parity_enables_error_correction() {
        let original = Hash::hash(b"error correction").unwrap();
        let mut corrupted = original.inner;
        corrupted[0] ^= 0xFF;
        let recovered = Hash::validate(&corrupted).unwrap();
        assert_eq!(original, recovered);
    }

    #[test]
    fn parity_preserved_after_validation() {
        let original = Hash::hash(b"preserved").unwrap();
        let validated = Hash::validate(original.to_string()).unwrap();
        assert_eq!(original.parity(), validated.parity());
    }
}
