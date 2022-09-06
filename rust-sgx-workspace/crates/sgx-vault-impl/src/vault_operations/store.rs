use std::io;
use std::prelude::v1::Box;

use sgx_trts::memeq::ConsttimeMemEq;
use thiserror::Error;

use crate::ported::kv_store::fs::{FsStore, SgxFiler};
use crate::ported::kv_store::{Key, KvStore};
use crate::schema::entities::VaultStorable;

type VaultStore = FsStore<SgxFiler, VaultStorable>;

// FIXME: Hardcoded
pub const VAULT_STORE_DIR: &str = "vault_store";

pub fn vault_store() -> VaultStore {
    FsStore::new(VAULT_STORE_DIR, SgxFiler)
}

pub fn save_new_vault(new_vault: &VaultStorable) -> Result<(), io::Error> {
    let mut store = vault_store();
    let key = &key_from_id(&new_vault.vault_id)?;
    match store.try_insert(key, new_vault)? {
        None => Ok(()),
        Some(existing) => panic!(
            "save_vault: key conflict! key = {:?}, existing owner = {:?}, new owner = {:?}",
            key, existing.owner_name, new_vault.owner_name
        ),
    }
}

/// Return `None` if `vault_id` not found.
pub fn load_vault(vault_id: &str) -> Result<Option<VaultStorable>, io::Error> {
    let store = vault_store();
    let key = &key_from_id(vault_id)?;
    store.load(key)
}

pub fn key_from_id(vault_id: &str) -> Result<Box<Key>, io::Error> {
    // XXX: Assume XRP address, for now.
    let address = ripple_address_codec::decode_account_id(vault_id).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("key_from_id failed for vault_id = {:?}: {}", vault_id, err),
        )
    })?;
    Ok(address.into())
}

/// Load and authenticate access to a vault.
pub fn unlock_vault(vault_id: &str, auth_pin: &str) -> Result<VaultStorable, UnlockVaultError> {
    let stored: VaultStorable = load_vault(vault_id)?.ok_or(UnlockVaultError::InvalidVaultId)?;

    match ConsttimeMemEq::consttime_memeq(stored.auth_pin.as_bytes(), auth_pin.as_bytes()) {
        true => Ok(stored),
        false => Err(UnlockVaultError::InvalidAuthPin),
    }
}

/// [`unlock_vault`] failed.
///
/// # Security note
///
/// This representation distinguishes `InvalidVaultId` and `InvalidAuthPin`,
/// but this distinction should be limited to internal interfaces:
/// public interfaces should combine invalid authentication cases to avoid information leakage.
#[derive(Debug, Error)]
pub enum UnlockVaultError {
    #[error("invalid vault ID provided")]
    InvalidVaultId,

    #[error("invalid authentication PIN provided")]
    InvalidAuthPin,

    #[error("I/O error while opening vault")]
    IoError(#[from] io::Error),
}

pub fn mutate_vault(
    vault_id: &str,
    mutate_fn: impl FnOnce(VaultStorable) -> VaultStorable,
) -> Result<Option<VaultStorable>, io::Error> {
    let mut store = vault_store();
    let key = &key_from_id(vault_id)?;
    store.mutate(key, mutate_fn)
}
