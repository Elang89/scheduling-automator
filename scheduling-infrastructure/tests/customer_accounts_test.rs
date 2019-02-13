use chrono::prelude::*;
use chrono::NaiveDateTime;
use scheduling_infrastructure::customer_accounts::repository::CustomerAccountRepository;
use scheduling_infrastructure::customer_accounts::HandleCustomerAccounts;
use scheduling_infrastructure::models::customer_account::{CustomerAccount, NewCustomerAccount};
use uuid::Uuid;
mod common;

#[test]
fn get_customer_account_by_id_test() {
    let connection = common::setup();
    let service = CustomerAccountRepository::new(connection);
    let id = Uuid::parse_str("").expect("UUID parsing failed");

    let result = service.get_customer_account_by_id(id);
    assert!(result.is_ok());
}

#[test]
fn get_all_customer_accounts_test() {
    let connection = common::setup();
    let service = CustomerAccountRepository::new(connection);

    let result = service.get_all_customer_accounts();
    assert!(result.is_ok());
}

#[test]
fn create_customer_account_test() {
    let connection = common::setup();
    let mut service = CustomerAccountRepository::new(connection);

    let account = NewCustomerAccount {
        first_name: "John".to_string(),
        last_name: "Williams".to_string(),
        email: "williams@gmail.com".to_string(),
    };

    let result = service.create_customer_account(account);
    assert!(result.is_ok());
}

#[test]
fn update_customer_account_test() {
    let connection = common::setup();
    let mut service = CustomerAccountRepository::new(connection);

    let account = CustomerAccount {
        id: Uuid::parse_str("7299f293-61cc-4632-b604-7d240792c7f6").unwrap(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "doe@gmail.com".to_string(),
        date_registered: NaiveDateTime::parse_from_str(
            "2019-01-29 05:14:20.202759",
            "%Y-%m-%d %H:%M:%S",
        )
        .unwrap(),
        date_updated: Some(Utc::now().naive_utc()),
        status: true,
    };

    let result = service.update_customer_account(account);
    assert!(result.is_ok());
}

#[test]
fn delete_customer_account_test() {
    let connection = common::setup();
    let mut service = CustomerAccountRepository::new(connection);

    let id = Uuid::parse_str("7299f293-61cc-4632-b604-7d240792c7f6").unwrap();

    let result = service.delete_customer_account(id);
    assert!(result.is_ok());
}
