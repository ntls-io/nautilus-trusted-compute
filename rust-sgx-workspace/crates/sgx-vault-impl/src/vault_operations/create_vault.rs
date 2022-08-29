use std::prelude::v1::ToString;

use crate::schema::actions::{CreateVault, CreateVaultResult};
use crate::schema::entities::{AlgorandAccount, VaultDisplay, VaultStorable};
use crate::vault_operations::store::save_new_vault;

type Result = CreateVaultResult;

pub fn create_vault(request: &CreateVault) -> Result {
    // TODO(Pi): Pull account / keypair creation into a separate operation.
    //           For now, just generate both Algorand keypairs.
    let new_algorand_account = AlgorandAccount::generate();

    let storable = VaultStorable {
        vault_id: new_algorand_account.address_base32(),
        owner_name: request.owner_name.clone(),
        auth_pin: request.auth_pin.clone(),
        phone_number: request.phone_number.clone(),

        algorand_account: new_algorand_account,
    };
    match save_new_vault(&storable) {
        Ok(()) => Result::Created(VaultDisplay::from(storable)),
        Err(err) => Result::Failed(err.to_string()),
    }
}
