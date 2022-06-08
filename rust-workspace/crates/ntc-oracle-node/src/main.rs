mod handlers;
mod helpers;
mod settings;
mod signer;

use std::error::Error;

use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;

use crate::helpers::bind_addr_from_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bind_addr = bind_addr_from_env()?;

    // CORS:
    let cors_layer = CorsLayer::permissive();

    let axum_app = Router::new()
        .route("/auth_data", get(handlers::auth_data::auth_data))
        // The CORS layer must come after the wrapped resources, for correct response headers.
        .layer(cors_layer);
    let axum_server = axum::Server::bind(&bind_addr).serve(axum_app.into_make_service());
    println!("listening on http://{}", bind_addr);
    axum_server.await?;

    Ok(())
}
