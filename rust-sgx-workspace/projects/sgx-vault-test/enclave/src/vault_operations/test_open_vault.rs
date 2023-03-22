use sgx_vault_impl::ported::kv_store::KvStore;
use std::prelude::v1::ToString;

use sgx_vault_impl::schema::actions;
use sgx_vault_impl::schema::actions::OpenVaultResult;
use sgx_vault_impl::vault_operations::open_vault::open_vault;
use sgx_vault_impl::vault_operations::store::{key_from_id, vault_store};

use crate::helpers::vault_store;

type Result = OpenVaultResult;

pub(crate) fn open_vault_works() {
    let existing = &vault_store::create_test_vault_with_username("Open Vault");

    let request = &actions::OpenVault {
        vault_id: existing.vault_id.clone(),
        auth_password: "123456".to_string(),
    };
    let display = &match open_vault(request) {
        Result::Opened(opened) => opened,
        otherwise => panic!("{:?}", otherwise),
    };

    assert_eq!(display, existing);

    let mut store = vault_store();
    let key = &key_from_id(&existing.vault_id).unwrap();
    store.delete(key).unwrap();
}

pub(crate) fn open_vault_malformed_vault_id() {
    let request = &actions::OpenVault {
        vault_id: "malformed".to_string(),
        auth_password: "123456".to_string(),
    };

    match open_vault(request) {
        Result::InvalidAuth => (),
        otherwise => panic!("{:?}", otherwise),
    }
}

pub(crate) fn open_vault_bad_pin() {
    let existing = &vault_store::create_test_vault_with_username("Bad Pin");

    let request = &actions::OpenVault {
        vault_id: existing.vault_id.clone(),
        auth_password: "000000".to_string(),
    };

    match open_vault(request) {
        Result::InvalidAuth => (),
        otherwise => panic!("{:?}", otherwise),
    }

    let mut store = vault_store();
    let key = &key_from_id(&existing.vault_id).unwrap();
    store.delete(key).unwrap();
}
