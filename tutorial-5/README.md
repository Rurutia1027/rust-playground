# Rust in Action

## Implement KVStore Based on PostgresSQL

### Introduction of Key-Value Store

KVStore supports key & value `get_value` and `set_value` operations, where the value is stored in a PostgreSQL table with the name of `key_value_store`. The value type in the table is declared as a serialized JOSN. Operations on JSON strings are handled using the `serde_json` crate.

- Key & Value API code and tests are located in [KeyValueStorePostgres](./src/key_value_store.rs).
- Database and PostgreSQL-related code and test cases are located [db.rs](./src/db.rs).

### Introduction of `sqlx` Usage

`sqlx` is an asynchronous, compile-time checked SQL toolkit for Rust. It provides a way to interact with databases like PostgreSQL, MySQL and SQLite, while ensuring that SQL queries are valid and match the database schema at compile time.
`sqlx` support Rust's async/await syntax for non-blocking database operations, and it also integrates well with serde for serializing and deserializing data between Rust and the database.

#### Key Features of `sqlx`:

- Compile-time: Ensures SQL queries are correct by checking them against the database schema during compile time, preventing runtime errors.
- Async support: Fully async, making it ideal for scalable applications that need to perform non-blocking database queries.
- Supports multiple databases: Works with PostgreSQL, MySQL, and SQLite.
- Type safety: Automatically maps Rust types to SQL types, ensuring the correct mapping of values.

### How to Use `sqlx`

#### Declare DataSource in Cargo.toml

`sqlx` supports multiple datasources like `PostgreSQL`, `MySQL` and `SQLite`.
We need to speicfy the exact type of the data source at the beginning when importing `sqlx` into our project in `Cargo.toml`, as shown below:

```toml
sqlx = { version = "0.7.2", features = [
  # Enable sqlx conversion between Rust types like serde_json::Value and DB tables' json/jsonb types(PostgreSQL)
  "json",


  # Enable sqlx compile & parse PostgreSQL native SQL grammar
  "postgres",


  # Ensure the apps can connect securely to remote databases and services
  "runtime-tokio-native-tls",
] }
```

#### Install `sqlx` on Local Environtment

If we want to use `sqlx` cli tools in local development environment to execute database migraitons and check SQL syntax in the Rust project, we need to intall the `sqlx` CLI tool with the following command:

```shell
cargo install sqlx-cli --no-default-features --features postgres
```

After installation, we can use the following command to check s

```shell
% sqlx
Command-line utility for SQLx, the Rust SQL toolkit.

Usage: sqlx <COMMAND>

Commands:
  database     Group of commands for creating and dropping your database
  prepare      Generate query metadata to support offline compile-time verification
  migrate      Group of commands for creating and running migrations
  completions  Generate shell completions for the specified shell
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Migration Step

Database tables needs to be created before application setup. Migraitons handle both creating tables and loading initial databases, as well as modifying the database schema (e.g., altering, adding new columns). In `sqlx`, all migrations are expressed in SQL files, which are executed sequentially in the [`./migrations`](./migrations/) folder of the project. The SQL files are named using the format ${timestamp-generated-by-sqlx-command}_${name}.sql.

To add a new migration(sql file), use the following command(before this make sure sqlx-cli is installed):

```shell
# this command will generate a [${timestamp}alter_key_value_store_value_column.sql](./migrations/20241129061451_alter_key_value_store_value_column.sql) under the ./migrations folder

cargo sqlx migrate add alter_key_value_store_value_column
```

Next, open the generated migration file and add the necessary SQL commands. To run the migrations, execute the following command:

```shell
# sqlx will load the database URL(DATABASE_URL) from .env file by default.
# But we can also specify the database URL directly:
sqlx migrate run --database-url postgres://admin:admin@localhost:5432/defaultdb
```

#### `run sqlx preapre`

After configuring the `DATABASE_URL` in the `.env` file and successfully running migrations, we can use the `cargo sqlx prepare` command to verify if the SQL operations and table schemas match the expectations.

This command checks the following:

1. Whether all tables in the Rust code exist in the database (as defined by the .env file).
2. Whether the column types declared in the database match the types declared in Rust code.
3. After validation, it generates "mapping files" representing the relationship between the database table types and the Rust types (e.g., `jsonb` in PostgreSQL matched with the `serde_json::Value` in Rust).

```shell
cargo sqlx prepare
```

After executing the command above, we will get sereis of prefix with `query-` files under the folder `./sqlx`, and the above DB table mapping relationships those metadata are stored in the `query-...` files.

## Implement Environment Variables Env

```rust
todo!()
```
