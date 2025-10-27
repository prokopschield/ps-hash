use ps_base64::encode;

use crate::Hash;

impl Hash {
    #[must_use]
    #[allow(clippy::inherent_to_string_shadow_display)]
    /// Returns the base64-encoded representation of this [`Hash`].
    ///
    /// This method uses a single allocation,
    /// avoiding the potential reallocations of `Display`.
    pub fn to_string(&self) -> String {
        encode(&self.inner)
    }
}
