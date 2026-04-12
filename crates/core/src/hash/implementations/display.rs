use std::fmt::{Debug, Display};

use ps_base64::base64;

use super::super::Hash;

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        base64::encode_into(&self.inner, f)
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Hash, HASH_SIZE};

    #[test]
    fn display_correct_length() {
        let h = Hash::hash(b"test").unwrap();
        let s = format!("{h}");
        assert_eq!(s.len(), HASH_SIZE);
    }

    #[test]
    fn display_matches_to_string() {
        let h = Hash::hash(b"matches").unwrap();
        let displayed = format!("{h}");
        let to_string = h.to_string();
        assert_eq!(displayed, to_string);
    }

    #[test]
    fn display_is_deterministic() {
        let h = Hash::hash(b"deterministic").unwrap();
        let s1 = format!("{h}");
        let s2 = format!("{h}");
        assert_eq!(s1, s2);
    }

    #[test]
    fn display_different_for_different_data() {
        let h1 = Hash::hash(b"data1").unwrap();
        let h2 = Hash::hash(b"data2").unwrap();
        let s1 = format!("{h1}");
        let s2 = format!("{h2}");
        assert_ne!(s1, s2);
    }

    #[test]
    fn debug_correct_length() {
        let h = Hash::hash(b"debug").unwrap();
        let s = format!("{h:?}");
        assert_eq!(s.len(), HASH_SIZE);
    }

    #[test]
    fn debug_matches_display() {
        let h = Hash::hash(b"debug display").unwrap();
        let debug = format!("{h:?}");
        let display = format!("{h}");
        assert_eq!(debug, display);
    }

    #[test]
    fn debug_matches_to_string() {
        let h = Hash::hash(b"debug to_string").unwrap();
        let debug = format!("{h:?}");
        let to_string = h.to_string();
        assert_eq!(debug, to_string);
    }

    #[test]
    fn debug_is_deterministic() {
        let h = Hash::hash(b"debug deterministic").unwrap();
        let s1 = format!("{h:?}");
        let s2 = format!("{h:?}");
        assert_eq!(s1, s2);
    }

    #[test]
    fn display_round_trips() {
        let original = Hash::hash(b"round trip").unwrap();
        let displayed = format!("{original}");
        let recovered = Hash::validate(&displayed).unwrap();
        assert_eq!(original, recovered);
    }
}
