use bson::{doc, Bson, Document};
use mongodb::{error::Error, options::ClientOptions, Client, Collection, Database};
use actix_web::{get, web, App, HttpServer, Responder};
use std::env;
use std::sync::*;

mod logs_handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Create log for actix to output errors
    env::set_var("RUST_LOG", "actix_web=debug");
    //Will remove connection string for production - included for testing
    let mut client_options = ClientOptions::parse("mongodb://ntc-data:zkicQepvrK6B1SySo3vDLQpycrEsSt3WByUE1Zg7SYs47CceytRCzIuP3cu3p09GtY2cREJyJtdc9zSjqlxcvA==@ntc-data.mongo.cosmos.azure.com:10255/?ssl=true&replicaSet=globaldb&retrywrites=false&maxIdleTimeMS=120000&appName=@ntc-data@").await.unwrap();
    client_options.app_name = Some("ntc-data".to_string());
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