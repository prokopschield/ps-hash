//! The compact base64url representation.

use crate::{HASH_SIZE_BASE64, HASH_SIZE_BIN};

/// Encodes the internal representation as unpadded base64url.
#[inline]
#[must_use]
pub fn encode(inner: &[u8; HASH_SIZE_BIN]) -> [u8; HASH_SIZE_BASE64] {
    ps_base64::sized_encode(inner)
}

/// Decodes a base64url representation.
///
/// Decoding is lenient: it accepts both the URL-safe (`-`, `_`) and standard
/// (`+`, `/`) alphabets, and it zero-fills when `bytes` is shorter than a
/// whole hash. Truncation and stray characters are left for the Reed-Solomon
/// codec to correct.
#[inline]
#[must_use]
pub fn decode(bytes: &[u8]) -> [u8; HASH_SIZE_BIN] {
    ps_base64::sized_decode(bytes)
}

/// Returns the number of output bytes of [`decode`] that `bytes` fully
/// determines.
///
/// Only the characters the lenient decoder accepts contribute bits (ASCII
/// whitespace and `=` are skipped), and a trailing character group that fills
/// its last byte only partially does not count that byte.
#[inline]
#[must_use]
pub fn decoded_len(bytes: &[u8]) -> usize {
    let symbols = bytes
        .iter()
        .filter(|&&byte| !byte.is_ascii_whitespace() && byte != b'=')
        .count();

    symbols / 4 * 3 + symbols % 4 * 6 / 8
}

#[allow(clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::{decode, decoded_len, encode};
    use crate::{Hash, HASH_SIZE_BASE64, HASH_SIZE_BIN};

    #[test]
    fn encode_produces_the_expected_length() {
        let encoded = encode(&[0; HASH_SIZE_BIN]);

        assert_eq!(encoded.len(), HASH_SIZE_BASE64);
    }

    #[test]
    fn encode_uses_the_url_safe_alphabet() {
        for index in 0u32..256 {
            let hash = Hash::hash(index.to_le_bytes()).expect("hash should work");

            for byte in encode(&hash.inner) {
                assert!(
                    byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_',
                    "invalid base64url character: {}",
                    byte as char
                );
            }
        }
    }

    #[test]
    fn encode_emits_no_padding() {
        let hash = Hash::hash(b"padding").expect("hash should work");

        assert!(!encode(&hash.inner).contains(&b'='));
    }

    #[test]
    fn decode_ignores_trailing_padding() {
        let hash = Hash::hash(b"trailing").expect("hash should work");
        let encoded = encode(&hash.inner);

        let mut padded = encoded.to_vec();
        padded.extend_from_slice(b"==");

        assert_eq!(decode(&padded), decode(&encoded));

        // A full-length input already determines every output byte, so the
        // padding must also be ignored after a truncated input, where any
        // contributed bits would change the output.
        let truncated = &encoded[..60];
        let mut truncated_padded = truncated.to_vec();
        truncated_padded.extend_from_slice(b"==");

        assert_eq!(decode(&truncated_padded), decode(truncated));
        assert_eq!(decoded_len(&truncated_padded), decoded_len(truncated));
    }
}
