//! The two textual representations of a [`Hash`](crate::Hash).
//!
//! Both encode the same [`HASH_SIZE_BIN`](crate::HASH_SIZE_BIN)-byte internal
//! representation and differ only in alphabet:
//!
//! - [`crockford`] is the canonical form: case-insensitive, and free of the
//!   ambiguous glyphs `I`, `L`, `O`, and `U`.
//! - [`base64`] is the compact form, using the URL-safe alphabet.
//!
//! Their encoded lengths differ, which is what lets
//! [`Hash::validate`](crate::Hash::validate) tell them apart by length alone.

pub mod base64;
pub mod crockford;

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{base64, crockford};
    use crate::{hash_inner, HASH_SIZE_BASE64, HASH_SIZE_BIN, HASH_SIZE_CROCKFORD};

    #[test]
    fn crockford_round_trips() {
        let inner = hash_inner(b"crockford round trip").expect("hash_inner should work");

        let encoded = crockford::encode(&inner);

        assert_eq!(encoded.len(), HASH_SIZE_CROCKFORD);
        assert_eq!(crockford::decode(&encoded), inner);
    }

    #[test]
    fn base64_round_trips() {
        let inner = hash_inner(b"base64 round trip").expect("hash_inner should work");

        let encoded = base64::encode(&inner);

        assert_eq!(encoded.len(), HASH_SIZE_BASE64);
        assert_eq!(base64::decode(&encoded), inner);
    }

    #[test]
    fn representations_differ_in_length() {
        assert_ne!(HASH_SIZE_CROCKFORD, HASH_SIZE_BASE64);
    }

    #[test]
    fn round_trips_over_the_whole_byte_range() {
        for byte in 0..=u8::MAX {
            let inner = [byte; HASH_SIZE_BIN];

            assert_eq!(crockford::decode(&crockford::encode(&inner)), inner);
            assert_eq!(base64::decode(&base64::encode(&inner)), inner);
        }
    }
}
