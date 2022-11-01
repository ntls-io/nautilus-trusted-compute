use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::sync::*;

mod data_handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Create log for actix to output errors
    env::set_var("RUST_LOG", "actix_web=debug");
    // Remember to set connection string when starting the server
    let mongo_url = env::var("CONNECTION_STRING").unwrap();
    let client_options = ClientOptions::parse(&mongo_url).await.unwrap();
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(web::scope("/api").configure(data_handlers::scoped_config))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
