use crate::api::v1::routes::handlers::*;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/backend/v1")
            .service(rcp)
            .service(fetch_sp)
            .service(fetch_all)
            .service(fetch_pfp)
            .service(fetch_coop)
            .service(update_from_legacy_boards),
    );
}
