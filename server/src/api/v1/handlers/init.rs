use actix_web::web;

use crate::api::v1::handlers::sp::*;
use crate::api::v1::handlers::coop::*;
use crate::api::v1::handlers::changelog::*;
use crate::api::v1::handlers::users::*;

/// Mounts the routes to /api/..
pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .service(get_singleplayer_maps) // SP
            .service(get_singleplayer_preview)
            .service(get_banned_scores_sp)
            .service(post_banned_scores_sp)
            .service(get_banned_scores_coop)
            .service(post_banned_scores_coop)
            .service(get_sp_pbs)
            .service(post_score_sp)
            .service(put_score_sp)
            .service(get_cooperative_maps) // Coop
            .service(get_cooperative_preview)
            .service(get_changelog)
            .service(post_changelog_filtered)
            .service(check_ban_status) // Users
            .service(get_banned_users)
    );
}