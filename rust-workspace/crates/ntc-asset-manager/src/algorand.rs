use crate::secret::Secret;

use algonaut::core::SuggestedTransactionParams;

#[cfg(test)]
use mockall::automock;

pub use algonaut::{
    algod::v2::Algod as AlgonautAlgod,
    core::Address,
    error::ServiceError,
    model::algod::v2::TransactionResponse,
    transaction::{
        account::Account as AlgonautAccount, error::TransactionError, SignedTransaction,
        Transaction,
    },
};

pub(crate) use algonaut::transaction::{CreateAsset, TxnBuilder};

/// A blockchain account that is used to sign transactions.
#[derive(Debug, Eq, PartialEq)]
pub struct Account(AlgonautAccount);

/// A handle on a stateful connection with a specified node on the blockchain.
#[derive(Debug)]
pub struct Algod(pub AlgonautAlgod);

#[cfg_attr(test, automock)]
impl Account {
    pub fn from_secret(secret: Secret) -> Result<Self, TransactionError> {
        Ok(Account(match secret {
            Secret::Mnemonic(text) => AlgonautAccount::from_mnemonic(&text)?,
            Secret::Seed(seed) => AlgonautAccount::from_seed(seed),
        }))
    }
    pub fn address(&self) -> Address {
        let Account(algonaut_account) = self;
        algonaut_account.address()
    }
    pub fn sign_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<SignedTransaction, TransactionError> {
        let Account(algonaut_account) = self;
        algonaut_account.sign_transaction(transaction)
    }
}

#[cfg_attr(test, automock)]
impl Algod {
    pub async fn broadcast_signed_transaction(
        &self,
        txn: &SignedTransaction,
    ) -> Result<TransactionResponse, ServiceError> {
        let Algod(algonaut_algod) = self;
        algonaut_algod.broadcast_signed_transaction(txn).await
    }
    pub async fn suggested_transaction_params(
        &self,
    ) -> Result<SuggestedTransactionParams, ServiceError> {
        let Algod(algonaut_algod) = self;
        algonaut_algod.suggested_transaction_params().await
    }
}

impl TryFrom<Secret> for Account {
    type Error = TransactionError;
    fn try_from(secret: Secret) -> Result<Self, TransactionError> {
        Account::from_secret(secret)
    }
}
impl From<AlgonautAccount> for Account {
    fn from(algonaut_account: AlgonautAccount) -> Self {
        Account(algonaut_account)
    }
}
impl From<Account> for AlgonautAccount {
    fn from(account: Account) -> Self {
        let Account(algonaut_account) = account;
        algonaut_account
    }
}
