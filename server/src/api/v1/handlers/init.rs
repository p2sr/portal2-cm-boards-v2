use actix_web::web;

use crate::api::v1::handlers::sp::{get_singleplayer_maps, get_singleplayer_preview, get_banned_scores, post_banned_scores, get_sp_pbs, post_score_sp};
use crate::api::v1::handlers::coop::{get_cooperative_maps, get_cooperative_preview};
use crate::api::v1::handlers::changelog::{get_changelog, post_changelog_filtered};
use crate::api::v1::handlers::users::{get_banned_users, check_ban_status};

/// Mounts the routes to /api/..
pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .service(get_singleplayer_maps) // SP
            .service(get_singleplayer_preview)
            .service(get_banned_scores)
            .service(post_banned_scores)
            .service(get_sp_pbs)
            .service(post_score_sp)
            .service(get_cooperative_maps) // Coop
            .service(get_cooperative_preview)
            .service(get_changelog)
            .service(post_changelog_filtered)
            .service(check_ban_status) // Users
            .service(get_banned_users)
    );
}