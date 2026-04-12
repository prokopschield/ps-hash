use super::super::Hash;

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for Hash {}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::Hash;

    #[test]
    fn eq_same_data() {
        let h1 = Hash::hash(b"same").expect("hashing should succeed");
        let h2 = Hash::hash(b"same").expect("hashing should succeed");
        assert_eq!(h1, h2);
    }

    #[test]
    fn eq_different_data() {
        let h1 = Hash::hash(b"data1").expect("hashing should succeed");
        let h2 = Hash::hash(b"data2").expect("hashing should succeed");
        assert_ne!(h1, h2);
    }

    #[test]
    fn eq_reflexive() {
        let h = Hash::hash(b"reflexive").expect("hashing should succeed");
        assert_eq!(h, h);
    }

    #[test]
    fn eq_symmetric() {
        let h1 = Hash::hash(b"symmetric").expect("hashing should succeed");
        let h2 = Hash::hash(b"symmetric").expect("hashing should succeed");
        assert_eq!(h1, h2);
        assert_eq!(h2, h1);
    }

    #[test]
    fn eq_transitive() {
        let h1 = Hash::hash(b"transitive").expect("hashing should succeed");
        let h2 = Hash::hash(b"transitive").expect("hashing should succeed");
        let h3 = Hash::hash(b"transitive").expect("hashing should succeed");
        assert_eq!(h1, h2);
        assert_eq!(h2, h3);
        assert_eq!(h1, h3);
    }

    #[test]
    fn eq_after_validation() {
        let original = Hash::hash(b"validation").expect("hashing should succeed");
        let validated = Hash::validate(original.to_string())
            .expect("validation of an uncorrupted hash should succeed");
        assert_eq!(original, validated);
    }

    #[test]
    fn eq_after_corruption_recovery() {
        let original = Hash::hash(b"recovery").expect("hashing should succeed");
        let mut corrupted = original.to_string().into_bytes();
        corrupted[5] ^= 0x01;
        let recovered = Hash::validate(
            String::from_utf8(corrupted).expect("corrupted bytes should be valid UTF-8"),
        )
        .expect("single-bit corruption should be recovered");
        assert_eq!(original, recovered);
    }

    #[test]
    fn ne_single_bit_difference_unrecovered() {
        let h1 = Hash::hash(b"bit1").expect("hashing should succeed");
        let h2 = Hash::hash(b"bit2").expect("hashing should succeed");
        assert_ne!(h1, h2);
    }

    #[test]
    fn eq_copy_semantics() {
        let h1 = Hash::hash(b"copy").expect("hashing should succeed");
        let h2 = h1;
        assert_eq!(h1, h2);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn eq_clone_semantics() {
        let h1 = Hash::hash(b"clone").expect("hashing should succeed");
        let h2 = Clone::clone(&h1);
        assert_eq!(h1, h2);
    }

    #[test]
    fn eq_trait_bound() {
        fn assert_eq<T: Eq>() {}
        assert_eq::<Hash>();
    }

    #[test]
    fn partial_eq_trait_bound() {
        fn assert_partial_eq<T: PartialEq>() {}
        assert_partial_eq::<Hash>();
    }

    /// Tests corruption recovery at multiple byte positions to avoid brittleness
    /// from assuming a specific position is always recoverable.
    #[test]
    fn eq_after_corruption_recovery_multiple_positions() {
        let original = Hash::hash(b"multi-position recovery").expect("hashing should succeed");

        for pos in [0, 10, 20, 30, 40, 50, 60] {
            let mut corrupted = original.to_string().into_bytes();

            if pos < corrupted.len() {
                corrupted[pos] ^= 0x01;
                let recovered = Hash::validate(
                    String::from_utf8(corrupted).expect("corrupted bytes should be valid UTF-8"),
                )
                .expect("single-bit corruption should be recovered");

                assert_eq!(original, recovered, "recovery failed at position {pos}");
            }
        }
    }
}
