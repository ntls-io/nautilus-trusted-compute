//! Test [`sgx_vault_impl::vault_operations::store`]

use std::prelude::v1::ToString;

use sgx_vault_impl::ported::kv_store::KvStore;
use sgx_vault_impl::schema::entities::VaultDisplay;
use sgx_vault_impl::vault_operations::store::{key_from_id, unlock_vault, vault_store};

use crate::helpers::vault_store::create_test_vault_with_username;

pub(crate) fn unlock_vault_works() {
    let existing = create_test_vault_with_username("Unlock Vault Works");
    let stored = unlock_vault(&existing.vault_id, "123456").unwrap();
    assert_eq!(existing, VaultDisplay::from(stored));

    let mut store = vault_store();
    let key = &key_from_id(&existing.vault_id).unwrap();
    store.delete(key).unwrap();
}

pub(crate) fn unlock_vault_not_found() {
    let existing = create_test_vault_with_username("Unlock Vault Not Found");
    let mut store = vault_store();
    let key = &key_from_id(&existing.vault_id).unwrap();
    store.delete(key).unwrap();

    let err = unlock_vault(&existing.vault_id, "123456").unwrap_err();
    assert_eq!(err.to_string(), "invalid vault ID provided");
}

pub(crate) fn unlock_vault_bad_auth_pin() {
    let existing = create_test_vault_with_username("Unlock Vault Bad Auth Pin");
    let err = unlock_vault(&existing.vault_id, "000000").unwrap_err();
    assert_eq!(err.to_string(), "invalid authentication PIN provided");

    let mut store = vault_store();
    let key = &key_from_id(&existing.vault_id).unwrap();
    store.delete(key).unwrap();
}
