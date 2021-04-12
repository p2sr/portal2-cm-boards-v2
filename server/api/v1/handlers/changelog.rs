#[get("/")]
async fn changelog_default(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    let limit: i32 = 200;
    let changelog_entries = web::block(move || ChangelogPage::show(&conn, limit))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    if let Some(changelog_entries) = changelog_entries{
        Ok(HttpResponse::Ok().json(changelog_entries))
    } else {
        let res = HttpResponse::NotFound()
            .body("No changelog entries found.");
        Ok(res)
    }
}

// Thank you POST method :)
/// POST method for changelog that allows the user to submit a JSON body to filter for specific parameters. See the ChangelogQueryParams struct info on accepted query parameters.println!
#[post("/")]
async fn changelog_filtered(params: web::Json<ChangelogQueryParams>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    println!("Requested: {:#?}", params);
    let changelog_entries = web::block(move || ChangelogPage::show_filtered(&conn, params.nickname.clone(), params.profilenumber.clone(), params.chamber.clone(), params.sp, params.coop, params.wrgain, params.hasdemo, params.yt, params.limit))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    if let Some(changelog_entries) = changelog_entries{
        Ok(HttpResponse::Ok().json(changelog_entries))
    } else {
        let res = HttpResponse::NotFound()
            .body("No changelog entries found.");
        Ok(res)
    }
}

pub fn mnt_changelog(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/changelog")
            .service(changelog_default)
            .service(changelog_filtered)
    );
}