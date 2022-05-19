use crate::models::coop::*;
use crate::models::maps::Maps;
use crate::models::changelog::Changelog;
use anyhow::Result;
use futures::future::try_join_all;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::collections::HashSet;

impl CoopBundled {
    pub async fn insert_coop_bundled(pool: &PgPool, cl: CoopBundledInsert) -> Result<i64> {
        Ok(sqlx::query(
            r#"
                INSERT INTO coop_bundled 
                (p_id1, p_id2, p1_is_host, cl_id1, cl_id2) VALUES 
                ($1, $2, $3, $4, $5)
                RETURNING id"#,
        )
        .bind(cl.p_id1)
        .bind(cl.p_id2)
        .bind(cl.p1_is_host)
        .bind(cl.cl_id1)
        .bind(cl.cl_id2)
        .map(|row: PgRow| row.get(0))
        .fetch_one(pool)
        .await?)
    }
    pub async fn get_temp_coop_changelog(pool: &PgPool, map_id: &str) -> Result<CoopTempUser> {
        Ok(sqlx::query_as::<_, CoopTempUser>(r#"SELECT id AS cl_id, profile_number FROM changelog WHERE profile_number = 'N/A' AND map_id = $1"#)
            .bind(map_id)
            .fetch_one(pool)
            .await?)
    }
    pub async fn update_changelog_with_coop_id(pool: &PgPool, cl_id: i64, coop_id: i64) -> Result<Option<Changelog>> {
        Ok(sqlx::query_as::<_, Changelog>(r#"UPDATE changelog SET coop_id = $1 WHERE id = $2 RETURNING *"#)
            .bind(coop_id)
            .bind(cl_id)
            .fetch_optional(pool)
            .await?)
    }
}

impl CoopMap {
    pub async fn get_coop_map_page(
        pool: &PgPool,
        map_id: &String,
        limit: i32,
        cat_id: i32,
        game_id: i32,
    ) -> Result<Vec<CoopMap>> {
        match sqlx::query_as::<_, CoopMap>(
            r#"
                SELECT c1.timestamp, 
                    c1.score, cb.p1_is_host, c1.note AS note1, c2.note AS note2,
                    COALESCE(p1.board_name, p1.steam_name) AS user_name1,
                    COALESCE(p2.board_name, p2.steam_name) AS user_name2,
                    c1.profile_number AS profile_number1, c2.profile_number AS profile_number2, 
                    c1.demo_id AS demo_id1, c2.demo_id AS demo_id2, 
                    c1.youtube_id AS youtube_id1, c2.youtube_id AS youtube_id2,
                    c1.submission AS submission1, c2.submission AS submission2, 
                    c1.category_id, p1.avatar AS avatar1, p2.avatar AS avatar2
                FROM (SELECT * FROM 
                coop_bundled 
                WHERE id IN 
                    (SELECT coop_id
                    FROM changelog
                    WHERE map_id = $1
                    AND coop_id IS NOT NULL)) as cb 
                INNER JOIN changelog AS c1 ON (c1.id = cb.cl_id1)
                INNER JOIN changelog AS c2 ON (c2.id = cb.cl_id2)
                INNER JOIN users AS p1 ON (p1.profile_number = cb.p_id1)
                INNER JOIN users AS p2 ON (p2.profile_number = cb.p_id2)
                INNER JOIN maps ON (c1.map_id = maps.steam_id)
                INNER JOIN chapters ON (maps.chapter_id = chapters.id)
                WHERE p1.banned=False
                    AND p2.banned = False
                    AND c1.banned = False
                    AND c2.banned = False
                    AND c1.verified = True
                    AND c2.verified = True
                    AND c1.category_id = $2
                    AND chapters.game_id = $3
                ORDER BY score ASC
                "#,
        )
        .bind(map_id)
        .bind(cat_id)
        .bind(game_id)
        .fetch_all(pool)
        .await
        {
            Ok(mut res) => {
                res.truncate(limit as usize);
                Ok(res)
            }
            Err(e) => {
                eprintln!("{}", e);
                Err(anyhow::Error::new(e).context("Error with Coop Maps"))
            }
        }
    }
}

impl CoopPreview {
    // TODO: Filter by default cat_id
    /// Gets the top 7 (unique on player) times on a given Coop Map.
    pub async fn get_coop_preview(pool: &PgPool, map_id: &str) -> Result<Vec<CoopPreview>> {
        // TODO: Open to PRs to contain all this functionality in the SQL statement.
        let res = sqlx::query_as::<_, CoopPreview>(
            r#"
                SELECT
                    c1.profile_number AS profile_number1, c2.profile_number AS profile_number2,
                    c1.score,
                    c1.youtube_id AS youtube_id1, c2.youtube_id AS youtube_id2, c1.category_id,
                    COALESCE(p1.board_name, p1.steam_name) AS user_name1, 
                    COALESCE(p2.board_name, p2.steam_name) AS user_name2, c1.map_id
                FROM (SELECT * FROM 
                coop_bundled 
                WHERE id IN 
                    (SELECT coop_id
                    FROM changelog
                    WHERE map_id = $1
                    AND coop_id IS NOT NULL)) as cb 
                INNER JOIN changelog AS c1 ON (c1.id = cb.cl_id1)
                INNER JOIN changelog AS c2 ON (c2.id = cb.cl_id2)
                INNER JOIN users AS p1 ON (p1.profile_number = cb.p_id1)
                INNER JOIN users AS p2 ON (p2.profile_number = cb.p_id2)
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
        .bind(map_id)
        .fetch_all(pool)
        .await?;

        let mut vec_final = Vec::new();
        let mut remove_dups = HashSet::with_capacity(80);
        remove_dups.insert("N/A".to_string());
        for entry in res {
            match remove_dups.insert(entry.profile_number1.clone()) {
                false => match remove_dups.insert(entry.profile_number2.clone().unwrap()) {
                    false => (),
                    true => vec_final.push(entry),
                },
                true => match remove_dups.insert(entry.profile_number2.clone().unwrap()) {
                    false => vec_final.push(entry),
                    true => vec_final.push(entry),
                },
            }
        }
        vec_final.truncate(7);
        Ok(vec_final)
    }
    // Collects the top 7 preview data for all Coop maps.
    pub async fn get_coop_previews(pool: &PgPool) -> Result<Vec<Vec<CoopPreview>>> {
        let map_id_vec = Maps::get_steam_ids(pool, true).await?;
        let futures: Vec<_> = map_id_vec
            .iter()
            .map(|map_id| CoopPreview::get_coop_preview(pool, map_id))
            .collect();
        try_join_all(futures).await
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
        Ok(sqlx::query_as::<_, CoopBanned>(r#"
                SELECT c1.score, c1.profile_number AS profile_number1, c2.profile_number AS profile_number2
                FROM (SELECT * FROM 
                    coop_bundled 
                    WHERE id IN 
                    (SELECT coop_id
                    FROM changelog
                    WHERE map_id = $1
                    AND coop_id IS NOT NULL)) as cb
                LEFT JOIN changelog AS c1 ON (c1.id = cb.cl_id1)
                LEFT JOIN changelog AS c2 ON (c2.id = cb.cl_id2)
                    WHERE (c1.banned = True OR c1.verified = False)
                    OR (c2.banned = True OR c2.verified = False)
                    AND c1.category_id = $2
                "#)
            .bind(map_id)
            .bind(cat_id)
            .fetch_all(pool)
            .await?)
    }
}
