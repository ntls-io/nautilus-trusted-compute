use std::prelude::v1::ToString;

use sgx_vault_impl::schema::actions;
use sgx_vault_impl::schema::actions::CreateVaultResult;
use sgx_vault_impl::schema::entities::VaultDisplay;
use sgx_vault_impl::vault_operations::create_vault::create_vault;

pub fn create_test_vault() -> VaultDisplay {
    type Result = CreateVaultResult;

    let request = &actions::CreateVault {
        username: "New Username".to_string(),
        auth_password: "123456".to_string(),
    };
    match create_vault(request) {
        Result::Created(created) => created,
        otherwise => panic!("{:?}", otherwise),
    }
}
