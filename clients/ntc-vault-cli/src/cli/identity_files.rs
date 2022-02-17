//! Support for working with identity files.

use std::path::{Path, PathBuf};

use anyhow::Context;
use confy::ConfyError;
use rusty_sodalite::types::SecureSeed;
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::serde_as;

use crate::core::identity::VaultIdentity;

#[derive(Default, Debug)] // core
#[serde_as]
#[derive(Serialize, Deserialize)] // serde
pub(crate) struct VaultIdentityConfig {
    pub(crate) name: String,

    #[serde_as(as = "Base64")]
    pub(crate) seed: SecureSeed,
}

impl VaultIdentityConfig {
    pub(crate) fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        confy::load_path(path).with_context(|| format!("Failed to load {}", path.to_string_lossy()))
    }

    pub(crate) fn store(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        confy::store_path(path, self)
            .with_context(|| format!("Failed to store {}", path.to_string_lossy()))
    }

    pub(crate) fn get_default_path() -> Result<PathBuf, ConfyError> {
        confy::get_configuration_file_path("ntc-vault", "identity")
    }
}

impl From<VaultIdentityConfig> for VaultIdentity {
    fn from(VaultIdentityConfig { name, seed }: VaultIdentityConfig) -> Self {
        let seed = seed.into();
        VaultIdentity { name, seed }
    }
}
