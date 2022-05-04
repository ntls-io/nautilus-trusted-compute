use crate::algorand::*;

#[cfg(not(test))]
use crate::algorand::{Account, Algod};
#[cfg(test)]
use crate::algorand::{MockAccount as Account, MockAlgod as Algod};

use algonaut::transaction::TransactionType;
use serde::Serialize;
use serde_with::{base64::Base64, serde_as};
use thiserror::Error;
use url::Url;

/// DRT configuration settings, ready for submission to the blockchain network
/// to create a new DRT.
#[derive(Debug)]
pub struct DrtConfig {
    creator: Address,
    encoder: AsaNote,
    txn_type: TransactionType,
}

impl DrtConfig {
    pub async fn submit(
        self,
        algod: &Algod,
        sender: &Account,
    ) -> Result<TransactionResponse, SubmitError> {
        self.check_addr_match(sender)?;
        let Self {
            encoder, txn_type, ..
        } = self;

        let suggested_txn_params = algod.suggested_transaction_params().await?;
        let txn = TxnBuilder::with(&suggested_txn_params, txn_type)
            .note(encoder.encode_drt()?)
            .build()?;

        let signed_txn = sender.sign_transaction(txn)?;
        Ok(algod.broadcast_signed_transaction(&signed_txn).await?)
    }

    fn check_addr_match(&self, sender: &Account) -> Result<(), SubmitError> {
        let creator = self.creator;
        let sender = sender.address();
        if sender != creator {
            return Err(SubmitError::AddressMismatch { creator, sender });
        }
        Ok(())
    }
}

/// Contents of the note field of the initial transaction that created a DRT
#[serde_as]
#[derive(Clone, Debug, Serialize)]
pub struct AsaNote {
    #[serde_as(as = "Base64")]
    pub binary: Vec<u8>,
    pub binary_url: String,
    #[serde_as(as = "Base64")]
    pub data_package: Vec<u8>,
    pub data_url: String,
}

impl AsaNote {
    fn encode_drt(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }
}

/// A builder used to spawn new DRT configurations.
#[derive(Clone, Debug)]
pub struct DrtConfigBuilder {
    creator: [u8; 32],
    encoder: AsaNote,
    name: String,
    supply: u64,
    url: Option<String>,
    meta_data_hash: Option<Vec<u8>>,
}

impl DrtConfigBuilder {
    pub fn new(creator: [u8; 32], encoder: AsaNote) -> Self {
        Self {
            creator,
            encoder,
            name: String::from("Digital Rights Token"),
            supply: u64::MAX,
            url: None,
            meta_data_hash: None,
        }
    }
    pub fn name(self, new_name: &str) -> Self {
        Self {
            name: String::from(new_name),
            ..self
        }
    }
    pub fn meta_data_hash(self, new_hash: &[u8]) -> Self {
        Self {
            meta_data_hash: Some(Vec::from(new_hash)),
            ..self
        }
    }
    pub fn url(self, new_url: &str) -> Result<Self, url::ParseError> {
        let _try_parse = Url::parse(new_url)?;
        Ok(Self {
            url: Some(String::from(new_url)),
            ..self
        })
    }
    pub fn supply(self, new_supply: u64) -> Self {
        Self {
            supply: new_supply,
            ..self
        }
    }
    pub fn build(self) -> DrtConfig {
        let Self {
            creator,
            encoder,
            name,
            supply,
            url,
            meta_data_hash,
        } = self;

        //TODO(validation): add checks for limits set by Algorand

        let creator = Address(creator);
        let create_asset = CreateAsset::new(creator, supply, 0, false)
            .unit_name(String::from("DRT"))
            .asset_name(name)
            .manager(creator)
            .reserve(creator)
            .freeze(creator)
            .clawback(creator);

        let txn_type = match (url, meta_data_hash) {
            (Some(text), Some(hash)) => create_asset.url(text).meta_data_hash(hash),
            (Some(text), None) => create_asset.url(text),
            (None, Some(hash)) => create_asset.meta_data_hash(hash),
            _ => create_asset,
        }
        .build();

        DrtConfig {
            creator,
            encoder,
            txn_type,
        }
    }
}

#[derive(Debug, Error)]
pub enum SubmitError {
    #[error(
        "sender address does not match that of the DRT creator: expected \
        {creator:?}, but found {sender:?} instead"
    )]
    AddressMismatch { creator: Address, sender: Address },
    #[error("failed to sign transaction: {0:?}")]
    SigningError(#[from] TransactionError),
    #[error("service error: {0:?}")]
    ServiceError(#[from] ServiceError),
    #[error("failed to serialize DRT note parameters as JSON: {0:?}")]
    SerializeError(#[from] serde_json::Error),
}

// TODO(JP): Add all validation checks and corresponding errors.  Once done,
// remove the `non_exhaustive` attribute below.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ValidationError {
    #[error("failed to parse URL: {0:?}")]
    InvalidUrl(#[from] url::ParseError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorand::{MockAccount as Account, MockAlgod as Algod};

    use algonaut::core::{MicroAlgos, Round, SuggestedTransactionParams};
    use algonaut::crypto::{HashDigest, Signature};
    use algonaut::transaction::transaction::Payment;

    const CREATOR_ADDRESS: [u8; 32] = [1u8; 32];

    fn build_drt_config() -> DrtConfig {
        let note = AsaNote {
            binary: [1u8; 32].to_vec(),
            binary_url: String::from("https://host1.example.com"),
            data_package: [2u8; 32].to_vec(),
            data_url: String::from("https://host2.example.com"),
        };

        let builder = DrtConfigBuilder::new(CREATOR_ADDRESS, note)
            .name("DRT")
            .meta_data_hash([5u8; 32].as_slice())
            .url("https://drt.example.com")
            .unwrap()
            .supply(10);
        builder.build()
    }

    fn get_suggested_txn_params() -> SuggestedTransactionParams {
        SuggestedTransactionParams {
            genesis_id: String::from("id"),
            genesis_hash: HashDigest([7u8; 32]),
            consensus_version: String::from("v2"),
            fee_per_byte: MicroAlgos(2u64),
            min_fee: MicroAlgos(1u64),
            first_valid: Round(10),
            last_valid: Round(20),
        }
    }

    fn get_test_transaction() -> Transaction {
        let payment = Payment {
            sender: Address(CREATOR_ADDRESS),
            receiver: Address([2u8; 32]),
            amount: MicroAlgos(3u64),
            close_remainder_to: None,
        };
        TxnBuilder::new(
            algonaut::transaction::builder::TxnFee::Fixed(MicroAlgos(4u64)),
            Round(10),
            Round(20),
            HashDigest([7u8; 32]),
            algonaut::transaction::TransactionType::Payment(payment),
        )
        .build()
        .unwrap()
    }

    fn signed_transaction(txn: &Transaction) -> SignedTransaction {
        SignedTransaction {
            transaction: txn.clone(),
            transaction_id: String::from("id"),
            sig: algonaut::transaction::transaction::TransactionSignature::Single(Signature(
                [6u8; 64],
            )),
        }
    }

    #[tokio::test]
    async fn submit_drt_success() {
        let drt_config = build_drt_config();

        let mut mock_account = Account::new();
        let mut mock_algod = Algod::new();

        let txn = get_test_transaction();
        mock_account
            .expect_address()
            .once()
            .return_const(Address(CREATOR_ADDRESS));
        mock_algod
            .expect_suggested_transaction_params()
            .once()
            .return_once(|| Ok(get_suggested_txn_params()));

        let signed_txn = signed_transaction(&txn);
        mock_account
            .expect_sign_transaction()
            .once()
            .return_once(|_| Ok(signed_txn));

        mock_algod
            .expect_broadcast_signed_transaction()
            .once()
            .withf(|t| *t == signed_transaction(&get_test_transaction()))
            .return_once(|_| {
                let tx_id = String::from("id");
                Ok(TransactionResponse { tx_id })
            });

        let response = drt_config.submit(&mock_algod, &mock_account).await.unwrap();
        assert!(matches!(response, TransactionResponse { .. }));
    }
}
