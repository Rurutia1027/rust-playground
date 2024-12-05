# Notes About Async in Rust

## `async`, `unwrap`, `expect` and `context`

#### `async` for Futures

When we call `.async` on a Future, it pauses the current task until the future completes, yielding the result wrapped inside. If the result is `Result<T, E>`, you often want to extract the success value `(T)`.

```rust

```

#### `unwrap` and `expect`

These are helper methods for extract the inner value from `Future<...>` directly.

- `.unwrap()` is used when you are confident there will be no error. It panics if there is an error.

- `.expect("error message")` this works similarly but allows you to provide a custom error message for debugging purposes.

```rust
// suppose
// fn some_async_function() -> Future<Table> {...}
let table: Table = some_async_function().await.unwrap();

// Or
let table: Table = some_async_function().await.expect("Failed to fetch table");
```

#### Chaining with `?` or `.context()`

In real-world applications, directly calling unwrap isn't always safe. The `?` operator propagates errors up the call stack, while `.context` (from the anyhow or eyre crates) enriches the error with additional information.

```rust
// async fn some_async_function() -> Future<Table>
let table: Table = some_async_functions()
.await
.context("Failed to fetch table");
```

Rust examples often use unwrap or expect because they simplify code for demostration. In production code, propagating errors with `?` providing helpful context is preferred for better error handling.

#### Some Patterns in Real World Codes

**Chaining with `.await`**

In this example, the return type combines transformation, unwrapping, and awaiting into a concise expression.

```rust
let result = async_function()
.await
.map(|val| val.some_transformation())
.unwrap();
```

**Error Handling with `?`**

```rust
let table = async_function().await?.transformation();
```

**Contextual Errors**

```rust
let table = async_function().await.context("Failed to fetch table")?;
```

The frequent pairing of `.await` with `.unwrap`, `.expect` or `.cntext` is a particular way to handle asynchronous results and errors. Over time, as you encounter more Rust projects, you'll see a balance between these patterns based on the project's error-handling strategy and the use of libraries like anyhow.

## Examples

### Basic Result Usage

This example of returning a Result from a async function to handle success or error states. The function `fetch_data` returns a `Result<String, MyError>` indicating success or failure. The result is then matched on the caller side, where we either handle the `Ok` value or deal with the `Err` result.

```rust
use tokio;

#[derive(Debug)]
struct MyError;

async fn fetch_data(url: &str) -> Result<String, MyError> {
    if url == 'https://xxx.com' {
        Ok("Success".to_string())
    } else {
        Err(MyError)
    }
}

#[tokio::main]
async fn main() {
    match fetch_data("https://example.com").await {
        Ok(data) => println!("Fetch data: {}", data),
        Err(_) => println!("Failed to fetch data"),
    }
}
```

### Future with Result Handling

```rust
use tokio;

#[derive(Debug)]
struct MyError;

async fn fetch_data(url: &str) -> Result<String, MyError> {
    // simulate some async behavior
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    if url == "https://xxx.com" {
        Ok("Fetched data successfully".to_string())
    } else {
        Err(MyError)
    }
}

async fn process_data(url: &str) -> Result<String, MyError> {
    match fetch_data(url).await {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}

#[tokio::main]
async fn main() {
    match process_data("https://xxx.com").await {
        Ok(result) => println!("Process success: {}", result),
        Err(_) => println!("Process failed"),
    }
}
```

### Multiple Async Operations with Result and Future

In this example, `get_full_data` asynchronously fetches both user and product data. If either of the `fetch_*` functions fails, the whole operation will fail due to the use of the `?` operation, which propagates errors. This is a very common pattern when calling multiple async functions in paralle.

```rust
use tokio;

#[derive(Debug)]
struct MyError;

async fn fetch_user_data(user_id: u32) -> Result<String, MyError> {
    if user_id == 1 {
        Ok("User Data".to_string())
    } else {
        Err(MyError)
    }
}

async fn fetch_product_data(product_id: u32) -> Result<String, MyError> {
    if product_id == 100 {
        Ok("Product Data:".to_string())
    } else {
        Err(MyError)
    }
}

async fn get_full_data(user_id: u32, product_id: u32) -> Result<(String, String), MyError> {
    // Here we use the ? to ensure either of the two fetch_*
    // functions fail, an Error will be return this logic.
    let user_data = fetch_user_data(user_id).await?;
    let product_data = fetch_product_data(product_id).await?;
    Ok((user_data, product_data))
}

#[tokio::main]
async fn main() {
    match get_full_data(1, 100).await {
        Ok(user, product) => println!("Success: User: {}, Product: {}", user, product),
        Err(_) => println!("Failed to fetch data"),
    }
}
```

### Using context for More Detailed Errors

When we handle async functions, `context` is useful for providing more information when an error occurs. In this example, `context` is used to add a custom error message to the result of `fetch_data`. This is helpful when debugging complex async code where multiple operations may fail, and you want to retain the context of the error.

```rust
use tokio;
use anyhow::{Result, Context};

#[derive(Debug)]
struct MyError;

async fn fetch_data(url: &str) -> Result<String> {
    if url == "https://example.com" {
        Ok("Fetched data".to_string())
    } else {
        Err(anyhow::anyhow("Invalid URL").into())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    fetch_data("https://invalid.com")
        .await
        // if got error, the return data will be interrpted
        // by the context("xxx")? and this operation
        // will get more error context information and messages to the invoker side
        .context("Failed to fetch data from the given URL")?;
    Ok(())
}
```

### Combining Result and Future for Complex Flows

We often combine `Result`, `Future`, and other asynchronous operations to create a more complex product-like system.

```rust
use tokio;
use std::time::Duration;

#[derive(Debug)]
struct MyError;

async fn fetch_data_from_db() -> Result<String, MyError> {
    tokio::time::sleep(Duration::from_secs(2)).await;
    Ok("Database Data".to_string())
}

async fn fetch_data_from_api() -> Result<String, MyError> {
    tokio::time::sleep(Duration::from_secs(3)).await;
    Ok("API Data".to_string())
}

async fn process_data() -> Result<String, MyError> {
    // either fetch_data_from_db or fetch_data_from_api
    // failed, An error type of MyError will be returned
    let db_data = fetch_data_from_db().await?;
    let api_data = fetch_data_from_api().await?;
    Ok(format!("Processed Data: {} + {}", db_data, api_data))
}

#[tokio::main]
async fn main() {
    match process_data().await {
        Ok(data) => println!("xxx"),
        Err(_) => println!("xxx"),
    }
}
```

## Chain Operations using `async` Functions

### Chaining Operations in an Async Context with `then`

In this example, we simulate an async chain of operations using closures to process a value in steps. Each operation might either succeed(returning Ok) or fail(returning Err).

```rust
use tokio;

#[derive(Debug)]
enum MyError {
    InvalidInput,
    DatabaseError
}

async fn fetch_data() -> Result<i32, MyError> {
    println!("Fetching data...");
    Ok(42)
}

async fn process_data(data: i32) -> Result<i32, MyError> {
    if data % 2 == 0 {
        Ok(data * 2)
    } else {
        Err(MyError::InvalidInput)
    }
}

async fn save_to_db(data: i32) -> Result<i32, MyError> {
    if data > 0 {
        Ok(data)
    } else {
        Err(MyError::DatabaseError)
    }
}

#[tokio::main]
async fn main() {
    let result = fetch_data()
    .await
    // here this method allows us to apply
    // the next async operation only if the previous one was successful(i.e., the result is Ok)
    .and_then(|data| async move {
        save_to_db(processed_data).await
    });

    match result {
        Ok(final_data) => println!("..."),
        Err(e) => println!("...{:?}", e),
    }
}
```

### Using `map` and `then-like` Async Operations

```rust
use tokio;

#[derive(Debug)]
enum MyError {
    InvalidInput,
    NotFound
}

async fn fetch_user_by_id(id: i32) -> Result<String, MyError> {
    if id == 44 {
        Ok("User_42".to_string())
    } else {
        Err(MyError::NotFound)
    }
}

async fn validate_user(user: String) -> Result<String, MyError> {
    if user == "User_42" {
        Ok(user)
    } else {
        Err(MyError::InvalidInput)
    }
}

async fn fetch_user_data(user: String) -> Result<String, MyError> {
    Ok(format!("Details of {}", user))
}

#[tokio::main]
async fn main () {
    let result = fetch_user_by_id(42)
        .await
        .map(|user| {
            // this is a transformation, not an error-prone operation
            validate_user(user)
        })
        // and_then in async chains is a combinator that allows you to conditionally execute a closure only if the previous step was successful.
        // if the previous step returns Err, the entire chain stops, and the error is returned immediately. No futher operations are executed.
        // if the previous step is Ok, then the and_then closure is invoked, and it is expected to return a future.
        // then we can await this future to get the result of the operation
        .and_then(|validate_result| async {
            validation_result.await
        })

        // here the `and_then` plays the same role of both
        // filter and operator, filter Err and operate future's await logic
        .and_then(|valid_user| async {
            fetch_user_data(valid_user).await
        });

    match result {
        Ok(user_data) => println!("Success: {}", user_data),
        Err(e) => println!("Error: {:?}", e)
    }
}
```

## Conclusion

The combination of Future and Result in async Rust can get more complex as our work with multiple operations that may fail, especially when integrating with external systems like APIs, databases, or file systems. Some common patterns include:

- Use Result to represent operations that can succeed or fail.
- Combining async functions with await to asynchronously handle the results.
- Using the `?` operator for propagating errors.
- Using `context` or `.map_err` to provide more context to errors.
