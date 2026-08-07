use super::super::Hash;

impl Hash {
    /// Returns the canonical Crockford Base32 representation.
    #[must_use]
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.to_crockford()
    }

    /// Returns the Crockford Base32 representation.
    ///
    /// The alphabet is case-insensitive on input and excludes the ambiguous
    /// glyphs `I`, `L`, `O`, and `U`, which makes it the better choice for
    /// hashes that are read, spoken, or typed by hand.
    #[must_use]
    pub fn to_crockford(&self) -> String {
        ps_crockford32::encode(&self.inner)
    }

    /// Returns the unpadded base64url representation.
    ///
    /// This is the shorter of the two representations, and the better choice
    /// where a hash is only ever handled by machines.
    #[must_use]
    pub fn to_base64(&self) -> String {
        ps_base64::encode(&self.inner)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::{
        encoding::{base64, crockford},
        Hash, HASH_SIZE_BASE64, HASH_SIZE_CROCKFORD,
    };

    #[test]
    fn to_string_returns_correct_length() {
        let h = Hash::hash(b"test").expect("hashing should succeed");

        assert_eq!(h.to_string().len(), HASH_SIZE_CROCKFORD);
    }

    #[test]
    fn to_string_is_the_crockford_representation() {
        let h = Hash::hash(b"canonical").expect("hashing should succeed");

        assert_eq!(h.to_string(), h.to_crockford());
    }

    #[test]
    fn representations_have_their_expected_lengths() {
        let h = Hash::hash(b"lengths").expect("hashing should succeed");

        assert_eq!(h.to_crockford().len(), 77);
        assert_eq!(h.to_base64().len(), 64);
    }

    #[test]
    fn representations_agree_with_the_encoding_module() {
        let h = Hash::hash(b"agreement").expect("hashing should succeed");

        assert_eq!(h.to_crockford().as_bytes(), crockford::encode(&h.inner));
        assert_eq!(h.to_base64().as_bytes(), base64::encode(&h.inner));
    }

    #[test]
    fn representations_differ_from_each_other() {
        let h = Hash::hash(b"distinct").expect("hashing should succeed");

        assert_ne!(h.to_crockford(), h.to_base64());
    }

    #[test]
    fn representations_are_deterministic() {
        let h = Hash::hash(b"deterministic").expect("hashing should succeed");

        assert_eq!(h.to_crockford(), h.to_crockford());
        assert_eq!(h.to_base64(), h.to_base64());
    }

    #[test]
    fn representations_differ_for_different_data() {
        let h1 = Hash::hash(b"data1").expect("hashing should succeed");
        let h2 = Hash::hash(b"data2").expect("hashing should succeed");

        assert_ne!(h1.to_crockford(), h2.to_crockford());
        assert_ne!(h1.to_base64(), h2.to_base64());
    }

    #[test]
    fn representations_agree_for_the_same_data() {
        let h1 = Hash::hash(b"same").expect("hashing should succeed");
        let h2 = Hash::hash(b"same").expect("hashing should succeed");

        assert_eq!(h1.to_crockford(), h2.to_crockford());
        assert_eq!(h1.to_base64(), h2.to_base64());
    }

    #[test]
    fn both_representations_round_trip_through_validate() {
        let original = Hash::hash(b"round trip").expect("hashing should succeed");

        assert_eq!(
            Hash::validate(original.to_crockford())
                .expect("round trip through validate should succeed"),
            original
        );
        assert_eq!(
            Hash::validate(original.to_base64())
                .expect("round trip through validate should succeed"),
            original
        );
    }

    #[test]
    fn to_string_matches_display() {
        let h = Hash::hash(b"display").expect("hashing should succeed");

        assert_eq!(h.to_string(), format!("{h}"));
    }

    #[test]
    fn to_string_matches_into_string() {
        let h = Hash::hash(b"into").expect("hashing should succeed");
        let s: String = h.into();

        assert_eq!(h.to_string(), s);
    }

    #[test]
    fn crockford_uses_only_its_alphabet() {
        let h = Hash::hash(b"crockford").expect("hashing should succeed");

        for c in h.to_crockford().chars() {
            assert!(
                b"0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(&(c as u8)),
                "invalid Crockford Base32 character: {c}"
            );
        }
    }

    #[test]
    fn base64_uses_only_the_url_safe_alphabet() {
        let h = Hash::hash(b"base64").expect("hashing should succeed");

        for c in h.to_base64().chars() {
            assert!(
                c.is_ascii_alphanumeric() || c == '-' || c == '_',
                "invalid base64url character: {c}"
            );
        }
    }

    #[test]
    fn crockford_excludes_ambiguous_glyphs() {
        for i in 0u32..1000 {
            let h = Hash::hash(i.to_le_bytes()).expect("hashing should succeed");

            for c in h.to_crockford().chars() {
                assert!(
                    !matches!(c, 'I' | 'L' | 'O' | 'U' | 'i' | 'l' | 'o' | 'u'),
                    "Crockford output should not contain ambiguous glyph {c}"
                );
            }
        }
    }

    #[test]
    fn both_representations_are_ascii() {
        let h = Hash::hash(b"ascii").expect("hashing should succeed");

        assert!(h.to_crockford().is_ascii());
        assert!(h.to_base64().is_ascii());
    }

    #[test]
    fn representations_never_collide_in_length() {
        assert_ne!(HASH_SIZE_CROCKFORD, HASH_SIZE_BASE64);
    }
}
