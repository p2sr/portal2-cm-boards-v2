use actix_web::web;

use crate::api::v1::handlers::changelog::*;
use crate::api::v1::handlers::chapters::*;
use crate::api::v1::handlers::coop::*;
use crate::api::v1::handlers::demo::*;
use crate::api::v1::handlers::maps::*;
use crate::api::v1::handlers::points::*;
use crate::api::v1::handlers::sp::*;
use crate::api::v1::handlers::users::*;

/// Mounts the routes to /api/..
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(get_singleplayer_maps) // SP
            // .service(web::resource(["/map/sp/{map_id}"]).to(get_singleplayer_maps))
            .service(get_singleplayer_preview)
            .service(get_banned_scores_sp)
            .service(post_banned_scores_sp)
            .service(get_banned_scores_coop)
            .service(post_banned_scores_coop)
            .service(get_sp_pbs)
            .service(post_score_sp)
            .service(put_score_sp)
            .service(get_newscore_details)
            .service(get_cooperative_maps) // Coop
            .service(get_cooperative_preview)
            .service(get_changelog)
            .service(post_score_coop)
            // .service(post_changelog_filtered)
            .service(check_ban_status) // Users
            .service(get_banned_users)
            .service(post_new_user)
            .service(get_map_ids_by_chapter) // Chapters
            .service(get_chapter_by_name)
            .service(get_default_cat) // Maps
            .service(get_points_sp) // Points
            .service(post_points_sp)
            .service(get_points_coop)
            .service(post_points_coop)
            .service(get_points_chapter)
            .service(post_points_chapter)
            .service(get_points_overall)
            .service(post_points_overall)
            .service(receive_multiparts)
            .service(changelog_with_demo),
    );
}
