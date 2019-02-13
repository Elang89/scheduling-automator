/// This module contains the specifications to create a Repository
///     - HandleCustomerAccounts specifies the behavior that a repository for customer accounts must have
///     - Mock is used to create unit tests
///     - Repository
pub mod mock;
pub mod repository;

use crate::models::customer_account::{CustomerAccount, NewCustomerAccount};
use uuid::Uuid;

pub trait HandleCustomerAccounts {
    fn get_customer_account_by_id(&self, id: Uuid) -> Result<CustomerAccount, String>;

    fn get_all_customer_accounts(&self) -> Result<Vec<CustomerAccount>, String>;

    fn create_customer_account(&mut self, account: NewCustomerAccount) -> Result<CustomerAccount, String>;

    fn update_customer_account(&mut self, account: CustomerAccount) -> Result<CustomerAccount, String>;

    fn delete_customer_account(&mut self, id: Uuid) -> Result<CustomerAccount, String>;
}

type DefaultRepository = Box<dyn HandleCustomerAccounts>;

pub fn create_mock() -> DefaultRepository {
    Box::new(mock::Mock::new())
}
