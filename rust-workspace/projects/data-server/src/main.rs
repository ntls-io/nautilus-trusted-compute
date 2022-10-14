use bson::{doc, Bson, Document};
use env_var_helpers::env_vars;
use mongodb::{error::Error, options::ClientOptions, Client, Collection, Database};

pub struct CosmosDBMongo {
    connection_string: String,
    database_name: String,
    collection_name: String,
}

fn main() {
    println!("Hello, world!");
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