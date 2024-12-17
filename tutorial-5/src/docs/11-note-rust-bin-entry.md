# Rust's Binary Entry Point System

Rust projects allow us to declare multiple binary entry points for a single crate. This enables modular design where different binaries can perform separate tasks or be executed in sequence.

## Understand

- `main` function:

* This is the default entry point for any Rust binary.
* The main function serves as the execution start point of your application.
* Any initialization of **configuration**, **database migraiton**, or **pre-required services** should happen here.

- Multiple Binaries (`bin/folder`):
  By default, Rust looks for binarys under src/main.rs file. However, if we need **multiple entry points**, we can define them under the `bin/` this directory. Each file under `bin/` acts as an **independent binary**, with its own `main` function.

- Cargo.toml Declaration:
  Rust allows us to declare binaries explicitly in the `Cargo.toml`. However, files placed under the `bin/` directory are automatically detected without needing manual declaration.

--

## Sequential Entry Points

In projects where we need to run **different binaries in sequence**, you can design the primary **main** function(or an orchestrator binary) to execute them binaries one by one.

Here is an example:

- First, we use a **primary entry point**(e.g., src/main.rs) to handle setup(e.g., **database initialization**, **database migration**, **config loading**, etc).
- ## After everything setup and get ready, continue **call other binaries** which each main entry point is located under folder `bin/` to perform specific task(sync data, scheduler, long-running monitoring).

## Code Example

- Project Structure

```shell
my_project/
├── Cargo.toml
├── src/
│   └── main.rs          # Default entry point
└── bin/
    ├── task-one.rs      # Private binary entry point 1
    └── task-two.rs      # Private binary entry point 2
```

#### Cargo toml

```toml
[package]
name = "your_project_name"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "server"
path = "src/bin/server.rs"

[dependencies]
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.6", features = ["runtime-tokio", "postgres"] }
serde = { version = "1", features = ["derive"] }
```

#### Primary Entry Point (src/main.rs)

The main function initializes all required dependencies (DB, configs, etc.) and then triggers private binaries.

```rust
use std::process::Command;

async fn main() {
    println!("Starting main setup ...");

    // step 1: initialize databases, do migrations and configuration
    initialize_database();
    db_migraiton();

    println!("Setup complete. Begin to execute private entry points ...");

    loop {
        // setup server
        // listen on port 3000
        // match request {
        //   client_query_db_data : {
        //       fetch_data_from_db <-- this db table's data are fetched and stored by task-one
        //  }
        //   client_query_streaming_system_data: {
        //       fetch_data_from_streaming_event_sys: <--- this streaming events are monitored and broadcasted by task-two
        // }
        // ...
        // }
    }

    println!("All tasks completed successfully.");
}

fn initialize_database() {
    // ...
}

fn db_migration() {
    // ...
}

fn load_configuration() {
    // ...
}
```

#### Private Entry Point 1 (bin/task_one.rs)

This binary entry point is **Task1** logic, such as fetching data from remote **API Endpoints** and sync to database tables.

```rust
// bin/task-one.rs
async fn main() {
    // first, task-one need to loaded all database tables data
    //        --> all database tables are already migrated in the --> server.rs's main etnry point
    // then, task-one begins to fetch data && conver data model && sync to database tables

    loop {
        // all mocked codes...
        let dataset = fetch_data(url).filter().map().iter().collect();
        db_handler.save(db_config, "table-1", dataset).await?;
        sleep(1000ms);
    }
}
```

#### Private Entry Point 2 (bin/task_two.rs)

This binary entry point is **Task2** logic, such as register && monitoring remote listener specified event and broadcast the events to system streaming system.

```rust
// bin/task-two.rs
async fn main() {
    // first, task-two need to query latest data value from datbase table
    // ---> all datasets and status are migrated via migrated files by the primary
    //      server.rs's main() function since it is the first setup process
    // then, task-two begin setup service that listens to remote event and subscribe specified for example transaction events
    // and then broadcast the event to flink system as a streaming tasks (just for example...)

    loop {
        // listen to remote event system
        // trigger event
        // convet event to system inner-defined data structure
        // sync to event database
        // send/broadcast event to streaming system which is subscribed by other services
        flink.submit("streaming-task", dataset);
    }
}
```

--

## How to Run the Project

### Run the Primary Entry Point:

```shell
# build and run the server binary
# server's fn main() will get everything ready: {update database tables schemas, and load the initial datasets to datbase tables, and even load all environment varriables to local env config instance which gonna be shared among other entry point functions.}
cargo build --release --bin server
```

### Run Individual Tasks

Even though we haven't declare the bin entry point for task-one and task-two's main function.
Their entry point names are their bin/{file-name}.rs filename

```shell
cargo build --release --bin task-one
cargo build --release --bin task-two
```

---

## Benefits of Organizing Project Like This

- **Separation of Concerns**: Each binary focuses on a single responsibility.
- **Isolation**: Async runtimes and resources are independently in each binary.
- **Scalability**: Individual binarys can be built, tested, and deployed independently.
- **Parallel Execution**: Multiple binaries can run concurrently, benefiting multi-threaded systems.
