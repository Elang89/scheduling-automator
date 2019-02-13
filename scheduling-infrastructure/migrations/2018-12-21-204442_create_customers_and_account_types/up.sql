-- Your SQL pub mod schema;goes here

CREATE TABLE customer_accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(), 
    first_name VARCHAR NOT NULL, 
    last_name VARCHAR NOT NULL, 
    email VARCHAR NOT NULL,  
    date_registered TIMESTAMP NOT NULL DEFAULT NOW(), 
    date_updated TIMESTAMP,
    status BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE customer_account_types (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_account_id UUID NOT NULL, 
    account_type VARCHAR NOT NULL,
    hash VARCHAR NOT NULL 
);