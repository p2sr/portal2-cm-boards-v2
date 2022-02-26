use crate::controllers::models::ChangelogPage;
use crate::controllers::models::ChangelogQueryParams;

use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

/// **GET** method for changelog entiries. Utilizes `ChangelogQueryParrams` as an optional addition to the query
///
/// **Optional Parameters**: [crate::controllers::models::ChangelogQueryParams]
/// ## Parameters:
///    - **limit**           
///         - The # of max returned results.
///    - **nick_name**       
///         - Filters for results from all profile_numbers were steam/board name matches `(%TEXT%)`.
///    - **profile_number**  
///         - Returns scores only from a specific profile (steam) id.
///    - **chamber**         
///         - Filters for only a specfic map by id.
///    - **sp**              
///         - Boolean for determines if sp maps should be returned
///    - **coop**            
///         - Boolean that determines if coop maps should be returned
///    - **wr_gain**         
///         - Boolean that, if true, will only return scores that were originally World Records
///    - **has_demo**        
///         - Boolean that will filter for only scores with demos
///    - **yt**              
///         - Boolean that will filter for onlny scores with youtube links
///    - **first**           
///         - Will only return scores with an ID higher than the given amount
///    - **last**            
///         - Will only return scores with an ID lower than the given amount
/// ## Example endpoints:
///  - **Default**           
///     - `/changelog`
///  - **With parameters**   
///     - `/changelog?limit=200&nick_name=Zypeh&chamber=47759&sp=true&coop=false&wr_gain=true&has_demo=true&yt=true`
///  - **A refresh call**    
///     - `/changelog?limit=200&first=157804`
///  - **A scroll call**     
///     - `/changelog?limit-200&last=157604`
///
/// Makes a call to the underlying [ChangelogPage::get_changelog_page]
#[get("/changelog")]
async fn get_changelog(
    pool: web::Data<PgPool>,
    query_params: web::Query<ChangelogQueryParams>,
) -> impl Responder {
    let res = ChangelogPage::get_changelog_page(pool.get_ref(), query_params.into_inner()).await;
    match res {
        Ok(changelog_entries) => HttpResponse::Ok().json(changelog_entries),
        _ => HttpResponse::NotFound().body("No changelog entries found."),
    }
}
