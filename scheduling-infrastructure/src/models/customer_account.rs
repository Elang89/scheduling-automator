#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::customer_accounts;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable)]
pub struct CustomerAccount {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_registered: NaiveDateTime,
    pub date_updated: Option<NaiveDateTime>,
    pub status: bool,
}

#[derive(Debug, Insertable, Clone)]
#[table_name = "customer_accounts"]
pub struct NewCustomerAccount {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
