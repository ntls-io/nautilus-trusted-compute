use std::prelude::v1::ToString;

use crate::schema::actions::{CreateVault, CreateVaultResult};
use crate::schema::entities::{AlgorandAccount, VaultDisplay, VaultStorable, XrplAccount};
use crate::vault_operations::store::save_new_vault;

type Result = CreateVaultResult;

pub fn create_vault(request: &CreateVault) -> Result {
    // TODO(Pi): Pull account / keypair creation into a separate operation.
    //           For now, just generate both Algorand and XRP keypairs.
    let new_algorand_account = AlgorandAccount::generate();
    let new_xrpl_account = XrplAccount::generate_default();

    let storable = VaultStorable {
        vault_id: new_xrpl_account.to_address_base58(),
        owner_name: request.owner_name.clone(),
        auth_pin: request.auth_pin.clone(),
        phone_number: request.phone_number.clone(),

        algorand_account: new_algorand_account,
        xrpl_account: new_xrpl_account,

        onfido_check_result: None,
    };
    match save_new_vault(&storable) {
        Ok(()) => Result::Created(VaultDisplay::from(storable)),
        Err(err) => Result::Failed(err.to_string()),
    }
}
