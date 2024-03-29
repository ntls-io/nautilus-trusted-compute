use sgx_types::{sgx_report_t, sgx_status_t, sgx_target_info_t, SgxResult};

/// Interface for working with vault enclave.
pub trait VaultEnclave: Send + 'static {
    fn create_report(
        &self,
        target_info: sgx_target_info_t,
    ) -> SgxResult<SgxResult<(sgx_report_t, [u8; 32])>>;

    fn vault_operation(
        &self,
        sealed_request: &[u8],
        sealed_response_capacity: usize,
    ) -> SgxResult<SgxResult<Box<[u8]>>>;

    /// Wrapper: Retry [`Self::vault_operation`] with increasing capacity.
    fn vault_operation_with_retry(&self, sealed_request: &[u8]) -> SgxResult<SgxResult<Box<[u8]>>> {
        // Attempt sizes: 1 KiB, 64 KiB, 1 MiB
        for &sealed_response_capacity in &[1 << 10, 1 << 16, 1 << 20] {
            let result = self.vault_operation(sealed_request, sealed_response_capacity);
            match result {
                Ok(Err(sgx_status_t::SGX_ERROR_FAAS_BUFFER_TOO_SHORT)) => {
                    println!(
                        "DEBUG: vault_operation_with_retry: capacity={} too short, retrying…",
                        sealed_response_capacity
                    )
                }
                result => {
                    if cfg!(feature = "verbose-debug-logging") {
                        println!(
                            "DEBUG: vault_operation_with_retry: capacity={} returning {:?}",
                            sealed_response_capacity, result
                        );
                    }
                    return result;
                }
            }
        }
        println!("DEBUG: vault_operation_with_retry: giving up!",);
        Ok(Err(sgx_status_t::SGX_ERROR_FAAS_BUFFER_TOO_SHORT))
    }
}
