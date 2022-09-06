use std::prelude::v1::ToString;

use sgx_vault_impl::schema::actions;
use sgx_vault_impl::schema::actions::{OnfidoCheckResult, SaveOnfidoCheckResult};
use sgx_vault_impl::schema::types::VaultPin;
use sgx_vault_impl::vault_operations::save_onfido_check::save_onfido_check;

use crate::helpers::vault_store::{create_test_vault, load_test_check};

fn get_check() -> OnfidoCheckResult {
    OnfidoCheckResult {
        id: "stub id".to_string(),
        href: "stub href".to_string(),
        result: "stub result".to_string(),
        sub_result: None,
    }
}

pub(crate) fn save_onfido_check_works() {
    let existing = create_test_vault();
    let check = get_check();

    let request = actions::SaveOnfidoCheck {
        vault_id: existing.vault_id.clone(),
        auth_pin: VaultPin::from("123456"),
        check: check.clone(),
    };
    match save_onfido_check(&request) {
        SaveOnfidoCheckResult::Saved => {}
        otherwise => panic!("{:?}", otherwise),
    };

    let saved = load_test_check(&existing);
    assert_eq!(saved, check)
}

pub(crate) fn save_onfido_check_malformed_vault_id() {
    let request = actions::SaveOnfidoCheck {
        vault_id: "malformed".into(),
        auth_pin: "123456".into(),
        check: get_check(),
    };

    match save_onfido_check(&request) {
        SaveOnfidoCheckResult::Failed(_) => (),
        otherwise => panic!("{:?}", otherwise),
    }
}

pub(crate) fn save_onfido_check_bad_pin() {
    let existing = create_test_vault();

    let request = actions::SaveOnfidoCheck {
        vault_id: existing.vault_id,
        auth_pin: "000000".to_string(),
        check: get_check(),
    };

    match save_onfido_check(&request) {
        SaveOnfidoCheckResult::InvalidAuth => (),
        otherwise => panic!("{:?}", otherwise),
    }
}
