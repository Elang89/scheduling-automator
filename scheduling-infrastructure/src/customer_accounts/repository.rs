use super::HandleCustomerAccounts;
use crate::connection::PostgresPool;
use crate::models::customer_account::{CustomerAccount, NewCustomerAccount};
use uuid::Uuid;

pub struct CustomerAccountRepository {
    db_connection: PostgresPool,
}

impl CustomerAccountRepository {
    pub fn new(connection: PostgresPool) -> Self {
        CustomerAccountRepository {
            db_connection: connection,
        }
    }
}

impl HandleCustomerAccounts for CustomerAccountRepository {
    fn get_customer_account_by_id(&self, id: Uuid) -> Result<CustomerAccount, String> {
        unimplemented!();
    }

    fn get_all_customer_accounts(&self) -> Result<Vec<CustomerAccount>, String> {
        unimplemented!();
    }

    fn create_customer_account(
        &mut self,
        account: NewCustomerAccount,
    ) -> Result<CustomerAccount, String> {
        unimplemented!();
    }

    fn update_customer_account(
        &mut self,
        account: CustomerAccount,
    ) -> Result<CustomerAccount, String> {
        unimplemented!();
    }

    fn delete_customer_account(&mut self, id: Uuid) -> Result<CustomerAccount, String> {
        unimplemented!();
    }
}
