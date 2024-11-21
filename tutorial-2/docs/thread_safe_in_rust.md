# Note about Thread Safety in Rust 

Rust’s approach to thread safety combines ownership, borrowing, and type system guarantees to enforce safety at compile time, without relying on runtime checks. Here’s how it works and how it compares to Java:

## Understanding Threads in Rust

Creating Threads: Rust uses std::thread::spawn to create new threads. Closures passed to spawn contain the logic executed by the thread.
Capturing Variables:
move Keyword: Captures variables from the surrounding environment by value. Ensures safe ownership transfer into the thread’s closure.
Captured variables are moved into the thread’s closure, avoiding issues with dangling references.

## Sharing Data Across Threads

To share data across threads in Rust, you must ensure thread safety using Arc and Mutex:

### Arc (Atomic Reference Counting)
* Purpose: Allows multiple ownership of heap-allocated data by keeping a thread-safe reference count.
* Behavior:
> Cloning an Arc only clones the reference, not the actual data.
> All clones share ownership of the same data.
* Use Case: For read-only or synchronized mutable access to shared data.

### Mutex (Mutual Exclusion)
Purpose: Allows safe, mutable access to shared data by ensuring that only one thread can access the data at a time.
Behavior:
lock(): Grants a thread-exclusive, mutable reference to the data. If another thread holds the lock, the current thread blocks until the lock is released.
Prevents data races by enforcing sequential access.
Use Case: For synchronized writes to shared data.

## Use Atomic Variables or Mutex to Protect, When It Is A Multiple Threads Shared Variable
* Regular Variable used in Multi-Threads will cause compile Error 
```rust 
let mut x = 4;

for _ in 0..5 {
    std::thread::spawn(move || {
        x += 1; // ERROR: cannot mutate x inside the closure
    });
}
```

* Use Atomic Type of Variable 
```rust 
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

let x = Arc::new(AtomicUsize::new(4));

for _ in 0..5 {
    let x = Arc::clone(&x);
    thread::spawn(move || {
        x.fetch_add(1, Ordering::SeqCst); // Atomically increment x
    });
}
```

* Wrap the data with Mutex for exclusive access.

But this is different from volative in Java, it more like the syncornized or Lock in java.
The mutex is a lock(read/write depends on the variable we declare is immutable or mutable).
And the `clone` instances are not the data instance either, instead they are the cloned (mutable/immutable)
references point to the heap-data's heap address. 

Eventhough it is said in Rust, mutable references are not allowed to create multiple times, and also create mutable reference and immutable references are now allowed in Rust at the same time. But they are the rules for regular heap-located objects, 
we can create multiple mutable or immutable references it is because: the heap-located instance is still protected by the `Mute`. With the help of `mute` actually the lock, even though the heap-located data is pointed by multiple references, but `mute` will protect the heap-data be get accessed only by one thread per time.
In this way, `mute` avoid the data race, or dirty read or dirty write such kind of mess.  


```rust
use std::sync::{Arc, Mutex};
use std::thread;

let x = Arc::new(Mutex::new(4));

for _ in 0..5 {
    let x = Arc::clone(&x); // Clone Arc for thread-safe shared ownership
    thread::spawn(move || {
        let mut num = x.lock().unwrap(); // Lock the mutex
        *num += 1; // Safely modify the value
    });
}
```

* Wrap the Mutex with Arc for shared ownership (like Java's Atomic Variables)

```rust 
use std::sync::{Arc, Mutex};
use std::thread;

let shared_data = Arc::new(Mutex::new(0));

let handles: Vec<_> = (0..5)
    .map(|_| {
        let shared_data = Arc::clone(&shared_data);
        thread::spawn(move || {
            let mut num = shared_data.lock().unwrap(); // Lock the Mutex
            *num += 1; // Safely modify the data
        })
    })
    .collect();

for handle in handles {
    handle.join().unwrap();
}

println!("Final value: {}", *shared_data.lock().unwrap());
```           

## How It Works
* Arc::clone():
* Creates multiple references to the same heap-allocated Mutex.
* Mutex::lock():
* Grants exclusive access to the data while ensuring no other thread can access it simultaneously.
* Thread-Safety:
* Prevents data races by allowing only one thread to modify the data at a time.

### Comparison to Java
* Rust’s Mutex vs. Java’s synchronized or volatile:
* Java’s volatile ensures memory visibility but does not enforce mutual exclusion.
* Rust’s Mutex enforces both memory visibility and mutual exclusion, ensuring complete safety.
* Arc and Reference Counting:
* Similar to Java’s AtomicReference but safer due to Rust’s ownership model.