use sha2::{Digest, Sha256};

pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    return result.into();
}

pub fn blake3(data: &[u8]) -> [u8; 32] {
    return *blake3::hash(data).as_bytes();
}

pub fn xor(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let mut result = [0; 32];

    for i in 0..32 {
        result[i] = a[i] ^ b[i];
    }

    return result;
}

pub fn checksum_u32(data: &[u8], length: u32) -> u32 {
    let mut hash: u32 = length;

    for c in data.iter() {
        hash = (*c as u32)
            .wrapping_add(hash << 6)
            .wrapping_add(hash << 16)
            .wrapping_sub(hash);
    }

    return hash;
}

pub fn checksum(data: &[u8], length: u32) -> [u8; 4] {
    checksum_u32(data, length).to_le_bytes()
}

pub type HashParts = ([u8; 32], [u8; 4], u16);

pub fn hash_to_parts(data: &[u8]) -> HashParts {
    let length: u16 = data.len() as u16;
    let shasum = sha256(data);
    let blasum = blake3(data);
    let xored = xor(shasum, blasum);
    let checksum = checksum(&xored, length as u32);

    return (xored, checksum, length);
}

pub fn encode_parts(parts: HashParts) -> String {
    let (xored, checksum, length) = parts;

    let mut vec: Vec<u8> = Vec::with_capacity(38);

    vec.extend_from_slice(&xored);
    vec.extend_from_slice(&checksum);
    vec.push(length as u8);
    vec.push((length >> 4) as u8);

    let mut encoded = ps_base64::encode(&vec);

    encoded.truncate(50);

    return encoded;
}

pub fn hash(data: &[u8]) -> String {
    encode_parts(hash_to_parts(data))
}

pub fn decode_parts(hash: &[u8]) -> Result<HashParts, ()> {
    if hash.len() < 50 {
        return Err(());
    }

    let bytes = ps_base64::decode(hash);

    return Ok((
        bytes[0..32].try_into().map_err(|_| ())?,
        bytes[32..36].try_into().map_err(|_| ())?,
        bytes[36] as u16 + ((bytes[37] as u16) << 4),
    ));
}

pub fn verify_hash_integrity(hash: &[u8]) -> bool {
    let parts = match decode_parts(hash) {
        Ok(parts) => parts,
        Err(_) => return false,
    };

    for i in 0..4 {
        if parts.1 == checksum(&parts.0, parts.2 as u32 + i << 12) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn hash() {
        let test_str = b"Hello, world!";
        let test_value = test_str.as_slice();
        let hash_value = super::hash(test_value);

        assert_eq!(
            "3Lqbann~vFOn43UpL64ukdU4TlKXU4nFFvUZCL1iFF5E1IlNDQ",
            hash_value
        );

        let parts = super::decode_parts(hash_value.as_bytes()).unwrap();

        assert_eq!(parts.2, test_value.len() as u16);
    }

    #[test]
    pub fn hash_length() {
        for input_length in 0..10000 {
            let input = b"F".repeat(input_length);
            let hash = super::hash(input.as_slice());
            let (_, _, length) = super::decode_parts(hash.as_bytes()).unwrap();

            assert_eq!(input_length % 4096, length as usize, "{}", input_length);
        }
    }
}
