#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate fake;
extern crate r2d2;
extern crate uuid;

pub mod connection;
pub mod customer_accounts;
pub mod error;
pub mod models;
pub mod schema;
