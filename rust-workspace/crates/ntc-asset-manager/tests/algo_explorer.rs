use dotenv_codegen::dotenv;
use ntc_asset_manager::{
    algorand::Address,
    algorand::{Account, Algod, AlgonautAlgod},
    drt::AsaNote,
    drt::DrtConfigBuilder,
    secret::Secret,
};
use std::str::FromStr;

static ALGO_ENDPOINT: &str = "https://node.testnet.algoexplorerapi.io/";
static ALGO_API_TOKEN: [char; 64] = ['a'; 64];
static ALGORAND_MNEMONIC: &str = dotenv!("ALGORAND_MNEMONIC");

#[tokio::test]
async fn create_new_drt() {
    let api_token: String = ALGO_API_TOKEN.iter().collect();
    let algod = Algod(AlgonautAlgod::new(ALGO_ENDPOINT, &api_token).unwrap());
    let secret = Secret::from_str(ALGORAND_MNEMONIC).unwrap();
    let account = Account::from_secret(secret).unwrap();

    let note = AsaNote {
        binary: [1u8; 32].to_vec(),
        binary_url: String::from("https://host1.example"),
        data_package: [2u8; 32].to_vec(),
        data_url: String::from("https://host2.example"),
    };

    let Address(public_key) = account.address();
    let config = DrtConfigBuilder::new(public_key, note)
        .name("DRT")
        .supply(10)
        .url("https://drt.example.com")
        .unwrap()
        .build();
    config.submit(&algod, &account).await.unwrap();
}
