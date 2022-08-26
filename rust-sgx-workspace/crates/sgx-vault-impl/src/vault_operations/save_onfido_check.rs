use crate::schema::actions::{SaveOnfidoCheck, SaveOnfidoCheckResult};
use crate::vault_operations::store::{mutate_vault, unlock_vault};

pub fn save_onfido_check(request: &SaveOnfidoCheck) -> SaveOnfidoCheckResult {
    let stored = match unlock_vault(&request.vault_id, &request.auth_pin) {
        Ok(ok) => ok,
        Err(err) => return err.into(),
    };

    match mutate_vault(&stored.vault_id, |mut stored| {
        // FIXME: Avoid mut?
        stored.onfido_check_result = Some(request.check.clone());
        stored
    }) {
        Ok(ok) => ok,
        Err(err) => return err.into(),
    }
    .expect("save_onfido_check: vault disappeared!");

    SaveOnfidoCheckResult::Saved
}
