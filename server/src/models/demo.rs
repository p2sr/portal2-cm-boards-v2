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
}

/// Insert struct for `Demos`, excludes `id`
#[derive(Debug, Default, Serialize, Deserialize, FromRow, Clone)]
pub struct DemoInsert {
    pub file_id: String,
    pub partner_name: Option<String>,
    pub parsed_successfully: bool,
    pub sar_version: Option<String>,
    pub cl_id: i64,
}

/// Allows us to accept an optional demo_id or cl_id as a set of query parameters for demo endpoints.
///
/// Intended to be used exclusively (you should either use one or the other, never both or neither).
#[derive(Debug, Clone, Deserialize)]
pub struct DemoOptions {
    pub demo_id: Option<i64>,
    pub cl_id: Option<i64>,
}
