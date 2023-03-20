use std::prelude::v1::ToString;

use sgx_vault_impl::schema::actions;
use sgx_vault_impl::schema::actions::CreateVaultResult as Result;
use sgx_vault_impl::vault_operations::create_vault::create_vault;
use sgx_vault_impl::vault_operations::store::load_vault;

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
}
