use crate::schema::actions::{LoadOnfidoCheck, LoadOnfidoCheckResult};
use crate::vault_operations::store::unlock_vault;

pub fn load_onfido_check(request: &LoadOnfidoCheck) -> LoadOnfidoCheckResult {
    let stored = match unlock_vault(&request.vault_id, &request.auth_pin) {
        Ok(stored) => stored,
        Err(err) => return err.into(),
    };

    stored.onfido_check_result.clone().map_or(
        LoadOnfidoCheckResult::NotFound,
        LoadOnfidoCheckResult::Loaded,
    )
}
