use ps_ecc::{RSGenerateParityError, ReedSolomon};
use ps_pint16::PackedInt;

use crate::{blake3, sha256, DIGEST_SIZE, HASH_SIZE_BIN, PARITY, PARITY_OFFSET};

const RS: ReedSolomon = match ReedSolomon::new(PARITY) {
    Ok(rs) => rs,
    Err(_) => panic!("Failed to construct Reed-Solomon codec."),
};

pub fn hash_inner(data: &[u8]) -> Result<[u8; HASH_SIZE_BIN], RSGenerateParityError> {
    let mut digest = [0u8; DIGEST_SIZE];

    let sha = sha256(data);
    let blake = blake3(data);

    for i in 0..DIGEST_SIZE {
        digest[i] = sha[i] ^ blake.as_bytes()[i];
    }

    inner_from_parts(&digest, PackedInt::from_usize(data.len()))
}

/// Assembles the internal representation from its stored parts, regenerating
/// the parity block.
pub fn inner_from_parts(
    digest: &[u8; DIGEST_SIZE],
    data_len: PackedInt,
) -> Result<[u8; HASH_SIZE_BIN], RSGenerateParityError> {
    let mut inner = [0u8; HASH_SIZE_BIN];

    inner[..DIGEST_SIZE].copy_from_slice(digest);
    inner[DIGEST_SIZE..PARITY_OFFSET].copy_from_slice(&data_len.to_16_bits());

    let parity = RS.generate_parity(&inner[..PARITY_OFFSET])?;

    inner[PARITY_OFFSET..].copy_from_slice(&parity);

    Ok(inner)
}

#[allow(clippy::expect_used)]
#[cfg(test)]
mod tests {
    use ps_pint16::PackedInt;

    use super::{hash_inner, DIGEST_SIZE, HASH_SIZE_BIN, PARITY_OFFSET};

    #[test]
    fn hash_inner_size_is_stable() {
        let inner = hash_inner(b"size").expect("hash_inner should work");
        assert_eq!(inner.len(), HASH_SIZE_BIN);
    }

    #[test]
    fn hash_inner_embeds_length() {
        let data = b"abc";
        let inner = hash_inner(data).expect("hash_inner should work");

        assert_eq!(
            &inner[DIGEST_SIZE..PARITY_OFFSET],
            &PackedInt::from_usize(data.len()).to_16_bits()
        );
    }

    #[test]
    fn hash_inner_length_field_is_byte_aligned() {
        assert_eq!(PARITY_OFFSET - DIGEST_SIZE, 2);
    }
}
