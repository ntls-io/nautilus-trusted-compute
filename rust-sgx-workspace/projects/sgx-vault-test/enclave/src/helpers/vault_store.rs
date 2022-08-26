use std::prelude::v1::ToString;

use sgx_vault_impl::schema::actions;
use sgx_vault_impl::schema::actions::{
    CreateVaultResult,
    LoadOnfidoCheck,
    LoadOnfidoCheckResult,
    OnfidoCheckResult,
    SaveOnfidoCheck,
    SaveOnfidoCheckResult,
};
use sgx_vault_impl::schema::entities::VaultDisplay;
use sgx_vault_impl::vault_operations::create_vault::create_vault;
use sgx_vault_impl::vault_operations::load_onfido_check::load_onfido_check;
use sgx_vault_impl::vault_operations::save_onfido_check::save_onfido_check;

pub fn create_test_vault() -> VaultDisplay {
    type Result = CreateVaultResult;

    let request = &actions::CreateVault {
        owner_name: "New Owner".to_string(),
        auth_pin: "123456".to_string(),
        phone_number: None,
    };
    match create_vault(request) {
        Result::Created(created) => created,
        otherwise => panic!("{:?}", otherwise),
    }
}

pub fn create_test_check(existing: &VaultDisplay) -> OnfidoCheckResult {
    let check = OnfidoCheckResult {
        id: "stub id".to_string(),
        href: "stub href".to_string(),
        result: "stub result".to_string(),
        sub_result: None,
    };
    match save_onfido_check(&SaveOnfidoCheck {
        vault_id: existing.vault_id.clone(),
        auth_pin: "123456".to_string(),
        check: check.clone(),
    }) {
        SaveOnfidoCheckResult::Saved => {}
        otherwise => panic!("{:?}", otherwise),
    };
    check
}

pub fn load_test_check(existing: &VaultDisplay) -> OnfidoCheckResult {
    match load_onfido_check(&LoadOnfidoCheck {
        vault_id: existing.vault_id.clone(),
        auth_pin: "123456".to_string(),
    }) {
        LoadOnfidoCheckResult::Loaded(check) => check,
        otherwise => panic!("{:?}", otherwise),
    }
}
