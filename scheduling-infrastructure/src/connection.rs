use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection(user: &str, pass: &str, db: &str) -> Result<PostgresPool, r2d2::Error> {
    let database_uri = format!(
        "postgres://{}:{}@localhost/{}",
        env::var(user).expect(&create_error_string(user)),
        env::var(pass).expect(&create_error_string(pass)),
        env::var(db).expect(&create_error_string(db))
    );

    let manager = ConnectionManager::<PgConnection>::new(database_uri);
    let sql_pool = Pool::new(manager)?;

    Ok(sql_pool)
}

fn create_error_string(name: &str) -> String {
    format!(
        "Connection failed, {} must be set, run 'source env.sh' to set",
        name
    )
}
