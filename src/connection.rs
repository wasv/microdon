use dotenv::dotenv;
use std::env;
use std::ops::Deref;

use diesel::r2d2::ConnectionManager;

use diesel::pg::PgConnection;

embed_migrations!("migrations/");

pub(crate) type Conn = PgConnection;

/// Generates database url from environment variables.
fn database_url() -> String {
    dotenv().ok();
    let dbhost = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
    let dbuser = env::var("DATABASE_USER").unwrap_or_else(|_| "microdon".to_string());
    let dbname = env::var("DATABASE_NAME").unwrap_or_else(|_| dbuser.clone());
    let dbpass = env::var("DATABASE_PASS").unwrap_or_else(|_| "".to_string());
    format!("postgres://{}:{}@{}/{}", dbuser, dbpass, dbhost, dbname)
}

/// Shorthand type for the database pool.
pub type Pool = r2d2::Pool<ConnectionManager<Conn>>;

/// Creates a new database pool.
pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<Conn>::new(database_url());
    let pool =
        Pool::new(manager).unwrap_or_else(|_| panic!("Error connecting to {}", database_url()));
    perform_migrations(&pool);
    pool
}

/// Preforms database migration.
pub fn perform_migrations(pool: &Pool) {
    let conn = pool.get().unwrap();
    embedded_migrations::run(&conn).unwrap_or_else(|_| panic!("Error running migration."));
}

/// Shorthand type for the pooled database connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<Conn>>);

impl Deref for DbConn {
    type Target = Conn;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
