use super::HandleCustomerAccounts;
use crate::models::customer_account::{CustomerAccount, NewCustomerAccount};
use chrono::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct Mock {
    customer_accounts: HashMap<Uuid, CustomerAccount>,
}

macro_rules! insert_customer_account  {
    { $( $e:expr, )+ } => {{
        let mut map = HashMap::new();
        $(
            map.insert($e.id, $e);
        )+
        map
    }};
}

impl Mock {
    pub fn new() -> Self {
        let customer_accounts = insert_customer_account![
            CustomerAccount {
                id: Uuid::parse_str("4c1457e7-13e1-46f2-8406-b25e7759308a").unwrap(),
                first_name: "John".into(),
                last_name: "Doe".into(),
                email: "jd@gmail.com".into(),
                date_updated: None,
                date_registered: Utc::now().naive_utc(),
                status: false,
            },
            CustomerAccount {
                id: Uuid::parse_str("be3f12b2-4d51-4246-835c-c4492dbacecf").unwrap(),
                first_name: "Jane".into(),
                last_name: "Wilson".into(),
                email: "wilson@gmail.com".into(),
                date_updated: None,
                date_registered: Utc::now().naive_utc(),
                status: true,
            },
            CustomerAccount {
                id: Uuid::parse_str("03d7b87f-679f-4384-bd94-83e870874bc7").unwrap(),
                first_name: "Bob".into(),
                last_name: "Roberts".into(),
                email: "bob@gmail.com".into(),
                date_updated: None,
                date_registered: Utc::now().naive_utc(),
                status: false,
            },
            CustomerAccount {
                id: Uuid::parse_str("29d8ce2c-fa2a-4659-88b5-c6bd929d2bad").unwrap(),
                first_name: "William".into(),
                last_name: "Jones".into(),
                email: "jones@gmail.com".into(),
                date_updated: None,
                date_registered: Utc::now().naive_utc(),
                status: false,
            },
        ];

        Mock { customer_accounts }
    }
}

impl HandleCustomerAccounts for Mock {
    fn get_customer_account_by_id(&self, id: Uuid) -> Result<CustomerAccount, String> {
        Ok(self
            .customer_accounts
            .get(&id)
            .map(Clone::clone)
            .expect("Could not find customer account"))
    }

    fn get_all_customer_accounts(&self) -> Result<Vec<CustomerAccount>, String> {
        Ok(self
            .customer_accounts
            .iter()
            .map(|(_, customer_account)| customer_account.clone())
            .collect())
    }

    fn create_customer_account(
        &mut self,
        account: NewCustomerAccount,
    ) -> Result<CustomerAccount, String> {
        let new_account = CustomerAccount {
            id: Uuid::new_v4(),
            first_name: account.first_name,
            last_name: account.last_name,
            email: account.email,
            date_registered: Utc::now().naive_utc(),
            date_updated: None,
            status: false,
        };

        self.customer_accounts
            .insert(new_account.id, new_account.clone());

        Ok(new_account)
    }

    fn update_customer_account(
        &mut self,
        account: CustomerAccount,
    ) -> Result<CustomerAccount, String> {
        let mut new_account = self
            .customer_accounts
            .get(&account.id)
            .map(Clone::clone)
            .unwrap();

        new_account.id = account.id;
        new_account.first_name = account.first_name;
        new_account.last_name = account.last_name;
        new_account.email = account.email;
        new_account.date_registered = account.date_registered;
        new_account.date_updated = Some(Utc::now().naive_utc());
        new_account.status = account.status;

        Ok(new_account)
    }

    fn delete_customer_account(&mut self, id: Uuid) -> Result<CustomerAccount, String> {
        let customer_account = self.customer_accounts.get(&id).map(Clone::clone).unwrap();

        self.customer_accounts.remove_entry(&id);

        Ok(customer_account)
    }
}
