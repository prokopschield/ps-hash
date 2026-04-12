use ps_base64::encode;

use super::super::Hash;

impl Hash {
    #[must_use]
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        encode(&self.inner)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Hash, HASH_SIZE};

    #[test]
    fn to_string_returns_correct_length() {
        let h = Hash::hash(b"test").unwrap();
        assert_eq!(h.to_string().len(), HASH_SIZE);
    }

    #[test]
    fn to_string_returns_64_chars() {
        let h = Hash::hash(b"64 chars").unwrap();
        assert_eq!(h.to_string().len(), 64);
    }

    #[test]
    fn to_string_is_deterministic() {
        let h = Hash::hash(b"deterministic").unwrap();
        let s1 = h.to_string();
        let s2 = h.to_string();
        assert_eq!(s1, s2);
    }

    #[test]
    fn to_string_different_for_different_data() {
        let h1 = Hash::hash(b"data1").unwrap();
        let h2 = Hash::hash(b"data2").unwrap();
        assert_ne!(h1.to_string(), h2.to_string());
    }

    #[test]
    fn to_string_same_for_same_data() {
        let h1 = Hash::hash(b"same").unwrap();
        let h2 = Hash::hash(b"same").unwrap();
        assert_eq!(h1.to_string(), h2.to_string());
    }

    #[test]
    fn to_string_is_valid_base64() {
        let h = Hash::hash(b"base64").unwrap();
        let s = h.to_string();
        for c in s.chars() {
            assert!(
                c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '-' || c == '_',
                "Invalid base64 character: {c}"
            );
        }
    }

    #[test]
    fn to_string_round_trips_through_validate() {
        let original = Hash::hash(b"round trip").unwrap();
        let s = original.to_string();
        let recovered = Hash::validate(&s).unwrap();
        assert_eq!(original, recovered);
    }

    #[test]
    fn to_string_matches_display() {
        let h = Hash::hash(b"display").unwrap();
        assert_eq!(h.to_string(), format!("{h}"));
    }

    #[test]
    fn to_string_matches_into_string() {
        let h = Hash::hash(b"into").unwrap();
        let s1 = h.to_string();
        let s2: String = h.into();
        assert_eq!(s1, s2);
    }

    #[test]
    fn to_string_is_utf8() {
        let h = Hash::hash(b"utf8").unwrap();
        let s = h.to_string();
        assert!(std::str::from_utf8(s.as_bytes()).is_ok());
    }

    /// Verifies that ps-base64 uses URL-safe alphabet (- and _) rather than
    /// standard base64 (+ and /). This documents the actual encoding used.
    #[test]
    fn to_string_uses_url_safe_base64_alphabet() {
        // Generate many hashes to have high confidence we'd see all alphabet chars
        let mut saw_standard = false;
        let mut saw_url_safe = false;

        for i in 0u32..1000 {
            let h = Hash::hash(&i.to_le_bytes()).unwrap();
            let s = h.to_string();

            for c in s.chars() {
                if c == '+' || c == '/' {
                    saw_standard = true;
                }
                if c == '-' || c == '_' {
                    saw_url_safe = true;
                }
            }
        }

        assert!(
            saw_url_safe,
            "expected to see URL-safe base64 characters (- or _)"
        );
        assert!(
            !saw_standard,
            "standard base64 characters (+ or /) should not appear in URL-safe encoding"
        );
    }
}
