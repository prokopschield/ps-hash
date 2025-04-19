use crate::{
    blake3, decode_parts, encode_parts, error::HashValidationError, sha256, Hash, HASH_SIZE, RS,
};

use super::*;

#[test]
fn test_hash_empty_data() {
    let data = [];
    let result = Hash::hash(data).unwrap();
    assert_eq!(result.as_bytes().len(), HASH_SIZE);
}

#[test]
fn test_hash_non_empty_data() {
    let data = b"some data to hash";
    let result = Hash::hash(data).unwrap();
    assert_eq!(result.as_bytes().len(), HASH_SIZE);
}

#[test]
fn test_hash_different_data() {
    let data1 = b"data one";
    let data2 = b"data two";
    let hash1 = Hash::hash(data1).unwrap();
    let hash2 = Hash::hash(data2).unwrap();
    assert_ne!(hash1, hash2);
}

#[test]
fn test_hash_deterministic() {
    let data = b"test data";
    let hash1 = Hash::hash(data).unwrap();
    let hash2 = Hash::hash(data).unwrap();
    assert_eq!(hash1, hash2);
}

#[test]
fn test_validate_valid_hash() {
    let data = b"validation data";
    let original_hash = Hash::hash(data).unwrap();
    let validated_hash = Hash::validate(original_hash).unwrap();
    assert_eq!(original_hash, validated_hash);
}

#[test]
fn test_validate_with_minor_corruption() {
    let data = b"correctable data";
    let original_hash = Hash::hash(data).unwrap();
    let mut corrupted_bytes = original_hash.as_bytes().to_vec();
    // Introduce a small corruption (e.g., flip one bit)
    let index_to_corrupt = 5;
    corrupted_bytes[index_to_corrupt] ^= 0b0000_0001;
    let corrupted_hash_str = String::from_utf8(corrupted_bytes).unwrap();
    let validated_hash = Hash::validate(&corrupted_hash_str).unwrap();
    assert_eq!(original_hash, validated_hash);
}

#[test]
fn test_validate_recoverable_corruption() {
    let data = b"unrecoverable data";
    let original_hash = Hash::hash(data).unwrap();
    let mut corrupted_bytes = original_hash.as_bytes().to_vec();
    // Introduce more significant corruption
    for item in corrupted_bytes.iter_mut().take(9) {
        *item ^= 0b0000_1111;
    }
    let corrupted_hash_str = String::from_utf8(corrupted_bytes).unwrap();
    let fixed = Hash::validate(&corrupted_hash_str).unwrap();
    assert_eq!(fixed, original_hash);
}

#[test]
fn test_validate_unrecoverable_corruption() {
    let data = b"unrecoverable data";
    let original_hash = Hash::hash(data).unwrap();
    let mut corrupted_bytes = original_hash.as_bytes().to_vec();
    // Introduce more significant corruption
    for item in corrupted_bytes.iter_mut().take(12) {
        *item ^= 0b0000_1111;
    }
    let corrupted_hash_str = String::from_utf8(corrupted_bytes).unwrap();
    let result = Hash::validate(&corrupted_hash_str);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HashValidationError::RSDecodeError(_)
    ));
}

#[test]
fn test_validate_invalid_base64() {
    let invalid_hash = "this_is_not_a_valid_base64_string";
    let result = Hash::validate(invalid_hash);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HashValidationError::RSDecodeError(ps_ecc::RSDecodeError::RSComputeErrorsError(
            ps_ecc::RSComputeErrorsError::TooManyErrors
        ))
    ));
}

#[test]
fn test_as_ref_u8_slice() {
    let data = b"as ref bytes";
    let hash = Hash::hash(data).unwrap();
    let bytes: &[u8] = hash.as_ref();
    assert_eq!(bytes, hash.as_bytes());
}

#[test]
fn test_as_ref_str() {
    let data = b"as ref string";
    let hash = Hash::hash(data).unwrap();
    let string: &str = hash.as_ref();
    assert_eq!(string, hash.as_str());
}

#[test]
fn test_display() {
    let data = b"display test";
    let hash = Hash::hash(data).unwrap();
    let displayed_hash = format!("{hash}");
    assert_eq!(displayed_hash, hash.as_str());
}

#[test]
fn test_debug() {
    let data = b"debug test";
    let hash = Hash::hash(data).unwrap();
    let debug_output = format!("{hash:?}");
    assert!(!debug_output.is_empty()); // Basic check that it produces some output
}

#[test]
fn test_hash_trait() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash as StdHash, Hasher};

    let data = b"hash trait test";
    let hash1 = Hash::hash(data).unwrap();
    let hash2 = Hash::hash(data).unwrap();

    let mut hasher1 = DefaultHasher::new();
    hash1.hash(&mut hasher1);
    let h1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    hash2.hash(&mut hasher2);
    let h2 = hasher2.finish();

    assert_eq!(h1, h2);
}

#[test]
fn test_deref() {
    let data = b"deref test";
    let hash = Hash::hash(data).unwrap();
    let slice: &str = &hash;
    assert_eq!(slice, hash.as_str());
}

#[test]
fn test_index() {
    let data = b"index test";
    let hash = Hash::hash(data).unwrap();
    assert_eq!(hash[0] as char, hash.as_str().chars().next().unwrap());
    assert_eq!(hash[HASH_SIZE + 1], 0); // Out of bounds should return 0
}

#[test]
fn test_index_range() {
    let data = b"range index test";
    let hash = Hash::hash(data).unwrap();
    assert_eq!(&hash[0..5], &hash.as_str()[0..5]);
    assert_eq!(&hash[5..], &hash.as_str()[5..]);
    assert_eq!(&hash[..5], &hash.as_str()[..5]);
    assert_eq!(&hash[..=5], &hash.as_str()[..=5]);
    assert_eq!(&hash[..], hash.as_str());
    assert_eq!(&hash[2..=7], &hash.as_str()[2..=7]);
}

#[test]
fn test_partial_eq() {
    let data_1 = b"equal test 1";
    let data_2 = b"equal test 2";
    let hash_1_a = Hash::hash(data_1).unwrap();
    let hash_1_b = Hash::hash(data_1).unwrap();
    let hash2 = Hash::hash(data_2).unwrap();
    assert_eq!(hash_1_a, hash_1_b);
    assert_ne!(hash_1_a, hash2);

    // Test equality with potentially corrupted but still decodable hashes
    let mut corrupted_bytes = hash_1_a.as_bytes().to_vec();
    corrupted_bytes[5] ^= 0b0000_0001;
    let corrupted_hash_str = String::from_utf8(corrupted_bytes).unwrap();
    let validated_corrupted = Hash::validate(&corrupted_hash_str).unwrap();
    assert_eq!(hash_1_a, validated_corrupted);
}

#[test]
fn test_ord() {
    let data1 = b"cat";
    let data2 = b"dog";
    let hash1 = Hash::hash(data1).unwrap();
    let hash2 = Hash::hash(data2).unwrap();
    assert!(hash1 < hash2);
    assert!(hash2 > hash1);
    assert_eq!(hash1.cmp(&hash1), std::cmp::Ordering::Equal);
}

#[test]
fn test_partial_ord() {
    let data1 = b"cat";
    let data2 = b"dog";
    let hash1 = Hash::hash(data1).unwrap();
    let hash2 = Hash::hash(data2).unwrap();
    assert!(hash1 < hash2);
    assert!(hash2 > hash1);
    assert_eq!(hash1.partial_cmp(&hash1), Some(std::cmp::Ordering::Equal));
}

#[test]
fn test_from_hash_to_array() {
    let data = b"from hash to array";
    let hash = Hash::hash(data).unwrap();
    let array: [u8; HASH_SIZE] = hash.into();
    assert_eq!(array, *hash.as_bytes());
}

#[test]
fn test_from_hash_ref_to_string() {
    let data = b"from hash ref to string";
    let hash = Hash::hash(data).unwrap();
    let string: String = (&hash).into();
    assert_eq!(string, hash.as_str());
}

#[test]
fn test_from_hash_ref_to_vec() {
    let data = b"from hash ref to vec";
    let hash = Hash::hash(data).unwrap();
    let vec: Vec<u8> = (&hash).into();
    assert_eq!(vec, hash.as_bytes().to_vec());
}

#[test]
fn test_try_from_slice() {
    let data = b"try from slice";
    let original_hash = Hash::hash(data).unwrap();
    let hash_from_slice = Hash::try_from(original_hash.as_bytes().as_ref()).unwrap();
    assert_eq!(original_hash, hash_from_slice);

    let invalid_slice = b"too_short";
    let result = Hash::try_from(invalid_slice.as_ref());

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HashValidationError::RSDecodeError(ps_ecc::RSDecodeError::RSComputeErrorsError(
            ps_ecc::RSComputeErrorsError::TooManyErrors
        ))
    ));

    let invalid_base64 = "invalid_base64==";
    let result = Hash::try_from(invalid_base64);

    assert_eq!(
        result,
        Err(HashValidationError::RSDecodeError(
            ps_ecc::RSDecodeError::RSComputeErrorsError(
                ps_ecc::RSComputeErrorsError::TooManyErrors
            )
        ))
    );
}

#[test]
fn test_try_from_str() {
    let data = b"try from str";
    let original_hash = Hash::hash(data).unwrap();
    let hash_from_str = Hash::try_from(original_hash.as_str()).unwrap();
    assert_eq!(original_hash, hash_from_str);

    let invalid_str = "not a valid hash string";
    let result = Hash::try_from(invalid_str);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HashValidationError::RSDecodeError(ps_ecc::RSDecodeError::RSComputeErrorsError(
            ps_ecc::RSComputeErrorsError::TooManyErrors
        ))
    ));
}

#[test]
fn test_as_bytes_const() {
    const TEST_DATA: &[u8] = b"as bytes const test";
    let hash = Hash::hash(TEST_DATA).unwrap();
    let bytes: &[u8; HASH_SIZE] = hash.as_bytes();
    assert_eq!(bytes.len(), HASH_SIZE);
}

#[test]
fn test_as_str_const() {
    const TEST_DATA: &[u8] = b"as str const test";
    let hash = Hash::hash(TEST_DATA).unwrap();
    let str: &str = hash.as_str();
    assert_eq!(str.len(), HASH_SIZE);
}

#[test]
fn test_to_vec() {
    let data = b"to vec test";
    let hash = Hash::hash(data).unwrap();
    let vec = hash.to_vec();
    assert_eq!(vec, hash.as_bytes().to_vec());
}

#[test]
fn test_data_max_len() {
    let data = b"data max len test";
    let hash = Hash::hash(data).unwrap();
    let decoded_len = hash.data_max_len().unwrap();
    assert_eq!(decoded_len, data.len());

    // Test with a different length
    let long_data = vec![0u8; 150];
    let long_hash = Hash::hash(&long_data).unwrap();
    let long_decoded_len = long_hash.data_max_len().unwrap();
    assert_eq!(long_decoded_len, long_data.len());
}

#[test]
fn test_encode_decode_parts() {
    let original_data = b"test encode decode parts";
    let sha_hash = sha256(original_data);
    let blake3_hash = blake3(original_data).as_bytes().to_vec();
    let xored: Vec<u8> = sha_hash
        .iter()
        .zip(blake3_hash.iter())
        .map(|(&s, &b)| s ^ b)
        .collect();
    let length = PackedInt::from_usize(original_data.len());
    let mut buffer = Buffer::with_capacity(34).unwrap();
    buffer.extend_from_slice(&xored).unwrap();
    buffer.extend_from_slice(length.to_16_bits()).unwrap();
    let parity = RS.generate_parity(&buffer).unwrap();

    let original_parts = (
        xored.try_into().unwrap(),
        parity.slice(..).try_into().unwrap(),
        length,
    );

    let encoded_hash = encode_parts(original_parts);
    let decoded_parts = decode_parts(encoded_hash.as_bytes()).unwrap();

    assert_eq!(Hash::validate(encoded_hash).unwrap(), encoded_hash);

    assert_eq!(original_parts.0, decoded_parts.0);
    assert_eq!(original_parts.1, decoded_parts.1);
    assert_eq!(original_parts.2, decoded_parts.2);

    // Re-encode the decoded parts and validate it recovers the original (modulo corruption)
    let re_encoded_hash = encode_parts((decoded_parts.0, decoded_parts.1, decoded_parts.2));
    let validated_re_encoded = Hash::validate(re_encoded_hash).unwrap();
    let validated_original = Hash::validate(encoded_hash).unwrap();
    assert_eq!(validated_original, validated_re_encoded);
}

#[test]
fn test_inline_hash_function() {
    let data = b"inline hash test";
    let hash1 = Hash::hash(data).unwrap();
    let hash2 = crate::hash(data).unwrap();
    assert_eq!(hash1, hash2);
}
