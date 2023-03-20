//! Core request / response message types.
//!
//! # Related
//!
//! * <https://developer.algorand.org/docs/reference/rest-apis/kmd/>

use std::prelude::v1::{String, ToString};

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::schema::entities::VaultDisplay;
use crate::schema::types::{Bytes, VaultId, VaultPassword};
use crate::vault_operations::store::UnlockVaultError;

#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
#[derive(Zeroize, ZeroizeOnDrop)] // zeroize
pub struct CreateVault {
    pub username: String,
    pub auth_password: VaultPassword,
}

#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub enum CreateVaultResult {
    Created(VaultDisplay),
    Failed(String),
}

#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
#[derive(Zeroize, ZeroizeOnDrop)] // zeroize
pub struct OpenVault {
    pub vault_id: VaultId,
    pub auth_password: VaultPassword,
}

#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub enum OpenVaultResult {
    Opened(VaultDisplay),
    InvalidAuth,
    Failed(String),
}

impl From<UnlockVaultError> for OpenVaultResult {
    fn from(err: UnlockVaultError) -> Self {
        use UnlockVaultError::*;
        match err {
            InvalidVaultId => Self::InvalidAuth,
            InvalidAuthPassword => Self::InvalidAuth,
            IoError(err) => Self::Failed(err.to_string()),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
#[derive(Zeroize, ZeroizeOnDrop)] // zeroize
pub struct SignTransaction {
    pub vault_id: VaultId,
    pub auth_password: VaultPassword,

    #[zeroize(skip)]
    pub transaction_to_sign: TransactionToSign,
}

impl From<UnlockVaultError> for SignTransactionResult {
    fn from(err: UnlockVaultError) -> Self {
        use UnlockVaultError::*;
        match err {
            InvalidVaultId => Self::InvalidAuth,
            InvalidAuthPassword => Self::InvalidAuth,
            IoError(err) => Self::Failed(err.to_string()),
        }
    }
}

/// For [`SignTransaction`]: A choice of type of transaction to sign.
#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub enum TransactionToSign {
    /// An unsigned Algorand transaction.
    AlgorandTransaction {
        #[serde(with = "serde_bytes")]
        transaction_bytes: Bytes,
    },
}

#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub enum SignTransactionResult {
    Signed(TransactionSigned),
    InvalidAuth,
    Failed(String),
}

impl SignTransactionResult {
    /// Unwrap [`Self::Signed`] or panic.
    pub fn unwrap_signed(self) -> TransactionSigned {
        match self {
            SignTransactionResult::Signed(signed) => signed,
            otherwise => panic!(
                "called `SignTransactionResult::unwrap_signed` on: {:?}",
                otherwise
            ),
        }
    }
}

/// For [`SignTransactionResult`]: The possible types of signed transactions.
#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub enum TransactionSigned {
    /// A signed Algorand transaction.
    AlgorandTransactionSigned {
        #[serde(with = "serde_bytes")]
        signed_transaction_bytes: Bytes,
    },
}

impl TransactionSigned {
    /// Create [`Self::AlgorandTransactionSigned`] from bytes.
    pub fn from_algorand_bytes(signed_transaction_bytes: Bytes) -> Self {
        Self::AlgorandTransactionSigned {
            signed_transaction_bytes,
        }
    }

    /// Unwrap [`Self::AlgorandTransactionSigned`] or panic.
    pub fn unwrap_algorand_bytes(self) -> Bytes {
        match self {
            TransactionSigned::AlgorandTransactionSigned {
                signed_transaction_bytes,
            } => signed_transaction_bytes,
        }
    }
}

/// Dispatching enum for action requests.
#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
#[derive(Zeroize, ZeroizeOnDrop)] // zeroize
pub enum VaultRequest {
    CreateVault(CreateVault),
    OpenVault(OpenVault),
    SignTransaction(SignTransaction),
}

/// Dispatching enum for action results.
#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub enum VaultResponse {
    CreateVault(CreateVaultResult),
    OpenVault(OpenVaultResult),
    SignTransaction(SignTransactionResult),
}

// Convenience conversions:

impl From<CreateVaultResult> for VaultResponse {
    fn from(result: CreateVaultResult) -> Self {
        Self::CreateVault(result)
    }
}

impl From<OpenVaultResult> for VaultResponse {
    fn from(result: OpenVaultResult) -> Self {
        Self::OpenVault(result)
    }
}

impl From<SignTransactionResult> for VaultResponse {
    fn from(result: SignTransactionResult) -> Self {
        Self::SignTransaction(result)
    }
}
