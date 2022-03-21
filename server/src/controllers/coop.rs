use crate::models::models::*;
use anyhow::Result;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

impl CoopBundled {
    pub async fn insert_coop_bundled(pool: &PgPool, cl: CoopBundledInsert) -> Result<i64> {
        let mut res: i64 = 0;
        let _ = sqlx::query(
            r#"
                INSERT INTO "p2boards".coop_bundled 
                (p_id1, p_id2, p1_is_host, cl_id1, cl_id2) VALUES 
                ($1, $2, $3, $4, $5)
                RETURNING id"#,
        )
        .bind(cl.p_id1)
        .bind(cl.p_id2)
        .bind(cl.p1_is_host)
        .bind(cl.cl_id1)
        .bind(cl.cl_id2)
        .map(|row: PgRow| res = row.get(0))
        .fetch_one(pool)
        .await?;
        Ok(res)
    }
}

impl CoopMap {
    // TODO: Check to make sure this is filtered when returned (I think it is).
    pub async fn get_coop_map_page(
        pool: &PgPool,
        map_id: String,
        limit: i32,
        cat_id: i32,
    ) -> Result<Vec<CoopMap>> {
        let res = sqlx::query_as::<_, CoopMap>(
            r#"
                SELECT  c1.timestamp, 
                    c1.score, cb.p1_is_host, c1.note AS note1, c2.note AS note2,
                    CASE 
                        WHEN p1.board_name IS NULL
                            THEN p1.steam_name
                        WHEN p1.board_name IS NOT NULL
                            THEN p1.board_name
                    END user_name1, 
                        CASE 
                        WHEN p2.board_name IS NULL
                            THEN p2.steam_name
                        WHEN p2.board_name IS NOT NULL
                            THEN p2.board_name
                    END user_name2,
                    c1.profile_number AS profile_number1, c2.profile_number AS profile_number2, 
                    c1.demo_id AS demo_id1, c2.demo_id AS demo_id2, 
                    c1.youtube_id AS youtube_id1, c2.youtube_id AS youtube_id2,
                    c1.submission AS submission1, c2.submission AS submission2, 
                    c1.category_id, p1.avatar AS avatar1, p2.avatar AS avatar2
                FROM (SELECT * FROM 
                "p2boards".coop_bundled 
                WHERE id IN 
                    (SELECT coop_id
                    FROM "p2boards".changelog
                    WHERE map_id = $1
                    AND coop_id IS NOT NULL)) as cb 
                INNER JOIN "p2boards".changelog AS c1 ON (c1.id = cb.cl_id1)
                INNER JOIN "p2boards".changelog AS c2 ON (c2.id = cb.cl_id2)
                INNER JOIN "p2boards".users AS p1 ON (p1.profile_number = cb.p_id1)
                INNER JOIN "p2boards".users AS p2 ON (p2.profile_number = cb.p_id2)
                WHERE p1.banned=False
                    AND p2.banned=False
                    AND c1.banned=False
                    AND c2.banned=False
                    AND c1.verified=True
                    AND c2.verified=True
                    AND c1.category_id=$2
                ORDER BY score ASC
                "#,
        )
        .bind(map_id)
        .bind(cat_id)
        .fetch_all(pool)
        .await;
        match res {
            Ok(mut res) => {
                res.truncate(limit as usize);
                Ok(res)
            }
            Err(e) => {
                eprintln!("{}", e);
                Err(anyhow::Error::new(e).context("Error with SP Maps"))
            }
        }
    }
}

impl CoopPreview {
    /// Gets the top 7 (unique on player) times on a given Coop Map.
    pub async fn get_coop_preview(pool: &PgPool, map_id: String) -> Result<Vec<CoopPreview>> {
        // TODO: Open to PRs to contain all this functionality in the SQL statement.
        // TODO: Filter by default cat_id
        let query = sqlx::query_as::<_, CoopPreview>(
            r#"
                SELECT
                    c1.profile_number AS profile_number1, c2.profile_number AS profile_number2,
                    c1.score,
                    c1.youtube_id AS youtube_id1, c2.youtube_id AS youtube_id2, c1.category_id,
                    CASE 
                    WHEN p1.board_name IS NULL
                        THEN p1.steam_name
                    WHEN p1.board_name IS NOT NULL
                        THEN p1.board_name
                    END user_name1, 
                    CASE 
                    WHEN p2.board_name IS NULL
                        THEN p2.steam_name
                    WHEN p2.board_name IS NOT NULL
                        THEN p2.board_name
                    END user_name2
                FROM (SELECT * FROM 
                "p2boards".coop_bundled 
                WHERE id IN 
                    (SELECT coop_id
                    FROM "p2boards".changelog
                    WHERE map_id = '47825'
                    AND coop_id IS NOT NULL)) as cb 
                INNER JOIN "p2boards".changelog AS c1 ON (c1.id = cb.cl_id1)
                INNER JOIN "p2boards".changelog AS c2 ON (c2.id = cb.cl_id2)
                INNER JOIN "p2boards".users AS p1 ON (p1.profile_number = cb.p_id1)
                INNER JOIN "p2boards".users AS p2 ON (p2.profile_number = cb.p_id2)
                WHERE p1.banned=False
                    AND p2.banned=False
                    AND c1.banned=False
                    AND c2.banned=False
                    AND c1.verified=True
                    AND c2.verified=True
                ORDER BY score ASC
                LIMIT 40
                "#,
        )
        .bind(map_id.clone())
        .fetch_all(pool)
        .await;
        match query {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                return Err(anyhow::Error::new(e).context("Error with SP Previews"));
            }
        }
        // TODO: Maybe remove unwrap(), it assumes that the profile_number2 will not be None.
        let mut vec_final = Vec::new();
        let mut remove_dups: HashMap<String, i32> = HashMap::with_capacity(80);
        remove_dups.insert("N/A".to_string(), 1);
        for entry in query.unwrap() {
            match remove_dups.insert(entry.profile_number1.clone(), 1) {
                Some(_) => match remove_dups.insert(entry.profile_number2.clone().unwrap(), 1) {
                    Some(_) => (),
                    _ => vec_final.push(entry),
                },
                _ => match remove_dups.insert(entry.profile_number2.clone().unwrap(), 1) {
                    Some(_) => vec_final.push(entry),
                    _ => vec_final.push(entry),
                },
            }
        }
        vec_final.truncate(7);
        Ok(vec_final)
    }
}

impl CoopPreviews {
    // Collects the top 7 preview data for all Coop maps.
    pub async fn get_coop_previews(pool: &PgPool) -> Result<Vec<CoopPreviews>> {
        let map_id_vec = Maps::get_steam_ids(pool, true).await?;
        let mut vec_final = Vec::new();
        for map_id in map_id_vec.iter() {
            let vec_temp = CoopPreview::get_coop_preview(pool, map_id.to_string()).await?;
            vec_final.push(CoopPreviews {
                map_id: map_id.clone(),
                scores: vec_temp,
            })
        }
        Ok(vec_final)
    }
}

impl CoopBanned {
    /// Currently returns two profile_numbers and a score associated with a coop_bundle where one or both times are either banned or unverifed.
    pub async fn get_coop_banned(
        pool: &PgPool,
        map_id: String,
        cat_id: i32,
    ) -> Result<Vec<CoopBanned>> {
        // TODO: Handle verified and handle if one is banned/not verified but the other isn't.
        // TODO: How to handle one player in coop not-being banned/unverified but the other is.
        let res = sqlx::query_as::<_, CoopBanned>(r#"
                SELECT c1.score, c1.profile_number AS profile_number1, c2.profile_number AS profile_number2
                FROM (SELECT * FROM 
                    "p2boards".coop_bundled 
                    WHERE id IN 
                    (SELECT coop_id
                    FROM "p2boards".changelog
                    WHERE map_id = $1
                    AND coop_id IS NOT NULL)) as cb
                LEFT JOIN "p2boards".changelog AS c1 ON (c1.id = cb.cl_id1)
                LEFT JOIN "p2boards".changelog AS c2 ON (c2.id = cb.cl_id2)
                    WHERE (c1.banned = True OR c1.verified = False)
                    OR (c2.banned = True OR c2.verified = False)
                    AND c1.category_id = $2
                "#)
            .bind(map_id)
            .bind(cat_id)
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
}
