use std::io::Error;
use std::path::Path;

use algonaut::indexer::v2::Indexer;
use algonaut::model::indexer::v2::{QueryAssetTransaction, QueryAssetsInfo};
use algonaut_client::Headers;
use anyhow::Context;
use axum::Json;
use ed25519::Signature;
use ring_compat::signature::ed25519::SigningKey;
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::serde_as;
use tokio::try_join;

use crate::errors::AnyhowErrorResponse;
use crate::settings::Settings;
use crate::signer::AuthDataSigner;

/// Request for [`auth_data`]
#[derive(Deserialize)]
pub(crate) struct AuthInput {
    txn_id: String,
    asset_id: u64,
}

/// Response of [`auth_data`]
#[serde_as]
#[derive(Serialize)]
pub(crate) struct SignedAuthData {
    #[serde_as(as = "Base64")]
    auth_data: Vec<u8>,
    #[serde_as(as = "Base64")]
    signature: Signature,
}

#[derive(Serialize)]
pub(crate) struct AuthData {
    redeemed: bool,
    drt_creator: String,
    drt_redeemer: String,
    binary: Vec<u8>,
    data_package: Vec<u8>,
    binary_url: String,
    data_url: String,
}

#[serde_as]
#[derive(Deserialize)]
struct DrtNote {
    #[serde_as(as = "Base64")]
    binary: Vec<u8>,
    binary_url: String,
    #[serde_as(as = "Base64")]
    data_package: Vec<u8>,
    data_url: String,
}

/// Query authorization data for redeemed DRT.
///
/// This will retrieve DRT information from Algorand blockchain, and return signed response.
pub(crate) async fn auth_data(
    Json(auth_input): Json<AuthInput>,
) -> Result<Json<SignedAuthData>, AnyhowErrorResponse> {
    // Read key from PKCS#8 der file
    let pkcs8_bytes = read_file(Path::new("/etc/ntc-oracle/pkcs8.der"))?;
    let signing_key = SigningKey::from_pkcs8(pkcs8_bytes.as_ref())?;

    let auth_data = get_auth_data(auth_input).await?;
    let auth_data_bytes = serde_json::to_vec(&auth_data)?;
    let signed_auth_data = sign_auth_data(auth_data_bytes, signing_key);
    Ok(Json(signed_auth_data))
}

fn sign_auth_data(auth_data: Vec<u8>, signing_key: SigningKey) -> SignedAuthData {
    type RingAuthDataSigner = AuthDataSigner<SigningKey>;

    let signer = RingAuthDataSigner { signing_key };
    let signature = signer.sign(&auth_data);
    println!("Signature: {}", signature);

    SignedAuthData {
        auth_data,
        signature,
    }
}

async fn get_auth_data(auth_input: AuthInput) -> Result<AuthData, AnyhowErrorResponse> {
    let indexer = init_purestake_indexer_client()?;

    let txn_id = auth_input.txn_id;
    let asset_id = auth_input.asset_id;

    let query_assets_info = QueryAssetsInfo::default();
    let query_asset_transaction = QueryAssetTransaction::default();
    let (transaction_info, asset_info, asset_transactions) = try_join!(
        indexer.transaction_info(&txn_id),
        indexer.assets_info(asset_id, &query_assets_info),
        indexer.asset_transactions(asset_id, &query_asset_transaction),
    )?;

    // `redeemed` - check txn receiver is same as reserve address of DRT
    let txn_receiver = transaction_info
        .transaction
        .asset_transfer_transaction
        .context("Fail to get transaction information")?
        .receiver;
    println!("Transaction receiver: {}", txn_receiver);
    let asset_reserve_addr = asset_info
        .asset
        .params
        .reserve
        .context("Fail to get asset reserve address")?;
    println!("Reserve address: {}", asset_reserve_addr);
    let redeemed = txn_receiver == asset_reserve_addr;

    // `drt_creator` - management address of DRT
    let drt_creator = asset_info
        .asset
        .params
        .manager
        .context("Fail to get asset management address")?
        .to_string();
    println!("Manager address: {}", drt_creator);

    // `drt_redeemer`
    let drt_redeemer = transaction_info.transaction.sender;
    println!("Transaction sender: {}", drt_redeemer);

    // Get information encoded in the DRT
    // Find first config transaction for DRT to get Note field
    let config_transaction = asset_transactions.transactions;
    let note_base64 = config_transaction[0]
        .clone()
        .note
        .context("Fail to get note of config transaction for DRT")?;
    let note = base64::decode(note_base64)?;
    let DrtNote {
        binary,
        binary_url,
        data_package,
        data_url,
    } = serde_json::from_slice(note.as_slice())?;

    Ok(AuthData {
        redeemed,
        drt_creator,
        drt_redeemer,
        binary,
        data_package,
        binary_url,
        data_url,
    })
}

fn init_purestake_indexer_client() -> Result<Indexer, AnyhowErrorResponse> {
    // load PureStake configuration settings
    let settings = Settings::new()?;

    // connect v2 indexer client using algonaut
    let indexer_header: Headers = vec![("X-API-Key", settings.purestake.indexer_token.as_str())];
    let indexer = Indexer::with_headers(settings.purestake.indexer_url.as_str(), indexer_header)?;
    Ok(indexer)
}

fn read_file(path: &Path) -> Result<Vec<u8>, Error> {
    use std::io::Read;

    let mut file = std::fs::File::open(path)?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use ring_compat::signature::ed25519::VerifyingKey;

    use super::*;
    use crate::signer::AuthDataVerifier;

    #[test]
    fn test_sign_auth_data_success() {
        // Setup dummy serialized auth data
        let auth_data = vec![7u8; 32];

        // Create signing key
        let seed = &[2u8; 32];
        let signing_key = SigningKey::from_seed(seed).unwrap();

        // Create verifying key and verifier
        let verify_key = signing_key.verify_key();
        type RingAuthDataVerifier = AuthDataVerifier<VerifyingKey>;
        let verifier = RingAuthDataVerifier { verify_key };

        // Run sign_auth_data function
        let signed_auth_data = sign_auth_data(auth_data.clone(), signing_key);

        // Asserts
        assert_eq!(auth_data, signed_auth_data.auth_data);
        assert!(verifier
            .verify(&auth_data, &signed_auth_data.signature)
            .is_ok());
    }
}
