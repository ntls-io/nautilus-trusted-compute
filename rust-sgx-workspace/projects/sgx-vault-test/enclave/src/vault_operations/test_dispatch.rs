use std::prelude::v1::ToString;

use sgx_vault_impl::ported::crypto::SodaBoxCrypto;
use sgx_vault_impl::schema::actions::{OpenVault, OpenVaultResult, VaultRequest, VaultResponse};
use sgx_vault_impl::schema::sealing::{seal_msgpack, unseal_non_secret_msgpack};
use sgx_vault_impl::vault_operations::dispatch::vault_operation_impl;

pub(crate) fn vault_operation_sealing_works() {
    let client_crypto = &mut SodaBoxCrypto::from_seed([0; 32]);
    let enclave_crypto = SodaBoxCrypto::new();

    // Seal
    let vault_request = &VaultRequest::OpenVault(OpenVault {
        vault_id: "123456".to_string(),
        auth_pin: "1234".to_string(),
    });
    let sealed_request_bytes =
        &seal_msgpack(vault_request, &enclave_crypto.get_pubkey(), client_crypto).unwrap();

    // Call
    let sealed_response_bytes = &vault_operation_impl(sealed_request_bytes);

    // Unseal
    let unsealed_message: VaultResponse =
        unseal_non_secret_msgpack(sealed_response_bytes, client_crypto).unwrap();

    // Check
    assert_eq!(
        unsealed_message,
        OpenVaultResult::Failed(
            "key_from_id failed for vault_id = \"123456\": Error decoding base32: DecodeError { position: 5, kind: Length }"
                .to_string()
        )
        .into()
    );
}
