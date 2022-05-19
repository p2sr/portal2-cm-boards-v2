use chrono::NaiveDateTime;
use sqlx::types::Json;
use sqlx::FromRow;

/// One-to-one struct for demo data.
#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Demos {
    pub id: i64,
    pub file_id: String,
    pub partner_name: Option<String>,
    pub parsed_successfully: bool,
    pub sar_version: Option<String>,
    pub cl_id: i64,
    pub updated: Option<NaiveDateTime>,
}

/// One-to-one struct for mtrigger data.
#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Mtriggers {
    pub id: i32,
    pub map_id: String,
    pub category_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// One-to-one struct for mtrigger_entry data.
#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct MtriggerEntries {
    pub id: i32,
    pub mtrigger_id: i32,
    pub changelog_id: i64,
    pub time: i32,
}

/// The bundled mtrigger & mtrigger entry
#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct MtriggerBundle {
    pub mtrigger_id: i32,
    pub map_id: String,
    pub category_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub mtrigger_entry_id: i32,
    pub changelog_id: i64,
    pub time: i32,
}

/// Insert struct for `Demos`, excludes `id`
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DemoInsert {
    pub file_id: String,
    pub partner_name: Option<String>,
    pub parsed_successfully: bool,
    pub sar_version: Option<String>,
    pub cl_id: i64,
}

/// Insert struct for `MtriggerEntries`, excludes `id`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MtriggerEntriesInsert {
    mtrigger_id: i32,
    changelog_id: i64,
    time: i32,
}

/// Insert struct for `Mtriggers`, excludes `id`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MtriggersInsert {
    pub map_id: String,
    pub category_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Allows us to accept an optional demo_id or cl_id as a set of query parameters for demo endpoints.
///
/// Intended to be used exclusively (you should either use one or the other, never both or neither) if you're calling to query for a demo,
/// if you're using this to update a demo, both are required fields.
#[derive(Debug, Clone, Deserialize)]
pub struct DemoOptions {
    pub demo_id: Option<i64>,
    pub cl_id: Option<i64>,
}
