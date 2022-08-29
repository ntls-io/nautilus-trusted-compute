//! Core request / response message types.
//!
//! # Related
//!
//! * <https://developer.algorand.org/docs/reference/rest-apis/kmd/>

use std::io;
use std::prelude::v1::{String, ToString};

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::schema::entities::VaultDisplay;
use crate::schema::types::{Bytes, VaultId, VaultPin};
use crate::vault_operations::store::UnlockVaultError;

#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
#[derive(Zeroize, ZeroizeOnDrop)] // zeroize
pub struct CreateVault {
    pub owner_name: String,
    pub auth_pin: VaultPin,
    pub phone_number: Option<String>,
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
    pub auth_pin: VaultPin,
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
            InvalidAuthPin => Self::InvalidAuth,
            IoError(err) => Self::Failed(err.to_string()),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
#[derive(Zeroize, ZeroizeOnDrop)] // zeroize
pub struct SignTransaction {
    pub vault_id: VaultId,
    pub auth_pin: VaultPin,

    #[zeroize(skip)]
    pub transaction_to_sign: TransactionToSign,
}

impl From<UnlockVaultError> for SignTransactionResult {
    fn from(err: UnlockVaultError) -> Self {
        use UnlockVaultError::*;
        match err {
            InvalidVaultId => Self::InvalidAuth,
            InvalidAuthPin => Self::InvalidAuth,
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

    /// An unsigned XRPL transaction.
    XrplTransaction {
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

    /// A signed XRPL transaction.
    XrplTransactionSigned {
        #[serde(with = "serde_bytes")]
        signed_transaction_bytes: Bytes,

        #[serde(with = "serde_bytes")]
        signature_bytes: Bytes,
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
            otherwise => panic!(
                "called `TransactionSigned::unwrap_algorand_bytes` on: {:?}",
                otherwise
            ),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub struct SaveOnfidoCheck {
    pub vault_id: VaultId,
    pub auth_pin: VaultPin,

    pub check: OnfidoCheckResult,
}

#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub enum SaveOnfidoCheckResult {
    Saved,
    InvalidAuth,
    Failed(String),
}

impl From<io::Error> for SaveOnfidoCheckResult {
    fn from(err: io::Error) -> Self {
        Self::Failed(err.to_string())
    }
}

impl From<UnlockVaultError> for SaveOnfidoCheckResult {
    fn from(err: UnlockVaultError) -> Self {
        use UnlockVaultError::*;
        match err {
            InvalidVaultId => Self::InvalidAuth,
            InvalidAuthPin => Self::InvalidAuth,
            IoError(err) => Self::from(err),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub struct LoadOnfidoCheck {
    pub vault_id: VaultId,
    pub auth_pin: VaultPin,
}

#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub enum LoadOnfidoCheckResult {
    Loaded(OnfidoCheckResult),
    NotFound,
    InvalidAuth,
    Failed(String),
}

impl From<io::Error> for LoadOnfidoCheckResult {
    fn from(err: io::Error) -> Self {
        Self::Failed(err.to_string())
    }
}

impl From<UnlockVaultError> for LoadOnfidoCheckResult {
    fn from(err: UnlockVaultError) -> Self {
        use UnlockVaultError::*;
        match err {
            InvalidVaultId => Self::InvalidAuth,
            InvalidAuthPin => Self::InvalidAuth,
            IoError(err) => Self::from(err),
        }
    }
}

/// Docs: https://documentation.onfido.com/v2/#report-object
#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub struct OnfidoCheckResult {
    pub id: String,

    pub href: String,

    /// Docs: <https://documentation.onfido.com/v2/#report-results>
    pub result: String,

    /// Docs: <https://documentation.onfido.com/v2/#sub-results-document-reports>
    pub sub_result: Option<String>,
}

/// Dispatching enum for action requests.
#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
#[derive(Zeroize, ZeroizeOnDrop)] // zeroize
pub enum VaultRequest {
    CreateVault(CreateVault),
    OpenVault(OpenVault),
    SignTransaction(SignTransaction),

    #[zeroize(skip)]
    SaveOnfidoCheck(SaveOnfidoCheck),

    #[zeroize(skip)]
    LoadOnfidoCheck(LoadOnfidoCheck),
}

/// Dispatching enum for action results.
#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub enum VaultResponse {
    CreateVault(CreateVaultResult),
    OpenVault(OpenVaultResult),
    SignTransaction(SignTransactionResult),
    SaveOnfidoCheck(SaveOnfidoCheckResult),
    LoadOnfidoCheck(LoadOnfidoCheckResult),
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

impl From<SaveOnfidoCheckResult> for VaultResponse {
    fn from(result: SaveOnfidoCheckResult) -> Self {
        Self::SaveOnfidoCheck(result)
    }
}

impl From<LoadOnfidoCheckResult> for VaultResponse {
    fn from(result: LoadOnfidoCheckResult) -> Self {
        Self::LoadOnfidoCheck(result)
    }
}