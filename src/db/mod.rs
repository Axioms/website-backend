pub mod models;
pub mod schema;

use dotenv::dotenv;
use std::env;

use diesel::{r2d2, r2d2::ConnectionManager, Connection as DieselConnection, ConnectionError};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request, State,
};

type Connection = diesel::mysql::MysqlConnection;
type Pool = r2d2::Pool<ConnectionManager<Connection>>;
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<Connection>>);

pub fn get_database_url() -> String {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let url = db_url.to_owned();
    url
}

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::new(get_database_url());

    r2d2::Pool::builder().build(manager).expect("Failed to create pool")
}

pub fn get_connection() -> Result<Connection, ConnectionError> {
    Connection::establish(&get_database_url().to_owned()[..])
}

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<DbConn, ()> {
        // https://github.com/SergioBenitez/Rocket/commit/e3c1a4ad3ab9b840482ec6de4200d30df43e357c
        let pool = try_outcome!(request.guard::<State<Pool>>());
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as a &Database.
impl std::ops::Deref for DbConn {
    type Target = Connection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}