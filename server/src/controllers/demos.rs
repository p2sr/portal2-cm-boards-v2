use crate::models::demos::*;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};

impl Demos {
    /// Gets Demo information for a given demo_id
    pub async fn get_demo(pool: &PgPool, demo_id: i64) -> Result<Option<Demos>, sqlx::Error> {
        sqlx::query_as::<_, Demos>(r#"SELECT * FROM demos WHERE id = $1"#)
            .bind(demo_id)
            .fetch_optional(pool)
            .await
    }
    /// Gets Demo information for a given changelog_id
    pub async fn get_demo_by_cl_id(pool: &PgPool, cl_id: i64) -> Result<Option<Demos>, sqlx::Error> {
        sqlx::query_as::<_, Demos>(r#"SELECT * FROM demos WHERE cl_id = $1"#)
            .bind(cl_id)
            .fetch_optional(pool)
            .await
    }
    /// Returns a file id that can be used to download the demo
    #[allow(dead_code)]
    pub async fn get_demo_file_id(pool: &PgPool, demo_id: i64) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar(r#"SELECT file_id FROM demos WHERE id = $1"#)
            .bind(demo_id)
            .fetch_one(pool)
            .await
    }
    /// Returns the partner's name
    #[allow(dead_code)]
    pub async fn get_partner_name(pool: &PgPool, demo_id: i64) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar(r#"SELECT partner_name FROM demos WHERE id = $1"#)
            .bind(demo_id)
            .fetch_one(pool)
            .await
    }
    /// Check to see if a demo was parsed successfully
    #[allow(dead_code)]
    pub async fn check_parsed(pool: &PgPool, demo_id: i64) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar(r#"SELECT parsed_successfully FROM demos WHERE id = $1"#)
            .bind(demo_id)
            .fetch_one(pool)
            .await
    }
    /// Gets the SAR version associated with a demo
    #[allow(dead_code)]
    pub async fn get_sar_version(pool: &PgPool, demo_id: i64) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar(r#"SELECT sar_version FROM demos WHERE id = $1"#)
            .bind(demo_id)
            .fetch_one(pool) // This should NOT be optional, as the value can be NULL
            .await
    }
    /// Adds a new demo to the database, returns the demo's id
    pub async fn insert_demo(pool: &PgPool, demo: DemoInsert) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar(
            r#"
                INSERT INTO demos 
                (file_id, partner_name, parsed_successfully, sar_version, cl_id) VALUES 
                ($1, $2, $3, $4, $5)
                RETURNING id"#,
        )
        .bind(demo.file_id)
        .bind(demo.partner_name)
        .bind(demo.parsed_successfully)
        .bind(demo.sar_version)
        .bind(demo.cl_id)
        .fetch_one(pool)
        .await
    }
    /// Updates an existing demo
    #[allow(dead_code)]
    pub async fn update_demo(pool: &PgPool, updated_demo: Demos) -> Result<Demos, sqlx::Error> {
        // TODO: Validation
        sqlx::query_as::<_, Demos>(
            r#"
                UPDATE demos
                SET file_id = $1, partner_name = $2, parsed_successfully = $3,
                sar_version = $4, cl_id = $5
                WHERE id = $6 RETURNING *"#,
        )
        .bind(updated_demo.file_id)
        .bind(updated_demo.partner_name)
        .bind(updated_demo.parsed_successfully)
        .bind(updated_demo.sar_version)
        .bind(updated_demo.cl_id)
        .bind(updated_demo.id)
        .fetch_one(pool)
        .await
    }
    /// Deletes a demo
    pub async fn delete_demo(pool: &PgPool, demo_id: i64) -> Result<Demos, sqlx::Error> {
        sqlx::query_as::<_, Demos>(
            r#"DELETE FROM demos 
                WHERE id = $1 RETURNING *"#,
        )
        .bind(demo_id)
        .fetch_one(pool)
        .await
    }
}

impl Mtriggers {
    #[allow(dead_code)]
    /// Returns [MtriggerBundle] for a given changelog entry.
    pub async fn get_mtriggers_from_cl_id(
        pool: &PgPool,
        cl_id: i64,
    ) -> Result<Vec<MtriggerBundle>, sqlx::Error> {
        sqlx::query_as::<_, MtriggerBundle>(
            r#"SELECT mtriggers.id AS mtrigger_id, map_id, category_id, name, 
            description, mtrigger_entries.id AS mtrigger_entry_id, changelog_id, time
                FROM mtrigger_entries
                INNER JOIN mtriggers ON (mtrigger_entries.mtrigger_id = mtriggers.id)
                WHERE mtrigger_entries.changelog_id = $1;"#,
        )
        .bind(cl_id)
        .fetch_all(pool)
        .await
    }
}

