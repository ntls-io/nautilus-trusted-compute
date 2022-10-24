use mongodb::{options::ClientOptions, Client};
use actix_web::{web, App, HttpServer};
use std::env;
use std::sync::*;

mod logs_handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Create log for actix to output errors
    env::set_var("RUST_LOG", "actix_web=debug");
    let mongo_url = env::var("CONNECTION_STRING_LOGS").unwrap();
    let mut client_options = ClientOptions::parse(&mongo_url).await.unwrap();
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(web::scope("/api").configure(logs_handlers::scoped_config))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

// Upload data pool API //
// Encryt from client to enclave - seal using enclave key - upload to Azure Blob (or Cosmos DB)
// Data is uploaded in JSON format
// Encrypt: Use SSL or TLS




// Upload to Azure Blob storage or Cosmos DB - TBD //
// https://github.com/Azure/azure-sdk-for-rust
// https://crates.io/crates/azure_storage_blobs





// Query data API //
// Frontend must show which data packages the data creator has uploaded to the system
// Option 1 - Query data service for list of data packages uploaded by the data creator
// Option 2 - Lookup smart contracts associated with data creator's NTLS wallet