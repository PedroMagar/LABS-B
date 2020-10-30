use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;

use super::Pool;

pub fn connect() -> Pool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");// create db connection pool
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);              //
    r2d2::Pool::builder()                                                               //
        .build(manager)                                                                 //
        .expect("Failed to create pool.")                                               //
}
