use actix_web::{web, Responder};

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/logs")
            .route(web::get().to(get_logs))
            .route(web::post().to(add_log)),
    );
}

async fn get_logs() -> impl Responder {
    format!("Not yet implemented!")
}

async fn add_log() -> impl Responder {
    format!("Not yet implemented!")
}