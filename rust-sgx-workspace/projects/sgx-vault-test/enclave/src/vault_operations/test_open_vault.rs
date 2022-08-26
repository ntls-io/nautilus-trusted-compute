use std::prelude::v1::ToString;

use sgx_vault_impl::schema::actions;
use sgx_vault_impl::schema::actions::OpenVaultResult;
use sgx_vault_impl::vault_operations::open_vault::open_vault;

use crate::helpers::vault_store;

type Result = OpenVaultResult;

pub(crate) fn open_vault_works() {
    let existing = &vault_store::create_test_vault();

    let request = &actions::OpenVault {
        vault_id: existing.vault_id.clone(),
        auth_pin: "123456".to_string(),
    };
    let display = &match open_vault(request) {
        Result::Opened(opened) => opened,
        otherwise => panic!("{:?}", otherwise),
    };

    assert_eq!(display, existing);
}

pub(crate) fn open_vault_malformed_vault_id() {
    let request = &actions::OpenVault {
        vault_id: "malformed".to_string(),
        auth_pin: "123456".to_string(),
    };

    match open_vault(request) {
        Result::Failed(_) => (),
        otherwise => panic!("{:?}", otherwise),
    }
}

pub(crate) fn open_vault_bad_pin() {
    let existing = &vault_store::create_test_vault();

    let request = &actions::OpenVault {
        vault_id: existing.vault_id.clone(),
        auth_pin: "000000".to_string(),
    };

    match open_vault(request) {
        Result::InvalidAuth => (),
        otherwise => panic!("{:?}", otherwise),
    }
}
