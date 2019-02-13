use scheduling_infrastructure::connection::{create_connection, PostgresPool};

pub fn setup() -> PostgresPool {
    create_connection("APP_TESTING_USER", "APP_TESTING_PASS", "APP_TESTING_DB").unwrap()
}
