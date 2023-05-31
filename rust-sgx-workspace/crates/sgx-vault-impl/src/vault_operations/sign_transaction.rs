//! Implement [`SignTransaction`].

use std::prelude::v1::String;

use crate::schema::actions::{
    SignTransaction,
    SignTransactionResult,
    TransactionSigned,
    TransactionToSign,
};
use crate::vault_operations::sign_transaction_algorand::sign_algorand;
use crate::vault_operations::store::unlock_vault;

pub fn sign_transaction(request: &SignTransaction) -> SignTransactionResult {
    let stored = match unlock_vault(&request.vault_id, &request.auth_password) {
        Ok(stored) => stored,
        Err(err) => return err.into(),
    };

    let sign_result: Result<TransactionSigned, String> = match &request.transaction_to_sign {
        TransactionToSign::AlgorandTransaction { transaction_bytes } => {
            sign_algorand(&stored.algorand_account, transaction_bytes)
                .map(TransactionSigned::from_algorand_bytes)
        }
    };

    println!("Sign Result!!");
    println!("{:?}", sign_result);
    // `Result` → `SignTransactionResult`
    match sign_result {
        Ok(signed) => SignTransactionResult::Signed(signed),
        Err(message) => SignTransactionResult::Failed(message),
    }
}
