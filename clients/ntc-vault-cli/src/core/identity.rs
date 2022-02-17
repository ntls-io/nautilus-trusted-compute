//! Identity and key management.

use rusty_sodalite::safe_sign::{safe_sign_keypair_seed, SafeSignPublicKey};
use rusty_sodalite::types::SafeSecureSeed;

pub(crate) struct VaultIdentity {
    pub(crate) name: String,

    pub(crate) seed: SafeSecureSeed,
}

impl VaultIdentity {
    pub(crate) fn get_sign_public_key(&self) -> SafeSignPublicKey {
        let (pk, _sk) = safe_sign_keypair_seed(&self.seed);
        pk
    }
}
