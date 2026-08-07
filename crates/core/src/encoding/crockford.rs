//! The canonical Crockford Base32 representation.

use crate::{HASH_SIZE_BIN, HASH_SIZE_CROCKFORD};

/// Encodes the internal representation as Crockford Base32.
#[inline]
#[must_use]
pub const fn encode(inner: &[u8; HASH_SIZE_BIN]) -> [u8; HASH_SIZE_CROCKFORD] {
    ps_crockford32::sized_encode(inner)
}

/// Decodes a Crockford Base32 representation.
///
/// Decoding is lenient: it is case-insensitive, it maps the ambiguous glyphs
/// `I`, `L`, `O`, and `U` onto their intended values, and it zero-fills when
/// `bytes` is shorter than a whole hash. Any other out-of-alphabet byte is
/// skipped, which shifts every subsequent digit. Truncation is left for the
/// Reed-Solomon codec to correct.
#[inline]
#[must_use]
pub const fn decode(bytes: &[u8]) -> [u8; HASH_SIZE_BIN] {
    ps_crockford32::sized_decode(bytes)
}

/// Returns the number of output bytes of [`decode`] that `bytes` fully
/// determines.
///
/// Only the symbols the lenient decoder accepts contribute bits, and a
/// trailing symbol group that fills its last byte only partially does not
/// count that byte.
#[inline]
#[must_use]
pub fn decoded_len(bytes: &[u8]) -> usize {
    let symbols = bytes
        .iter()
        .filter(|&&byte| ps_crockford32::DECODE_MAP[byte as usize] != ps_crockford32::INVALID)
        .count();

    ps_crockford32::decoded_len(symbols)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{decode, encode};
    use crate::{Hash, HASH_SIZE_BIN, HASH_SIZE_CROCKFORD};

    #[test]
    fn encode_produces_the_expected_length() {
        let encoded = encode(&[0; HASH_SIZE_BIN]);

        assert_eq!(encoded.len(), HASH_SIZE_CROCKFORD);
    }

    #[test]
    fn encode_uses_only_the_crockford_alphabet() {
        let hash = Hash::hash(b"alphabet").expect("hash should work");

        for byte in encode(&hash.inner) {
            assert!(
                b"0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(&byte),
                "invalid Crockford Base32 character: {}",
                byte as char
            );
        }
    }

    #[test]
    fn encode_excludes_ambiguous_glyphs() {
        for index in 0u32..256 {
            let hash = Hash::hash(index.to_le_bytes()).expect("hash should work");

            for byte in encode(&hash.inner) {
                assert!(
                    !matches!(byte, b'I' | b'L' | b'O' | b'U'),
                    "ambiguous glyph in output: {}",
                    byte as char
                );
            }
        }
    }

    #[test]
    fn decode_is_case_insensitive() {
        let hash = Hash::hash(b"case").expect("hash should work");
        let encoded = encode(&hash.inner);
        let lowercase = encoded.map(|byte| byte.to_ascii_lowercase());

        assert_eq!(decode(&lowercase), decode(&encoded));
    }

    #[test]
    fn encoding_is_usable_in_const_context() {
        const ENCODED: [u8; HASH_SIZE_CROCKFORD] = encode(&[0; HASH_SIZE_BIN]);

        assert_eq!(decode(&ENCODED), [0; HASH_SIZE_BIN]);
    }
}
