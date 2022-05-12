//! Standalone binary that exposes HTTP interface that reads information from Algorand and
//! signs the resulting authorization data

mod settings;

use algonaut::core::Round;
use algonaut::indexer::v2::Indexer;
use algonaut_client::Headers;
use settings::Settings;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load configuration settings
    let settings = Settings::new().unwrap();
    println!("{:?}", settings);

    let indexer_header: Headers = vec![("X-API-Key", settings.purestake.indexer_token.as_str())];
    let indexer =
        Indexer::with_headers(settings.purestake.indexer_url.as_str(), indexer_header).unwrap();

    let block = indexer.block(Round(6)).await.unwrap();
    println!("Previous Block Hash: {}", block.previous_block_hash);
    Ok(())
}
