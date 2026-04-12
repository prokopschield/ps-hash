use super::super::Hash;

impl std::hash::Hash for Hash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.inner);
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::hash::{DefaultHasher, Hash as StdHash, Hasher};

    use crate::Hash;

    fn compute_hash<T: StdHash>(t: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        t.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn hash_deterministic() {
        let h = Hash::hash(b"deterministic").expect("hashing should succeed");
        let hash1 = compute_hash(&h);
        let hash2 = compute_hash(&h);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn hash_same_for_equal_hashes() {
        let h1 = Hash::hash(b"same").expect("hashing should succeed");
        let h2 = Hash::hash(b"same").expect("hashing should succeed");
        assert_eq!(compute_hash(&h1), compute_hash(&h2));
    }

    #[test]
    fn hash_different_for_different_hashes() {
        let h1 = Hash::hash(b"diff1").expect("hashing should succeed");
        let h2 = Hash::hash(b"diff2").expect("hashing should succeed");
        assert_ne!(compute_hash(&h1), compute_hash(&h2));
    }

    #[test]
    fn hash_enables_hashset() {
        let mut set = HashSet::new();
        set.insert(Hash::hash(b"a").expect("hashing should succeed"));
        set.insert(Hash::hash(b"b").expect("hashing should succeed"));
        set.insert(Hash::hash(b"a").expect("hashing should succeed"));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn hash_enables_hashmap_key() {
        let mut map = HashMap::new();
        let h1 = Hash::hash(b"key1").expect("hashing should succeed");
        let h2 = Hash::hash(b"key2").expect("hashing should succeed");
        map.insert(h1, "value1");
        map.insert(h2, "value2");
        assert_eq!(map.get(&h1), Some(&"value1"));
        assert_eq!(map.get(&h2), Some(&"value2"));
    }

    #[test]
    fn hash_lookup_works() {
        let mut set = HashSet::new();
        let original = Hash::hash(b"lookup").expect("hashing should succeed");
        set.insert(original);

        let same = Hash::hash(b"lookup").expect("hashing should succeed");
        assert!(set.contains(&same));
    }

    #[test]
    fn hash_lookup_after_validation() {
        let mut set = HashSet::new();
        let original = Hash::hash(b"validated").expect("hashing should succeed");
        set.insert(original);

        let validated = Hash::validate(original.to_string())
            .expect("validation of an uncorrupted hash should succeed");
        assert!(set.contains(&validated));
    }

    #[test]
    fn hash_consistent_with_eq() {
        let h1 = Hash::hash(b"consistent").expect("hashing should succeed");
        let h2 = Hash::hash(b"consistent").expect("hashing should succeed");

        if h1 == h2 {
            assert_eq!(compute_hash(&h1), compute_hash(&h2));
        }
    }

    #[test]
    fn hash_trait_bound() {
        fn assert_hash<T: StdHash>() {}
        assert_hash::<Hash>();
    }

    #[test]
    fn hash_after_corruption_recovery() {
        let mut set = HashSet::new();
        let original = Hash::hash(b"recovery").expect("hashing should succeed");
        set.insert(original);

        let mut corrupted = original.to_string().into_bytes();
        corrupted[5] ^= 0x01;
        let recovered = Hash::validate(
            String::from_utf8(corrupted).expect("corrupted bytes should be valid UTF-8"),
        )
        .expect("single-bit corruption should be recovered");
        assert!(set.contains(&recovered));
    }

    #[test]
    fn hash_many_insertions() {
        let mut set = HashSet::new();

        for i in 0u32..100 {
            let h = Hash::hash(i.to_le_bytes()).expect("hashing should succeed");
            set.insert(h);
        }

        assert_eq!(set.len(), 100);
    }
}
