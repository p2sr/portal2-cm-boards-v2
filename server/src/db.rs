use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};

/// DbPool using r2d2 to handle a pool of mysql connections.
///
/// Designed to be used with the configuration from `config.rs`, which extracts 
/// information set in the `.env` file for things like database connections and server
/// hostnames and ports.
/// 
/// Our project uses diesel.rs as the ORM to interface with the database, and MySQL as
/// the database client.
/// # Usage
/// ```rust
/// let manager = ConnectionManager::<MysqlConnection>::new(config.database.database_url);
/// let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
/// 
/// HttpServer::new(move || {
///     App::new()
///         .data(pool.clone())
///         .route("/", web::get().to(|| HttpResponse::Ok()))
/// })
/// .bind(format!("{}:{}", config.server.host, config.server.port))?
/// .run()
/// .await
/// ```
pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;