use std::error::Error;
use std::prelude::v1::Box;

use secrecy::{ExposeSecret, Secret};

use crate::ported::crypto::SecretBytes;
use crate::schema::actions::{VaultRequest, VaultResponse};
use crate::schema::msgpack::{FromMessagePack, ToMessagePack};
use crate::schema::sealing::{seal_from_enclave, unseal_to_enclave, SealedMessage};
use crate::vault_operations::create_vault::create_vault;
use crate::vault_operations::errors;
use crate::vault_operations::open_vault::open_vault;
use crate::vault_operations::sign_transaction::sign_transaction;

/// Implementation for [`crate::ecalls::vault_operation::vault_operation`].
///
/// This processes an exchange of the following:
///
/// Request: [`SealedMessage`] of [`VaultRequest`]
///
/// Response: [`SealedMessage`] of [`VaultResponse`]
///
pub fn vault_operation_impl(sealed_request_bytes: &[u8]) -> Box<[u8]> {
    match vault_operation_impl_sealing(sealed_request_bytes) {
        Ok(sealed_response_bytes) => sealed_response_bytes,
        Err(error) => panic!("{}", error), // FIXME: better reporting
    }
}

/// Handle sealing and unsealing the exchange.
fn vault_operation_impl_sealing(sealed_request_bytes: &[u8]) -> Result<Box<[u8]>, Box<dyn Error>> {
    // Unseal request
    let sealed_request = &SealedMessage::from_msgpack(sealed_request_bytes).map_err(|err| {
        errors::message_with_base64(
            "vault_operation_impl_sealing",
            "failed to unpack received sealed request",
            err,
            "sealed request msgpack",
            sealed_request_bytes,
        )
    })?;
    let request_bytes = &unseal_to_enclave(sealed_request).map_err(|err| {
        errors::message_with_debug_value(
            "vault_operation_impl_sealing",
            "failed to unseal request",
            err,
            "sealed request",
            sealed_request,
        )
    })?;
    let vault_request = &Secret::new(
        VaultRequest::from_msgpack(request_bytes.expose_secret()).map_err(|err| {
            errors::message_with_base64(
                "vault_operation_impl_sealing",
                "invalid VaultReq",
                err,
                "unsealed VaultRequest msgpack",
                request_bytes.expose_secret(),
            )
        })?,
    );

    // Dispatch
    let vault_response = vault_operation_impl_dispatch(vault_request.expose_secret());

    // Seal response
    let response_bytes = &SecretBytes::new(vault_response.to_msgpack().map_err(|err| {
        errors::message_with_debug_value(
            "vault_operation_impl_sealing",
            "failed to msgpack VaultResponse-to-seal",
            err,
            "unsealed VaultResponse",
            vault_response,
        )
    })?);
    let sealed_response = seal_from_enclave(response_bytes, &sealed_request.sender_public_key)
        .map_err(|err| {
            errors::message_with_base64(
                "vault_operation_impl_sealing",
                "failed to seal packed VaultResponse",
                err,
                "unsealed VaultResponse msgpack",
                response_bytes.expose_secret(),
            )
        })?;
    let sealed_response_bytes = sealed_response.to_msgpack().map_err(|err| {
        errors::message_with_debug_value(
            "vault_operation_impl_sealing",
            "failed to msgpack sealed VaultRequest",
            err,
            "sealed response",
            sealed_response,
        )
    })?;
    Ok(sealed_response_bytes)
}

/// Handle dispatching the exchange.
fn vault_operation_impl_dispatch(vault_request: &VaultRequest) -> VaultResponse {
    if cfg!(feature = "verbose-debug-logging") {
        println!(
            "DEBUG: vault_operation_impl_dispatch: dispatching vault_request = \n{:#?}",
            vault_request
        );
    }
    match vault_request {
        VaultRequest::CreateVault(request) => create_vault(request).into(),
        VaultRequest::OpenVault(request) => open_vault(request).into(),
        VaultRequest::SignTransaction(request) => sign_transaction(request).into(),
    }
}
