use super::super::Hash;

impl PartialOrd for Hash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hash {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::cmp::Ordering;

    use crate::Hash;

    #[test]
    fn cmp_equal() {
        let h1 = Hash::hash(b"equal").expect("hashing should succeed");
        let h2 = Hash::hash(b"equal").expect("hashing should succeed");
        assert_eq!(h1.cmp(&h2), Ordering::Equal);
    }

    #[test]
    fn cmp_not_equal() {
        let h1 = Hash::hash(b"a").expect("hashing should succeed");
        let h2 = Hash::hash(b"b").expect("hashing should succeed");
        assert_ne!(h1.cmp(&h2), Ordering::Equal);
    }

    #[test]
    fn cmp_reflexive() {
        let h = Hash::hash(b"reflexive").expect("hashing should succeed");
        assert_eq!(h.cmp(&h), Ordering::Equal);
    }

    #[test]
    fn cmp_antisymmetric() {
        let h1 = Hash::hash(b"anti1").expect("hashing should succeed");
        let h2 = Hash::hash(b"anti2").expect("hashing should succeed");
        let cmp1 = h1.cmp(&h2);
        let cmp2 = h2.cmp(&h1);
        assert_eq!(cmp1, cmp2.reverse());
    }

    #[test]
    fn cmp_transitive() {
        let mut hashes: Vec<Hash> = (0..10)
            .map(|i| Hash::hash([i]).expect("hashing should succeed"))
            .collect();
        hashes.sort();

        for i in 0..hashes.len() - 2 {
            let a = &hashes[i];
            let b = &hashes[i + 1];
            let c = &hashes[i + 2];

            if a.cmp(b) == Ordering::Less && b.cmp(c) == Ordering::Less {
                assert_eq!(a.cmp(c), Ordering::Less);
            }
        }
    }

    #[test]
    fn partial_cmp_returns_some() {
        let h1 = Hash::hash(b"some1").expect("hashing should succeed");
        let h2 = Hash::hash(b"some2").expect("hashing should succeed");
        assert!(h1.partial_cmp(&h2).is_some());
    }

    #[test]
    fn partial_cmp_matches_cmp() {
        let h1 = Hash::hash(b"match1").expect("hashing should succeed");
        let h2 = Hash::hash(b"match2").expect("hashing should succeed");
        assert_eq!(h1.partial_cmp(&h2), Some(h1.cmp(&h2)));
    }

    #[test]
    fn ord_enables_sorting() {
        let mut hashes: Vec<Hash> = (0..5)
            .map(|i| Hash::hash([i]).expect("hashing should succeed"))
            .collect();

        let before = hashes.clone();
        hashes.sort();

        for i in 0..hashes.len() - 1 {
            assert!(hashes[i] <= hashes[i + 1]);
        }

        hashes.sort_by(|a, b| b.cmp(a));

        for i in 0..hashes.len() - 1 {
            assert!(hashes[i] >= hashes[i + 1]);
        }

        let _ = before;
    }

    #[test]
    fn ord_enables_btreeset() {
        use std::collections::BTreeSet;

        let mut set = BTreeSet::new();
        set.insert(Hash::hash(b"a").expect("hashing should succeed"));
        set.insert(Hash::hash(b"b").expect("hashing should succeed"));
        set.insert(Hash::hash(b"a").expect("hashing should succeed"));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn ord_enables_min_max() {
        let h1 = Hash::hash(b"min").expect("hashing should succeed");
        let h2 = Hash::hash(b"max").expect("hashing should succeed");
        let _ = std::cmp::min(h1, h2);
        let _ = std::cmp::max(h1, h2);
    }

    #[test]
    fn ord_trait_bound() {
        fn assert_ord<T: Ord>() {}
        assert_ord::<Hash>();
    }

    #[test]
    fn partial_ord_trait_bound() {
        fn assert_partial_ord<T: PartialOrd>() {}
        assert_partial_ord::<Hash>();
    }

    #[test]
    fn cmp_consistent_with_eq() {
        let h1 = Hash::hash(b"consistent").expect("hashing should succeed");
        let h2 = Hash::hash(b"consistent").expect("hashing should succeed");
        assert_eq!(h1.cmp(&h2) == Ordering::Equal, h1 == h2);
    }
}
