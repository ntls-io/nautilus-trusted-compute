//! Cryptographic helper code.

use rand::{thread_rng, RngCore};
use rusty_sodalite::types::SecureSeed;

/// Generate a new secure seed using [`thread_rng`].
pub fn generate_secure_seed() -> Result<SecureSeed, rand::Error> {
    let mut seed = SecureSeed::default();
    thread_rng().try_fill_bytes(&mut seed)?;
    Ok(seed)
}
