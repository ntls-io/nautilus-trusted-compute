#![no_std]

extern crate sgx_tstd as std;

// Re-export ECALL implementations:
pub use sgx_vault_impl::ecalls::enclave_create_report::enclave_create_report;
pub use sgx_vault_impl::ecalls::vault_operation::vault_operation;
