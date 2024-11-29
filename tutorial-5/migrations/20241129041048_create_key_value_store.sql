-- Add migration script here
CREATE TABLE key_value_store (
    key VARCHAR(255) PRIMARY KEY,
    value TEXT NOT NULL
);