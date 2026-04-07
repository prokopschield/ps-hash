use ps_base64::encode;

use super::super::Hash;

impl Hash {
    #[must_use]
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        encode(&self.inner)
    }
}
