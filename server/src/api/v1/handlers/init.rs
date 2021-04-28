use actix_web::web;

use crate::api::v1::handlers::sp::mnt_sp;
use crate::api::v1::handlers::coop::mnt_coop;
use crate::api::v1::handlers::changelog::mnt_changelog;

pub fn mnt_api(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .configure(mnt_changelog)
            .configure(mnt_sp)
            .configure(mnt_coop)
    );
}