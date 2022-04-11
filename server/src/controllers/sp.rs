use crate::models::models::*;
use anyhow::Result;
use sqlx::PgPool;

impl SpMap {
    pub async fn get_sp_map_page(
        pool: &PgPool,
        map_id: &String,
        limit: i32,
        cat_id: i32,
    ) -> Result<Vec<SpMap>> {
        match sqlx::query_as::<_, SpMap>(
            r#" 
                SELECT t.timestamp,
                    t.CL_profile_number,
                    t.score,
                    t.demo_id,
                    t.youtube_id,
                    t.submission,
                    t.note,
                    t.category_id,
                    CASE
                    WHEN t.board_name IS NULL
                        THEN t.steam_name
                    WHEN t.board_name IS NOT NULL
                        THEN t.board_name
                    END user_name,
                    t.avatar
                FROM (
                    SELECT DISTINCT ON (changelog.profile_number) 
                        changelog.profile_number as CL_profile_number,
                        users.profile_number as U_profile_number, *
                    FROM "p2boards".changelog
                    INNER JOIN "p2boards".users ON (users.profile_number = changelog.profile_number)
                        WHERE map_id = $1
                        AND users.banned = False
                        AND changelog.verified = True
                        AND changelog.banned = False
                        AND changelog.category_id = $2
                    ORDER BY changelog.profile_number, changelog.score ASC
                ) t
                ORDER BY score
                LIMIT $3"#,
        )
        .bind(map_id)
        .bind(cat_id)
        .bind(limit)
        .fetch_all(pool)
        .await
        {
            Ok(res) => Ok(res),
            Err(e) => {
                eprintln!("{}", e);
                Err(anyhow::Error::new(e).context("Error with SP Maps"))
            }
        }
    }
}

impl SpPreview {
    /// Gets preview information for top 7 on an SP Map.
    pub async fn get_sp_preview(pool: &PgPool, map_id: String) -> Result<Vec<SpPreview>> {
        Ok(sqlx::query_as::<_, SpPreview>(
            r#"
                SELECT t.CL_profile_number, t.score, t.youtube_id, t.category_id,
                CASE
                    WHEN t.board_name IS NULL
                        THEN t.steam_name
                    WHEN t.board_name IS NOT NULL
                        THEN t.board_name
                    END user_name, t.map_id
                FROM (
                    SELECT DISTINCT ON (changelog.profile_number) 
                        changelog.profile_number as CL_profile_number,
                        users.profile_number as U_profile_number, *
                    FROM "p2boards".changelog
                    INNER JOIN "p2boards".users ON (users.profile_number = changelog.profile_number)
                    WHERE map_id = $1
                    AND users.banned = False
                    AND changelog.banned = False
                    ORDER BY changelog.profile_number, changelog.score ASC
                ) t
               ORDER BY score
               LIMIT 7"#,
        )
        .bind(map_id.clone())
        .fetch_all(pool)
        .await?)
    }
}

impl SpPreviews {
    /// Collects the top 7 preview data for all SP maps.
    pub async fn get_sp_previews(pool: &PgPool) -> Result<Vec<SpPreviews>> {
        let map_id_vec = Maps::get_steam_ids(pool, false).await?;
        let mut vec_final = Vec::new();
        for map_id in map_id_vec.iter() {
            let vec_temp = SpPreview::get_sp_preview(pool, map_id.to_string()).await?;
            vec_final.push(SpPreviews {
                map_id: map_id.clone(),
                scores: vec_temp,
            })
        }
        Ok(vec_final)
    }
}

impl SpBanned {
    // Returns all profile_numbers and scores associated with banned times on a given map
    pub async fn get_sp_banned(pool: &PgPool, map_id: String) -> Result<Vec<SpBanned>> {
        Ok(sqlx::query_as::<_, SpBanned>(
            r#"
                SELECT changelog.profile_number, changelog.score 
                    FROM "p2boards".changelog
                    WHERE changelog.banned = True
                        AND changelog.map_id = $1
                    ORDER BY changelog.score ASC
            "#,
        )
        .bind(map_id)
        .fetch_all(pool)
        .await?)
    }
}
