use actix_web::web;

use crate::api::v1::handlers::sp::{get_singleplayer_maps, get_singleplayer_preview};
use crate::api::v1::handlers::coop::{get_cooperative_maps, get_cooperative_preview};
use crate::api::v1::handlers::changelog::{get_changelog, post_changelog_filtered};
use crate::api::v1::handlers::users::{get_banned_users, check_ban_status};

/// Mounts the routes to /api/..
pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .service(get_singleplayer_maps)
            .service(get_cooperative_maps)
            .service(get_singleplayer_preview)
            .service(get_cooperative_preview)
            .service(get_changelog)
            .service(post_changelog_filtered)
            .service(check_ban_status)
            .service(get_banned_users)
    );
}