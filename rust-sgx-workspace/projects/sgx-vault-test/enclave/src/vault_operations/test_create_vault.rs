use std::prelude::v1::ToString;

use sgx_vault_impl::schema::actions;
use sgx_vault_impl::schema::actions::CreateVaultResult as Result;
use sgx_vault_impl::schema::entities::XrplAccountDisplay;
use sgx_vault_impl::vault_operations::create_vault::create_vault;
use sgx_vault_impl::vault_operations::store::load_vault;

pub(crate) fn create_vault_works() {
    let request = &actions::CreateVault {
        owner_name: "New Owner".to_string(),
        auth_pin: "123456".to_string(),
        phone_number: None,
    };
    let display = &match create_vault(request) {
        Result::Created(created) => created,
        Result::Failed(failed) => panic!("{}", failed),
    };

    assert_eq!(display.owner_name, request.owner_name);

    let stored = load_vault(&display.vault_id).unwrap().unwrap();
    assert_eq!(display.vault_id, stored.vault_id);
    assert_eq!(display.owner_name, stored.owner_name);
    assert_eq!(
        display.algorand_address_base32,
        stored.algorand_account.address_base32()
    );
    assert_eq!(
        display.xrpl_account,
        XrplAccountDisplay {
            key_type: stored.xrpl_account.key_type,
            public_key_hex: stored.xrpl_account.to_public_key_hex(),
            address_base58: stored.xrpl_account.to_address_base58()
        }
    );
}
