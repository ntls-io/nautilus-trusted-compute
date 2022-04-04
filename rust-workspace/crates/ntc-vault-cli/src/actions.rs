//! CLI action implementations.
//!
//! These provide the functionality invoked by [`crate::commands`].

use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context};
use ntc_data_packages::identity::VaultIdentity;

use crate::crypto::generate_secure_seed;
use crate::identity_files::VaultIdentityConfig;
use crate::{compat, fs_io};

pub fn identity_create(name: String) -> anyhow::Result<()> {
    let path = &VaultIdentityConfig::get_default_path()?;
    if compat::try_exists(path)? {
        Err(anyhow!("File exists: {}", path.to_string_lossy())
            .context("Identity already configured"))
    } else {
        let seed = generate_secure_seed()?;
        let config = VaultIdentityConfig { name, seed };
        config.store(path)?;
        println!("Identity created at {}", path.to_string_lossy());
        Ok(())
    }
}

pub fn identity_show() -> anyhow::Result<()> {
    let path = &VaultIdentityConfig::get_default_path()?;
    if compat::try_exists(path)? {
        let config = VaultIdentityConfig::load(path)?;
        let identity: VaultIdentity = config.into();
        let pk = identity.get_sign_public_key();
        let pk_base64 = base64::encode(*pk.as_ref());
        println!("Path:       {}", path.to_string_lossy());
        println!("Name:       {}", identity.name);
        println!("Public key: {}", pk_base64);
        Ok(())
    } else {
        Err(anyhow!("File not found: {}", path.to_string_lossy())
            .context("Identity not configured"))
    }
}

pub(crate) fn data_create(
    metadata: &Path,
    schema: &Path,
    data: &Path,
    output: &Path,
) -> anyhow::Result<()> {
    fs_io::read_metadata(metadata).context(anyhow!("failed to read metadata from {metadata:?}"))?;
    fs::read(schema)?;
    fs::read(data)?;
    fs::write(output, "")?;
    Ok(())
}

pub fn data_inspect(path: &Path) -> anyhow::Result<()> {
    fs::read(path)
        .with_context(|| format!("failed to inspect file: {}", path.to_string_lossy()))?;
    Ok(())
}
