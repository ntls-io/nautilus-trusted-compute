//! Structures representing various entities.

use std::prelude::v1::{String, ToString};

use algonaut::transaction::account::Account as AlgonautAccount;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::schema::types::{
    AlgorandAccountSeedBytes,
    AlgorandAddressBase32,
    AlgorandAddressBytes,
    VaultId,
    VaultPassword,
};

/// A Nautilus vault's basic displayable details.
///
/// This is what gets sent to clients.
#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
pub struct VaultDisplay {
    pub vault_id: VaultId,
    pub username: String,

    // TODO(Pi): Decouple for multiple accounts per vault.
    pub algorand_address_base32: AlgorandAddressBase32,
}

impl From<VaultStorable> for VaultDisplay {
    fn from(storable: VaultStorable) -> Self {
        Self {
            vault_id: storable.vault_id.clone(),
            username: storable.username.clone(),

            algorand_address_base32: storable.algorand_account.address_base32(),
        }
    }
}

/// A Nautilus vault's full details.
///
/// This is everything that gets persisted in the vault store.
#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
#[derive(Zeroize, ZeroizeOnDrop)] // zeroize
pub struct VaultStorable {
    pub vault_id: VaultId,
    pub auth_password: VaultPassword,

    pub username: String,

    pub algorand_account: AlgorandAccount,
}

// Algorand entities:

/// An Algorand account.
#[derive(Clone, Eq, PartialEq, Debug)] // core
#[derive(Deserialize, Serialize)] // serde
#[derive(Zeroize, ZeroizeOnDrop)] // zeroize
pub struct AlgorandAccount {
    pub seed_bytes: AlgorandAccountSeedBytes,
}

impl AlgorandAccount {
    pub(crate) fn generate() -> Self {
        Self {
            // XXX performance: Or just use OsRng directly?
            seed_bytes: AlgonautAccount::generate().seed(),
        }
    }

    // XXX performance: Repeated temporary conversions through AlgonautAccount?
    pub(crate) fn as_algonaut_account(&self) -> AlgonautAccount {
        AlgonautAccount::from_seed(self.seed_bytes)
    }

    pub fn address_bytes(&self) -> AlgorandAddressBytes {
        self.as_algonaut_account().address().0
    }

    pub fn address_base32(&self) -> AlgorandAddressBase32 {
        self.as_algonaut_account().address().to_string()
    }
}

impl From<AlgorandAccount> for AlgonautAccount {
    fn from(account: AlgorandAccount) -> Self {
        account.as_algonaut_account()
    }
}
