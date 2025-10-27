use crate::{Hash, HASH_SIZE_COMPACT};

impl Hash {
    /// Produces a compact binary form of this [`Hash`].
    ///
    /// To turn the binary form back into a [`Hash`], use [`Hash::validate`].
    #[inline]
    #[must_use]
    pub fn compact(&self) -> &[u8] {
        &self.inner[..HASH_SIZE_COMPACT]
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{hash, Hash};

    #[test]
    fn roundtrip() -> Result<(), Box<dyn Error>> {
        for i in 0..1000 {
            let input = "X".repeat(i);
            let h = hash(&input)?;
            let mut c = h.compact();

            let r1 = Hash::validate(&c)?;

            assert_eq!(r1, h, "validated should equal original");

            let r2 = Hash::validate(&mut c)?;

            assert_eq!(r1, r2, "validated hashes should be equal");
        }

        Ok(())
    }
}
