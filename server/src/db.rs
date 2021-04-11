use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};

// DbPool using r2d2 to handle a pool of mysql connections
pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;