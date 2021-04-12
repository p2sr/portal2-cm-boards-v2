pub mod handlers;

pub fn mnt_api(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .configure(handlers::changelog::mnt_changelog)
            .configure(handlers::sp::mnt_sp)
            .configure(handlers::coop::mnt_coop)
    );
}