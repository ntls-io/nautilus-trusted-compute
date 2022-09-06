use std::prelude::v1::ToString;

use sgx_vault_impl::schema::actions;
use sgx_vault_impl::schema::actions::{TransactionSigned, TransactionToSign};
use sgx_vault_impl::vault_operations::sign_transaction::sign_transaction;

use crate::helpers::vault_store::create_test_vault;

pub(crate) fn sign_transaction_empty() {
    let existing = &create_test_vault();

    let transaction_bytes = Default::default();

    let request = &actions::SignTransaction {
        vault_id: existing.vault_id.clone(),
        auth_pin: "123456".to_string(),
        transaction_to_sign: TransactionToSign::XrplTransaction { transaction_bytes },
    };
    let signed = sign_transaction(request).unwrap_signed();

    match signed {
        TransactionSigned::XrplTransactionSigned {
            signed_transaction_bytes: _,
            signature_bytes,
        } => {
            assert_eq!(signature_bytes[0], 0x30); // DER tag for SEQUENCE
        }
        otherwise => panic!("unexpected: {:?}", otherwise),
    }

    // TODO(Pi): Test more substantially.
}
