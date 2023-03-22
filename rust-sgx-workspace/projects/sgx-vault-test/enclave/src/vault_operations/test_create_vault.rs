use sgx_vault_impl::ported::kv_store::KvStore;
use std::prelude::v1::ToString;

use sgx_vault_impl::schema::actions;
use sgx_vault_impl::schema::actions::CreateVaultResult as Result;
use sgx_vault_impl::vault_operations::create_vault::create_vault;
use sgx_vault_impl::vault_operations::store::{key_from_id, load_vault, vault_store};

pub(crate) fn create_vault_works() {
    let request = &actions::CreateVault {
        username: "New Username".to_string(),
        auth_password: "123456".to_string(),
    };
    let display = &match create_vault(request) {
        Result::Created(created) => created,
        Result::Failed(failed) => panic!("{}", failed),
    };

    assert_eq!(display.username, request.username);

    let stored = load_vault(&display.vault_id).unwrap().unwrap();
    assert_eq!(display.vault_id, stored.vault_id);
    assert_eq!(display.username, stored.username);
    assert_eq!(
        display.algorand_address_base32,
        stored.algorand_account.address_base32()
    );

    let mut store = vault_store();
    let key = &key_from_id(&display.vault_id).unwrap();
    store.delete(key).unwrap();
}
