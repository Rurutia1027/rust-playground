# `tokio` in Rust

Tokio is a _runtime for writing asynchronous applications_ in Rust. It provides the essential components to build scalable and efficient applications, such as servers, background tasks, or any system that requires handling many concurrent operations. It's particularly suitable for network programming and IO-bound workloads.

## Role of `try_join!`

`try_join!` in the tokio asynchronous runtime is used to runtime is used to run multiple asynchrnous tasks concurrently. It does not create a collection of thread handlers but instead creates a "logical join" of features(asynchronous computations) into one combined future.

Here is how `try_join!` works:

- **Parallel Execution**: `try_join!` starts all the tasks concurrently. Each task runs independently but within the same async runtime.
- **Error Propagation**: If any task returns an error, `try_join!` stops waiting for other tasks and immediately propagates the error.
- **Completion Requirement**: For the `try_join!` macro to succeed, all tasks must complete successfully. If any one task errors out, the result of the `try_join!` is an error.

### Comparision to ThreadHandlers

With threads in a synchronous context:

- `std::thread::join` blocks the current thread until the spawned thread finishes. If you call join on all threads, the main thread will wait for all threads to finish before continuing or exiting.
- In `try_join!`, it's similar but in the async world: you're waiting for the completion of multiple asynchronous tasks instead of threads.

## Key Features of Tokio

### Essential Runtime: Using `#[tokio::main]` and spawning tasks

```rust
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // start two tasks concurrently
    let t1 = task::spawn(async {
        sleep(Duration::from_secs(2)).await;
    });

    let t2 = task::spawn(async {
        sleep(Duration::from_secs(3)).await;
    });

    // wait for two tasks to complete
    let _ = tokio::try_join!(t1, t2);

    println!("all tasks (t1, t2) completed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_runtime_tasks() {
        // test to ensure both tasks complete as expected
        let t1 = tokio::spawn(async {
            sleep(Duration::from_secs(1)).await;
        });

        let t2 = tokio::spawn(async {
            sleep(Duration::from_secs(2)).await;
        });

        let _ = tokio::try_join!(t1, t2);
    }
}
```

### Concurrency and Parallelism

```rust
use tokio::task;
use tokio::time::{sleep, Duration};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parallel_tasks() {
        let t1 = tokio::spawn(async {
            sleep(Duration::from_secs(2)).await;
            "Task1 Done"
        });

        let t2 = tokio::spawn(async {
            sleep(Duration::from_secs(2)).awiat;
            "Task2 Done"
        });

        let (r1, r2) = tokio::try_join!(t1, t2).unwrap();
        assert_eq!(r1, "Task1 Done");
        assert_eq!(r2, "Task2 Done);
    }
}
```

### Task Scheduling

```rust
use tokio::time::{sleep, Duration};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scheduling() {
        let start = tokio::time::Instant::now();
        sleep(Duratoin::from_secs(1)).await;
        let elapsed = start.elapsed();
        assert!(elapsed.as_secs() >= 1, "timer did not run as expeted");
    }
}
```

### Built-In Utilities

```rust
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn main() {

}

async fn update_counter(counter: Arc<Mutex<i32>>) {
    let mut counter = counter.lock().await;
    *counter += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mutex() {
        // here we use arc(atomic reference) to wrap the variable of Mutex
        // in order to share the mutex value among multiple threads and guarantee read&write thread safe
        let counter = Arc::new(Mutex::new(0));

        let t1 = tokio::spawn(update_counter(counter.clone()));
        let t2 = tokio::spawn(update_counter(counter.clone()));

        let _ = tokio::try_join!(t1, t2);
        let r = *counter.lock().await;
        assert_eq!(r, 2);
    }
}
```

### IO Integration

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_io() {
        let file_name = "test_io.txt";

        // Writing to file
        let mut file = File::create(file_name).await.unwrap();
        file.write_all(b"Hello, Async File IO!").await.unwrap();

        // Reading from file
        let mut file = File::open(file_name).await.unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.unwrap();

        assert_eq!(contents, "Hello, Async File IO!");
    }
}
```
