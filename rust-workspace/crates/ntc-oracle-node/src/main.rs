//Standalone binary that exposes HTTP interface that reads information from Algorand and signs the resulting authorization data

use algonaut::core::Round;
use algonaut::indexer::v2::Indexer;
use algonaut_client::Headers;
use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let indexer_url = &dotenv::var("INDEXER_URL").unwrap();
    let indexer_api_token = &dotenv::var("INDEXER_API_TOKEN").unwrap();

    let indexer_header: Headers = vec![("X-API-Key", indexer_api_token)];
    let indexer = Indexer::with_headers(indexer_url, indexer_header).unwrap();

    let block = indexer.block(Round(6)).await.unwrap();
    println!("Previous Block Hash: {}", block.previous_block_hash);
    Ok(())
}
