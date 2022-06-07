//! Standalone binary that exposes HTTP interface that reads information from Algorand and
//! signs the resulting authorization data

mod helpers;
mod settings;

use algonaut::core::Address;
use algonaut::crypto::Signature;
use algonaut::indexer::v2::Indexer;
use algonaut::model::indexer::v2::{QueryAssetTransaction, QueryAssetsInfo};
use algonaut_client::Headers;
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::error::Error;
use std::str::FromStr;

use crate::helpers::bind_addr_from_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bind_addr = bind_addr_from_env()?;

    let axum_app = Router::new().route("/auth_data", get(auth_data));
    let axum_server = axum::Server::bind(&bind_addr).serve(axum_app.into_make_service());
    println!("listening on http://{}", bind_addr);
    axum_server.await?;

    Ok(())
}

async fn auth_data(Json(payload): Json<AuthInput>) -> impl IntoResponse {
    let auth_data = sign_auth_data(get_auth_data(payload).await);
    (StatusCode::OK, Json(auth_data))
}

async fn get_auth_data(auth_input: AuthInput) -> AuthData {
    // load PureStake configuration settings
    let settings = Settings::new().unwrap();
    println!("{:?}", settings);

    // connect v2 indexer client using algonaut
    let indexer_header: Headers = vec![("X-API-Key", settings.purestake.indexer_token.as_str())];
    let indexer = Indexer::with_headers(settings.purestake.indexer_url.as_str(), indexer_header)
        .expect("Error connecting to PureStake v2 indexer");

    // redeem txn id = OZGIL3VY4TFAC5AOBLSG2WTRYSG5ZC5LCZWQQGEVQUAFCIDLMIOA
    let txn_id = auth_input.txn_id;
    // 93352061
    let asset_id = auth_input.asset_id;

    let transaction_info = indexer
        .transaction_info(&txn_id)
        .await
        .expect("Incorrect Transaction ID");

    let asset_info = indexer
        .assets_info(asset_id, &QueryAssetsInfo { include_all: None })
        .await
        .expect("Incorrect Asset ID");

    // `redeemed` - check destination address is same as reserve address of the DRT
    // check txn receiver is same as reserve address on DRT
    let txn_receiver = transaction_info
        .transaction
        .asset_transfer_transaction
        .unwrap()
        .receiver;
    println!("Transaction receiver: {}", txn_receiver);
    let asset_reserve_addr = asset_info.asset.params.reserve.unwrap();
    println!("Reserve address: {}", asset_reserve_addr);

    let redeemed = txn_receiver == asset_reserve_addr;
    let drt_creator = asset_info.asset.params.manager.unwrap();
    println!("Manager address: {}", drt_creator);

    let txn_sender = transaction_info.transaction.sender;
    println!("Transaction sender: {}", txn_sender);
    let drt_redeemer = Address::from_str(txn_sender.as_str()).unwrap();

    // Get encoded information in the DRT
    // find first config transaction for DRT to get Note field
    let asset_transactions = indexer
        .asset_transactions(asset_id, &QueryAssetTransaction::default())
        .await
        .expect("Incorrect Asset Transactions ID");

    let config_transaction = asset_transactions.transactions;
    let note_base64 = config_transaction[0].clone().note.unwrap();
    println!("Note_Base64: {}", note_base64);
    let note = base64::decode(note_base64).unwrap();
    println!("Note: {:?}", note);
    let note_drtnote: Result<DrtNote, _> = serde_json::from_slice(note.as_slice());
    let DrtNote {
        binary,
        binary_url,
        data_package,
        data_url,
    } = note_drtnote.unwrap();

    AuthData {
        redeemed,
        drt_creator,
        drt_redeemer,
        binary,
        data_package,
        binary_url,
        data_url,
    }
}

fn sign_auth_data(auth_data: AuthData) -> SignedAuthData {
    SignedAuthData {
        auth_data,
        signature: Signature([1; 64]),
    }
}

#[derive(Deserialize)]
struct DrtNote {
    binary: Vec<u8>,
    binary_url: String,
    data_package: Vec<u8>,
    data_url: String,
}

#[derive(Deserialize)]
struct AuthInput {
    txn_id: String,
    asset_id: u64,
}

#[derive(Serialize)]
struct AuthData {
    redeemed: bool,
    drt_creator: Address,
    drt_redeemer: Address,
    binary: Vec<u8>,
    data_package: Vec<u8>,
    binary_url: String,
    data_url: String,
}

#[derive(Serialize)]
struct SignedAuthData {
    auth_data: AuthData,
    signature: Signature,
}
