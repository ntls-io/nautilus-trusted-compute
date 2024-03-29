//! Implement [`VaultEnclave`] using [`safe_ecalls`].

use http_service_impl::traits::VaultEnclave;
use sgx_types::{sgx_report_t, sgx_target_info_t, SgxResult};
use sgx_urts::SgxEnclave;

use crate::safe_ecalls;

pub(crate) struct VaultEnclaveImpl {
    pub(crate) enclave: SgxEnclave,
}

impl VaultEnclave for VaultEnclaveImpl {
    fn create_report(
        &self,
        target_info: sgx_target_info_t,
    ) -> SgxResult<SgxResult<(sgx_report_t, [u8; 32])>> {
        safe_ecalls::safe_enclave_create_report(self.enclave.geteid(), target_info)
    }

    fn vault_operation(
        &self,
        sealed_request: &[u8],
        sealed_response_capacity: usize,
    ) -> SgxResult<SgxResult<Box<[u8]>>> {
        safe_ecalls::safe_vault_operation(
            self.enclave.geteid(),
            sealed_request,
            sealed_response_capacity,
        )
    }
}
