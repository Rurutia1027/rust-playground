# Read Write Lock in Rust

In Rust, an `RwLock`(short for read-write-lock) is a concurrency primitive provided by the `std::sync` module that allows multiple readers or one writer to access the protected data at any given time.

## Key Features of RwLock

**Mutual Exclusion for Writers**: If a thread wants to modify the data(write access), it must acquire the lock in a way that prevents all other threads from reading or writing until the write operation is complete.

**Multiple Concurrent Readers**: If multiple threads only need to read the data(read access), they can all acquire the read lock simultaneously.

**Thread Safety**: The `RwLock` ensures safe concurrent access to data without race conditions.

## Examples

This example shows the `RwLock` is wrapping an `Option` that holds the timestamp of the cache update.
This allows threads to

- Read the Last Update Time;
  Multiple threads can simultaneously read the `last_cache_update` value withou blocking each other.

- Update the Timestamp;
  When the cache is updated, one thread can accquire a write lock, blocking all readers and writers until the update is complete.

```rust
// Create an instance of the read&write lock
let last_cache_update:RwLock<Option<DateTime<Utc>>> = RwLock::new(None);

// Reading from the RwLock
let read_guard = last_cache_update.read().unwrap();
if let Some(timestamp) = *read_guard {
    println!("Last update was at : {:?}", timestamp);
}

// Writing to the RwLock
let mut write_guard = last_cache_update.write().unwrap();
*write_guard = Some(Utc::now());
```

## Why Use RwLock ?

- **Concurrency:**:
  It provides safe access to shared data in a multi-threaded environment.

- **Performance**:
  It alllows more efficient concurrent reads compared to a `Mutex`, which only supports exclusive access for both readers and writers.

## Alternatives to RwLock

- `Mutex`: If the data will be mostly written to or if write locks are frequent, a `Mutex` may be simple.
- `Atomic Types`: For simple shared data(like the threads' shared counter or flag), atomic types such as `AtomicUsize` are more efficient.
- `parking_lot's RwLock`: A more performance `RwLock` implementation from the `parking_lot` crate, often preferred in high-performance contexts.
