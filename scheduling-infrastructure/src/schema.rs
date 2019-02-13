table! {
    customer_accounts (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        date_registered -> Timestamp,
        date_updated -> Nullable<Timestamp>,
        status -> Bool,
    }
}

table! {
    customer_account_types (id) {
        id -> Uuid,
        customer_account_id -> Uuid,
        account_type -> Varchar,
        hash -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    customer_accounts,
    customer_account_types,
);
