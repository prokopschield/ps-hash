use ps_ecc::{RSGenerateParityError, ReedSolomon};
use ps_pint16::PackedInt;

use crate::{blake3, sha256, DIGEST_SIZE, HASH_SIZE_BIN, PARITY, PARITY_OFFSET};

const RS: ReedSolomon = match ReedSolomon::new(PARITY) {
    Ok(rs) => rs,
    Err(_) => panic!("Failed to construct Reed-Solomon codec."),
};

pub fn hash_inner(data: &[u8]) -> Result<[u8; HASH_SIZE_BIN], RSGenerateParityError> {
    let mut inner = [0u8; HASH_SIZE_BIN];

    let sha = sha256(data);
    let blake = blake3(data);

    for i in 0..DIGEST_SIZE {
        inner[i] = sha[i] ^ blake.as_bytes()[i];
    }

    inner[DIGEST_SIZE..PARITY_OFFSET]
        .copy_from_slice(&PackedInt::from_usize(data.len()).to_16_bits());

    let parity = RS.generate_parity(&inner[..PARITY_OFFSET])?;

    inner[PARITY_OFFSET..].copy_from_slice(&parity);

    Ok(inner)
}

#[cfg(test)]
mod tests {
    use ps_base64::base64;
    use ps_pint16::PackedInt;

    use super::{hash_inner, DIGEST_SIZE, HASH_SIZE_BIN, PARITY_OFFSET};

    #[test]
    fn hash_inner_matches_known_vector() {
        let inner = hash_inner(b"Hello, world!").expect("hash_inner should work");
        let encoded: [u8; crate::HASH_SIZE] = base64::sized_encode(&inner);

        assert_eq!(
            String::from_utf8_lossy(&encoded),
            "3Lqbann-vFOn43UpL64ukdU4TlKXU4nFFvUZCL1iFF4NAFBGLtfcLLDPwF92CquL"
        );
    }

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
}
