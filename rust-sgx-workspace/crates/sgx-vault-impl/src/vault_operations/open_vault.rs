use crate::schema::actions::{OpenVault, OpenVaultResult};
use crate::schema::entities::VaultDisplay;
use crate::vault_operations::store::unlock_vault;

pub fn open_vault(request: &OpenVault) -> OpenVaultResult {
    let stored = match unlock_vault(&request.vault_id, &request.auth_pin) {
        Ok(stored) => stored,
        Err(err) => return err.into(),
    };

    OpenVaultResult::Opened(VaultDisplay::from(stored))
}
