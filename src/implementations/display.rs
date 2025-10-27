use std::fmt::Display;

use ps_base64::base64;

use crate::Hash;

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        base64::encode_into(&self.inner, f)
    }
}
