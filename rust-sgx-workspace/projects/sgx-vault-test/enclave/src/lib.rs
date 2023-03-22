#![no_std]

#[macro_use]
extern crate sgx_tstd as std;

mod helpers;
mod ported;
mod schema;
mod vault_operations;

use std::backtrace;
use std::string::String;
use std::vec::Vec;

use sgx_tunittest::*;

#[no_mangle]
pub extern "C" fn run_tests_ecall() -> usize {
    backtrace::enable_backtrace("enclave.signed.so", backtrace::PrintFormat::Short).unwrap();

    rsgx_unit_tests!(
        ported::proptest_crypto::prop_soda_box_roundtrips,
        ported::test_attestation::create_report_impl_works,
        ported::test_crypto::soda_box_decrypt_works,
        ported::test_crypto::soda_box_encrypt_works,
        ported::test_kv_store::test_alter,
        ported::test_kv_store::test_load_save_delete,
        ported::test_kv_store::test_mutate,
        ported::test_kv_store::test_try_insert,
        ported::test_kv_store_fs::prop_fs_safe_roundtrip,
        schema::test_sealing::prop_seal_unseal_msgpack_roundtrips,
        schema::test_sealing::prop_seal_unseal_roundtrips,
        vault_operations::test_create_vault::create_vault_works,
        vault_operations::test_dispatch::vault_operation_sealing_works,
        vault_operations::test_open_vault::open_vault_bad_pin,
        vault_operations::test_open_vault::open_vault_malformed_vault_id,
        vault_operations::test_open_vault::open_vault_works,
        vault_operations::test_sign_transaction::sign_transaction_empty,
        vault_operations::test_sign_transaction::sign_transaction_malformed_transaction,
        vault_operations::test_sign_transaction::sign_transaction_without_tag,
        vault_operations::test_sign_transaction::sign_transaction_works,
        vault_operations::test_sign_transaction_msgpack::prop_transaction_msgpack_roundtrips,
        vault_operations::test_store::unlock_vault_bad_auth_pin,
        vault_operations::test_store::unlock_vault_not_found,
        vault_operations::test_store::unlock_vault_works,
    )
}
