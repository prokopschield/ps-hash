/// Size of the digest, in bytes.
pub const DIGEST_SIZE: usize = 32;

/// Size of the packed data-length field, in bytes.
pub const SIZE_SIZE: usize = std::mem::size_of::<u16>();

/// Number of byte errors the Reed-Solomon codec can correct.
pub const PARITY: u8 = 7;

/// Size of the Reed-Solomon parity block, in bytes.
pub const PARITY_SIZE: usize = 2 * PARITY as usize;

/// Offset of the parity block within the internal representation.
pub const PARITY_OFFSET: usize = DIGEST_SIZE + SIZE_SIZE;

/// Size of the internal representation, in bytes.
///
/// The layout is byte-aligned throughout:
///
/// | offset | size | content                                    |
/// |--------|------|--------------------------------------------|
/// | 0..32  | 32 B | digest                                     |
/// | 32..34 | 2 B  | data length, as a [`ps_pint16::PackedInt`] |
/// | 34..48 | 14 B | Reed-Solomon parity                        |
pub const HASH_SIZE_BIN: usize = PARITY_OFFSET + PARITY_SIZE;

/// Size of the truncated binary representation, in bytes.
///
/// The omitted trailing parity bytes are restored by [`Hash::validate`],
/// which spends all but one byte of the correction budget: a compact hash
/// still tolerates one corrupted byte.
///
/// [`Hash::validate`]: crate::Hash::validate
pub const HASH_SIZE_COMPACT: usize = MIN_RECOVERABLE_BIN + 1;

/// Size of the Crockford Base32 representation, in characters.
pub const HASH_SIZE_CROCKFORD: usize = encoded_size(CROCKFORD_BITS);

/// Size of the base64url representation, in characters.
pub const HASH_SIZE_BASE64: usize = encoded_size(BASE64_BITS);

/// Shortest binary input [`Hash::validate`] accepts.
///
/// [`Hash::validate`]: crate::Hash::validate
pub const MIN_RECOVERABLE_BIN: usize = HASH_SIZE_BIN - PARITY as usize;

/// Shortest Crockford Base32 input [`Hash::validate`] accepts.
///
/// [`Hash::validate`]: crate::Hash::validate
pub const MIN_RECOVERABLE_CROCKFORD: usize =
    HASH_SIZE_CROCKFORD - recoverable_truncation(CROCKFORD_BITS);

/// Shortest base64url input [`Hash::validate`] accepts.
///
/// [`Hash::validate`]: crate::Hash::validate
pub const MIN_RECOVERABLE_BASE64: usize = HASH_SIZE_BASE64 - recoverable_truncation(BASE64_BITS);

/// Bits carried by one Crockford Base32 character.
const CROCKFORD_BITS: usize = 5;

/// Bits carried by one base64url character.
const BASE64_BITS: usize = 6;

/// Number of characters needed to carry [`HASH_SIZE_BIN`] bytes.
const fn encoded_size(bits_per_char: usize) -> usize {
    (HASH_SIZE_BIN * 8).div_ceil(bits_per_char)
}

/// Number of trailing characters that may be dropped while the resulting loss
/// stays within the [`PARITY`]-byte correction budget.
const fn recoverable_truncation(bits_per_char: usize) -> usize {
    PARITY as usize * 8 / bits_per_char
}

/// [`Hash::validate`] dispatches on input length alone, so the ranges accepted
/// for the three representations must not overlap.
///
/// [`Hash::validate`]: crate::Hash::validate
const _: () = {
    assert!(MIN_RECOVERABLE_BIN <= HASH_SIZE_BIN);
    assert!(HASH_SIZE_BIN < MIN_RECOVERABLE_BASE64);

    assert!(MIN_RECOVERABLE_BASE64 <= HASH_SIZE_BASE64);
    assert!(HASH_SIZE_BASE64 < MIN_RECOVERABLE_CROCKFORD);

    assert!(MIN_RECOVERABLE_CROCKFORD <= HASH_SIZE_CROCKFORD);
};

/// The compact representation is accepted through the binary range, so it must
/// itself be recoverable.
const _: () = {
    assert!(HASH_SIZE_COMPACT >= MIN_RECOVERABLE_BIN);
    assert!(HASH_SIZE_COMPACT <= HASH_SIZE_BIN);
};

/// Both encodings must carry the whole internal representation.
const _: () = {
    assert!(HASH_SIZE_CROCKFORD * CROCKFORD_BITS >= HASH_SIZE_BIN * 8);
    assert!(HASH_SIZE_BASE64 * BASE64_BITS >= HASH_SIZE_BIN * 8);
};

#[cfg(test)]
mod tests {
    use super::{
        DIGEST_SIZE, HASH_SIZE_BASE64, HASH_SIZE_BIN, HASH_SIZE_COMPACT, HASH_SIZE_CROCKFORD,
        MIN_RECOVERABLE_BASE64, MIN_RECOVERABLE_BIN, MIN_RECOVERABLE_CROCKFORD, PARITY,
        PARITY_OFFSET, PARITY_SIZE, SIZE_SIZE,
    };

    /// Pins every derived constant, so that a change to [`PARITY`] cannot
    /// silently alter the wire format.
    #[test]
    fn constants_are_consistent() {
        assert_eq!(DIGEST_SIZE, 32);
        assert_eq!(SIZE_SIZE, 2);
        assert_eq!(PARITY, 7);
        assert_eq!(PARITY_SIZE, 14);
        assert_eq!(PARITY_OFFSET, 34);
        assert_eq!(HASH_SIZE_BIN, 48);
        assert_eq!(HASH_SIZE_COMPACT, 42);
        assert_eq!(HASH_SIZE_CROCKFORD, 77);
        assert_eq!(HASH_SIZE_BASE64, 64);
        assert_eq!(MIN_RECOVERABLE_BIN, 41);
        assert_eq!(MIN_RECOVERABLE_CROCKFORD, 66);
        assert_eq!(MIN_RECOVERABLE_BASE64, 55);
    }
}
