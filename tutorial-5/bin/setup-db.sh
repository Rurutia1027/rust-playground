#!/bin/bash

# Install PostgreSQL if not already installed (optional, if needed)
sudo apt update
sudo apt install -y postgresql postgresql-contrib

# Start PostgreSQL service
sudo service postgresql start

# Create the database if it doesn't exist
DB_NAME="defaultdb"
DB_USER="admin"
DB_PASSWORD="admin"

# Check if database exists, if not, create it
psql -U postgres -tc "SELECT 1 FROM pg_database WHERE datname = '$DB_NAME'" | grep -q 1 || psql -U postgres -c "CREATE DATABASE $DB_NAME;"

# Set the user and password (you can adjust this if needed)
psql -U postgres -c "CREATE USER $DB_USER WITH PASSWORD '$DB_PASSWORD';"
psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;"

# Ensure the key_value_store table exists
PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -d $DB_NAME -c "
CREATE TABLE IF NOT EXISTS key_value_store (
    key VARCHAR(255) PRIMARY KEY,
    value TEXT
);
"

# Print confirmation
echo "Database setup complete, key_value_store table is ready!"