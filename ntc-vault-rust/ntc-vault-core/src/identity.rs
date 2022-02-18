//! Identity and key management.

use rusty_sodalite::safe_sign::{safe_sign_keypair_seed, SafeSignPublicKey};
use rusty_sodalite::types::SafeSecureSeed;

pub struct VaultIdentity {
    pub name: String,

    pub seed: SafeSecureSeed,
}

impl VaultIdentity {
    pub fn get_sign_public_key(&self) -> SafeSignPublicKey {
        let (pk, _sk) = safe_sign_keypair_seed(&self.seed);
        pk
    }
}
