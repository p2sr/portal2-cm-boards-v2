use actix_web::web;

use crate::api::v1::handlers::{
    admin::*, changelog::*, chapters::*, coop::*, demos::*, maps::*, points::*, sp::*, stats::*,
    users::*,
};

/// Mounts the routes to /api/..
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(changelog)
            .service(changelog_new)
            .service(graph)
            .service(changelog_demo_update)
            .service(default_categories_all)
            .service(sp)
            .service(sp_map)
            .service(sp_banned)
            .service(sp_all_banned)
            .service(sp_history)
            .service(sp_update)
            .service(sp_validate)
            .service(sp_post_score)
            .service(coop)
            .service(coop_map)
            .service(coop_banned_all)
            .service(coop_banned)
            .service(coop_add)
            .service(coop_temp)
            .service(coop_update_changelog)
            .service(maps)
            .service(default_category)
            .service(map_ids)
            .service(chapter)
            .service(chapters_filtered)
            .service(maps_from_chapter)
            .service(user)
            .service(user_add)
            .service(avatar_update)
            .service(banned_users_all)
            .service(banned_user)
            .service(donators)
            .service(wall_of_shame)
            .service(profile)
            .service(points_sp)
            .service(points_sp_add)
            .service(points_coop)
            .service(points_coop_add)
            .service(points_chapter)
            .service(points_chapter_add)
            .service(points_overall)
            .service(points_overall_add)
            .service(demos)
            .service(demos_changelog)
            .service(demos_delete)
            .service(demos_add)
            .service(admin_changelog)
            .service(admin_banned_stats)
            .service(admins_list)
            .service(count_scores)
            .service(count_scores_by_map)
            .service(recap)
            .service(badges)
            .service(users_badges),
    );
}
