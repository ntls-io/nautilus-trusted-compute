//! Standalone binary that exposes HTTP interface that reads information from Algorand and
//! signs the resulting authorization data

mod helpers;
mod settings;

use algonaut::core::Round;
use algonaut::indexer::v2::Indexer;
use algonaut_client::Headers;
use axum::{routing::get, Router};
use settings::Settings;
use std::error::Error;

use crate::helpers::bind_addr_from_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bind_addr = bind_addr_from_env()?;

    // load configuration settings
    let settings = Settings::new().unwrap();
    println!("{:?}", settings);

    let indexer_header: Headers = vec![("X-API-Key", settings.purestake.indexer_token.as_str())];
    let indexer =
        Indexer::with_headers(settings.purestake.indexer_url.as_str(), indexer_header).unwrap();

    let block = indexer.block(Round(6)).await.unwrap();
    println!("Previous Block Hash: {}", block.previous_block_hash);

    let axum_app = Router::new().route("/auth_data", get(|| async { "Hello, world!" }));
    let axum_server = axum::Server::bind(&bind_addr).serve(axum_app.into_make_service());
    println!("listening on http://{}", bind_addr);
    axum_server.await?;

    Ok(())
}
