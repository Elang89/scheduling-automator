//! Seeding data into database
//!
//! This crate is an script to seed data into the database. It receives a user, password
//! and the name of a database to create a connection
//!
//!
//! # Layout
//!
//! This crate consists of two functions, main() and generate_fake_account.
//! The function main() serves as the entrypoint and inserts
//! the data into the database. Meanwhile generate_fake_date() uses the
//! fake! macro from the fake crate to generate fake user accounts

extern crate diesel;
extern crate scheduling_infrastructure;
extern crate uuid;

#[macro_use]
extern crate fake;

use diesel::prelude::*;
use scheduling_infrastructure::connection::create_connection;
use scheduling_infrastructure::models::customer_account::NewCustomerAccount;
use scheduling_infrastructure::schema::customer_accounts::dsl::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let user = &args[1];
    let pass = &args[2];
    let db = &args[3];

    let connection = create_connection(user, pass, db)
        .expect("Database connection could not be established")
        .get()
        .expect("Pool could not retrieve connection");

    let random_accounts: Vec<NewCustomerAccount> =
        (0..10).map(|_| generate_fake_account()).collect();

    diesel::insert_into(customer_accounts)
        .values(&random_accounts)
        .execute(&connection)
        .expect("Error inserting customer accounts");
}

fn generate_fake_account() -> NewCustomerAccount {
    NewCustomerAccount {
        first_name: fake!(Name.name),
        last_name: fake!(Name.name),
        email: fake!(Internet.free_email),
    }
}
