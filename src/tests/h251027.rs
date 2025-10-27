use crate::{Hash, HASH_SIZE, HASH_SIZE_BIN, HASH_SIZE_COMPACT, PARITY_SIZE};

// ============================================================================
// Basic Hash Creation and Validation
// ============================================================================

#[test]
fn test_hash_creation_empty_data() {
    let result = Hash::hash(&[]);
    assert!(result.is_ok());
    let hash = result.unwrap();
    assert_eq!(hash.to_string().len(), HASH_SIZE);
}

#[test]
fn test_hash_creation_non_empty_data() {
    let data = b"test data";
    let result = Hash::hash(data);
    assert!(result.is_ok());
    let hash = result.unwrap();
    assert_eq!(hash.to_string().len(), HASH_SIZE);
}

#[test]
fn test_hash_creation_large_data() {
    let data = vec![0u8; 1_000_000];
    let result = Hash::hash(&data);
    assert!(result.is_ok());
}

#[test]
fn test_hash_deterministic() {
    let data = b"deterministic test";
    let hash1 = Hash::hash(data).unwrap();
    let hash2 = Hash::hash(data).unwrap();
    assert_eq!(hash1, hash2);
}

#[test]
fn test_hash_different_inputs_different_outputs() {
    let hash1 = Hash::hash(b"data1").unwrap();
    let hash2 = Hash::hash(b"data2").unwrap();
    assert_ne!(hash1, hash2);
}

// ============================================================================
// Hash Validation
// ============================================================================

#[test]
fn test_validate_uncorrupted_hash() {
    let original = Hash::hash(b"validation test").unwrap();
    let validated = Hash::validate(original.to_string()).unwrap();
    assert_eq!(original, validated);
}

#[test]
fn test_validate_single_bit_corruption() {
    let original = Hash::hash(b"corruption test").unwrap();
    let mut corrupted = original.to_string().into_bytes();
    corrupted[5] ^= 0x01; // Flip single bit
    let corrupted_str = String::from_utf8(corrupted).unwrap();
    let result = Hash::validate(&corrupted_str);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), original);
}

#[test]
fn test_validate_multi_bit_corruption_recoverable() {
    let original = Hash::hash(b"multi corruption").unwrap();
    let mut corrupted = original.to_string().into_bytes();
    // Corrupt up to 6 bits (within Reed-Solomon correction capability)
    for byte in corrupted.iter_mut().take(4) {
        *byte ^= 0x0F;
    }
    let corrupted_str = String::from_utf8(corrupted).unwrap();
    let result = Hash::validate(&corrupted_str);
    assert!(result.is_ok());
}

#[test]
fn test_validate_unrecoverable_corruption() {
    let original = Hash::hash(b"unrecoverable").unwrap();
    let mut corrupted = original.to_string().into_bytes();

    for byte in corrupted.iter_mut().take(10) {
        *byte ^= 0x1F;
    }

    let corrupted_str = String::from_utf8(corrupted).unwrap();
    let result = Hash::validate(&corrupted_str);

    assert!(result.is_err());
}

#[test]
fn test_validate_bin_uncorrupted() {
    let original = Hash::hash(b"binary validation").unwrap();
    let compact = original.compact().to_vec();
    let mut binary = compact.clone();
    binary.resize(HASH_SIZE_BIN, 0);
    let result = Hash::validate(&mut binary);
    assert!(result.is_ok());
}

#[test]
fn test_validate_bin_corrupted() {
    let original = Hash::hash(b"binary corruption").unwrap();
    let mut binary = original.compact().to_vec();
    binary.resize(HASH_SIZE_BIN, 0);
    if !binary.is_empty() {
        binary[0] ^= 0x01;
    }
    let result = Hash::validate(&mut binary);
    assert!(result.is_ok());
}

// ============================================================================
// Internal Representation and Accessors
// ============================================================================

#[test]
fn test_compact_returns_correct_size() {
    let hash = Hash::hash(b"compact test").unwrap();
    assert_eq!(hash.compact().len(), HASH_SIZE_COMPACT);
}

#[test]
fn test_compact_is_slice_of_inner() {
    let hash = Hash::hash(b"compact slice").unwrap();
    let compact = hash.compact();
    let string_bytes = ps_base64::decode(&hash.to_string().into_bytes());
    assert_eq!(compact, &string_bytes[..HASH_SIZE_COMPACT]);
}

#[test]
fn test_length_accessor() {
    let data = b"length test";
    let hash = Hash::hash(data).unwrap();
    assert_eq!(hash.data_max_len().to_usize(), data.len());
}

#[test]
fn test_length_accessor_zero() {
    let hash = Hash::hash(b"").unwrap();
    assert_eq!(hash.data_max_len().to_usize(), 0);
}

#[test]
fn test_digest_accessor() {
    let hash = Hash::hash(b"digest test").unwrap();
    let digest = hash.digest();
    // Digest should be first 32 bytes of inner
    assert_eq!(digest.len(), 32);
}

#[test]
fn test_parity_accessor() {
    let hash = Hash::hash(b"parity test").unwrap();
    let parity = hash.parity();
    // Parity should be last 6 bytes
    assert_eq!(parity.len(), PARITY_SIZE);
}

#[test]
fn test_to_vec_consistency() {
    let hash = Hash::hash(b"to_vec test").unwrap();
    let vec = hash.to_vec();
    assert_eq!(vec, hash.to_string().into_bytes());
}

#[test]
fn test_data_max_len_small() {
    let data = b"test";
    let hash = Hash::hash(data).unwrap();
    let max_len = hash.data_max_len().to_usize();
    assert_eq!(max_len, data.len());
}

#[test]
fn test_data_max_len_large() {
    let data = vec![42u8; 65536];
    let hash = Hash::hash(&data).unwrap();
    let max_len = hash.data_max_len().to_usize();
    assert_eq!(max_len, data.len());
}

// ============================================================================
// Trait Implementations
// ============================================================================

#[test]
fn test_display_trait() {
    let hash = Hash::hash(b"display").unwrap();
    let displayed = format!("{hash}");
    assert_eq!(displayed, hash.to_string());
}

#[test]
fn test_debug_trait() {
    let hash = Hash::hash(b"debug").unwrap();
    let debug_str = format!("{:?}", hash);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_hash_trait_consistency() {
    use std::collections::HashSet;
    let hash1 = Hash::hash(b"hash trait").unwrap();
    let hash2 = Hash::hash(b"hash trait").unwrap();

    let mut set = HashSet::new();
    set.insert(hash1);
    assert!(set.contains(&hash2));
}

#[test]
fn test_partial_eq_same() {
    let hash1 = Hash::hash(b"eq test").unwrap();
    let hash2 = Hash::hash(b"eq test").unwrap();
    assert_eq!(hash1, hash2);
}

#[test]
fn test_partial_eq_different() {
    let hash1 = Hash::hash(b"eq1").unwrap();
    let hash2 = Hash::hash(b"eq2").unwrap();
    assert_ne!(hash1, hash2);
}

#[test]
fn test_partial_eq_corrupted_recoverable() {
    let original = Hash::hash(b"eq corrupted").unwrap();
    let mut corrupted = original.to_string().into_bytes();
    corrupted[3] ^= 0x01;
    let corrupted_str = String::from_utf8(corrupted).unwrap();
    let recovered = Hash::validate(&corrupted_str).unwrap();
    assert_eq!(original, recovered);
}

#[test]
fn test_ord_trait() {
    let hash1 = Hash::hash(b"ord1").unwrap();
    let hash2 = Hash::hash(b"ord2").unwrap();
    let _ = std::cmp::Ordering::Less;
    let _ = hash1.cmp(&hash2);
}

#[test]
fn test_ord_trait_symmetry() {
    let hash = Hash::hash(b"symmetry").unwrap();
    assert_eq!(hash.cmp(&hash), std::cmp::Ordering::Equal);
}

// ============================================================================
// Type Conversions
// ============================================================================

#[test]
fn test_from_hash_to_array() {
    let hash = Hash::hash(b"to array").unwrap();
    let array: [u8; HASH_SIZE] = hash.into();
    assert_eq!(array.len(), HASH_SIZE);
}

#[test]
fn test_from_hash_ref_to_string() {
    let hash = Hash::hash(b"to string").unwrap();
    let string: String = (&hash).into();
    assert_eq!(string, hash.to_string());
}

#[test]
fn test_from_hash_ref_to_vec() {
    let hash = Hash::hash(b"to vec").unwrap();
    let vec: Vec<u8> = (&hash).into();
    assert_eq!(vec, hash.to_string().into_bytes());
}

#[test]
fn test_try_from_valid_str() {
    let original = Hash::hash(b"from str").unwrap();
    let string = original.to_string();
    let hash = Hash::try_from(string.as_str()).unwrap();
    assert_eq!(hash, original);
}

#[test]
fn test_try_from_valid_slice() {
    let original = Hash::hash(b"from slice").unwrap();
    let bytes = original.to_string().into_bytes();
    let hash = Hash::try_from(bytes.as_slice()).unwrap();
    assert_eq!(hash, original);
}

#[test]
fn test_try_from_invalid_str() {
    let result = Hash::try_from("invalid_hash");
    assert!(result.is_err());
}

#[test]
fn test_try_from_too_short() {
    let result = Hash::try_from("short");
    assert!(result.is_err());
}

#[test]
fn test_try_from_invalid_base64() {
    let invalid = "!@#$%^&*()!@#$%^&*()!@#$%^&*()!@#$%^&*()!@#$%^&*()!@#$%^&";
    let result = Hash::try_from(invalid);
    assert!(result.is_err());
}

// ============================================================================
// Edge Cases and Error Handling
// ============================================================================

#[test]
fn test_clone_copy_semantics() {
    let hash1 = Hash::hash(b"clone").unwrap();
    let hash2 = hash1;
    assert_eq!(hash1, hash2);
}

#[test]
fn test_copy_after_use() {
    let hash1 = Hash::hash(b"copy").unwrap();
    let _str = hash1.to_string();
    let hash2 = hash1; // Should work due to Copy
    assert_eq!(hash1, hash2);
}

#[test]
fn test_data_max_len_boundary() {
    for len in [0, 1, 255, 256, 65536, 0x2f000] {
        let data = vec![0u8; len];
        let hash = Hash::hash(&data).unwrap();
        let max_len = hash.data_max_len().to_usize();
        assert_eq!(max_len, len);
    }
}

#[test]
fn test_validate_bin_vec() {
    let original = Hash::hash(b"bin vec").unwrap();
    let compact = original.compact().to_vec();
    let mut binary = compact.clone();
    binary.resize(HASH_SIZE_BIN, 0);
    let result = Hash::validate_bin_vec(&mut binary);
    assert!(result.is_ok());
}

#[test]
fn test_consecutive_validations() {
    let data = b"consecutive";
    let original = Hash::hash(data).unwrap();

    for _ in 0..10 {
        let validated = Hash::validate(original.to_string()).unwrap();
        assert_eq!(validated, original);
    }
}

// ============================================================================
// Compact Representation Consistency
// ============================================================================

#[test]
fn test_compact_round_trip() {
    let original = Hash::hash(b"compact round trip").unwrap();
    let compact = original.compact().to_vec();
    let mut binary = compact.clone();
    binary.resize(HASH_SIZE_BIN, 0);
    let recovered = Hash::validate(&mut binary).unwrap();
    assert_eq!(original, recovered);
}

#[test]
fn test_compact_preserves_data() {
    let data = b"compact preserves";
    let hash = Hash::hash(data).unwrap();
    let max_len_before = hash.data_max_len();

    let compact = hash.compact().to_vec();
    let mut binary = compact.clone();
    binary.resize(HASH_SIZE_BIN, 0);
    let recovered = Hash::validate(&mut binary).unwrap();
    let max_len_after = recovered.data_max_len();

    assert_eq!(max_len_before, max_len_after);
}

// ============================================================================
// Mixed Validation Scenarios
// ============================================================================

#[test]
fn test_str_then_bin_validation() {
    let original = Hash::hash(b"mixed validation").unwrap();
    let string = original.to_string();
    let validated_str = Hash::validate(&string).unwrap();

    let mut binary = original.compact().to_vec();
    binary.resize(HASH_SIZE_BIN, 0);
    let validated_bin = Hash::validate(&mut binary).unwrap();

    assert_eq!(validated_str, validated_bin);
}

#[test]
fn test_inline_hash_function() {
    let data = b"inline hash";
    let hash1 = crate::hash(data).unwrap();
    let hash2 = Hash::hash(data).unwrap();
    assert_eq!(hash1, hash2);
}
