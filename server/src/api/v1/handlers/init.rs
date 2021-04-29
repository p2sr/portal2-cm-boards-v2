use actix_web::web;

use crate::api::v1::handlers::sp::{singleplayer_maps, singleplayer_preview};
use crate::api::v1::handlers::coop::{coop_maps, cooperative_preview};
use crate::api::v1::handlers::changelog::{changelog_default, changelog_filtered};

// use crate::api::v1::handlers::sp::mnt_sp;
// use crate::api::v1::handlers::coop::mnt_coop;
// use crate::api::v1::handlers::changelog::mnt_changelog;

// pub fn mnt_api(cfg: &mut web::ServiceConfig){
//     cfg.service(
//         web::scope("/api")
//             .configure(mnt_changelog)
//             .configure(mnt_sp)
//             .configure(mnt_coop)
//     );
// }
// Mounts the routes to /api/..
pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .service(singleplayer_maps)
            .service(coop_maps)
            .service(singleplayer_preview)
            .service(cooperative_preview)
            .service(changelog_default)
            .service(changelog_filtered)
    );
}