use dotenv::dotenv;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::env;
use std::ops::Deref;

use diesel::r2d2::ConnectionManager;

use diesel::pg::PgConnection;

embed_migrations!("migrations/");

pub(crate) type Conn = PgConnection;

fn database_url() -> String {
    dotenv().ok();
    let dbhost = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let dbuser = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let dbname = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let dbpass = env::var("DATABASE_PASS").expect("DATABASE_PASS must be set");
    format!("postgres://{}:{}@{}/{}", dbuser, dbpass, dbhost, dbname)
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<Conn>>);

type Pool = r2d2::Pool<ConnectionManager<Conn>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<Conn>::new(database_url());
    let pool =
        Pool::new(manager).unwrap_or_else(|_| panic!("Error connecting to {}", database_url()));
    perform_migrations(&pool);
    pool
}

pub fn perform_migrations(pool: &Pool) {
    let conn = pool.get().unwrap();
    embedded_migrations::run(&conn).unwrap_or_else(|_| panic!("Error running migration."));
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = Conn;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
