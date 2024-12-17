-- Add migration script here
CREATE TABLE IF NOT EXISTS eth_prices (
    timestamp timestamptz PRIMARY KEY,
    ethusd float8 NOT NULL
)