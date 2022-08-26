use std::net::ToSocketAddrs;

use actix::{Actor, Addr, Arbiter};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use crate::actors::VaultEnclaveActor;
use crate::resources;
use crate::traits::VaultEnclave;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) vault_enclave_addr: Addr<VaultEnclaveActor>,
}

/// Run the server on the given address.
pub async fn run_server<Addr>(
    vault_enclave: Box<dyn VaultEnclave>,
    bind_addr: Addr,
) -> std::io::Result<()>
where
    Addr: ToSocketAddrs,
{
    let enclave_arbiter = Arbiter::new();
    let vault_enclave_addr = Actor::start_in_arbiter(&enclave_arbiter.handle(), |_ctx| {
        VaultEnclaveActor { vault_enclave }
    });

    // TODO: Test coverage
    let server = HttpServer::new(move || {
        let app_state = AppState {
            vault_enclave_addr: vault_enclave_addr.clone(),
        };
        let cors = Cors::permissive(); // TODO: Tighten this a bit more?
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_state))
            .service(resources::enclave_report::get_enclave_report)
            .service(resources::vault_operation::post_vault_operation)
    });
    server.bind(bind_addr)?.run().await
}
