# Stream in Rust

## Stream Pinning with `dyn` and `Box` Explanation

In Rust, streams can often be pinned, especially when we work with trait objects like `dyn Stream`.
Pinning ensures that the stream is not moved in the memory after it's been pinned.
We often use `Pin` in combination with `Box` for heap allocation when dealing with dynamic dispatch. This essential for
trait objects that can be used in async contexts.

```rust
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

// here we define a custom stream implementation using Pin and Box 
struct MyStream {
    count: usize,
    max: usize,
}

impl Stream for MyStream {
    type Item = usize;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.count < self.max {
            self.count += 1;
            Poll::Ready(Some(self.count))
        } else {
            Poll::Ready(None)
        }
    }
}

async fn process_stream(stream: Pin<Box<dyn Stream<Item=usize>>>) -> i32 {
    let mut sum = 0;
    let mut stream = stream;
    while let Some(value) = stream.next().await {
        sum += value;
    }

    sum
}

#[tokio::test]
async fn test_stream_with_dyn_and_pin() {
    let my_stream = MyStream { count: 0, max: 5 };
    let stream = Box::pin(my_stream); // Pinning MyStream as a dyn Stream 
    let sum = process_stream(stream).await;
    assert!(sum > 0);
}
```

### Explanation

- `MyStream`: A custom stream that returns numbers from 1 to max as items.
- `Pin<Box<dyn Stream>>`: We use `Pin<Box<dyn Stream<Item = usize>>> to dynamically allocate `MyStream` on the heap and
  pin it. This allows us to use dynamic dispatch (`dyn Stream`), enabling polymorphic behavior, while ensuring the
  stream is pinned in memory.
- In the `process_stream` function, we are taking pinned `Box<dyn Stream>` and processing it just like any other stream.

### Why Pinning is Necessary in Streaming Manipulation

In Rust, async takes are often implemented with pinned memory. A pinned object cannot be moved after it's been pinned
because moving an async task could invalidate the references that are being awaited. Therefore, we use `Pin` to ensure
that objects are not moved during their lifecycle, ensuring memory safety when dealing with async operations or streams.

---

## Explain `Pin<Box<dyn Stream<Item=usize>>>` in More Detail

- `dyn Stream<Item = uszie>`
    - This represents a dynamic trait object that implements the `Stream` this **trait**, where the item yielded by the
      stream is type of `usize`.
    - `Stream` is a trait in Rust, representing a series of asynchronous values (i.e., items) that can be streamed over
      time.
    - The `dyn` keyword indicates a **trait object**, meaning that you are working with a type that implements the
      `Stream` trait, but you don't know (or care) about the exact type at compile time. Instead, it's dynamically
      dispatched at run time.
- `Box<dyn Stream<Item=usize> + Send>`
    - `Box<T>` is a heap-allocated pointer in Rust. It is used the size of the type is not known at compile time, or
      when you need to transfer ownership of a value without copying it.
    - Here, `Box<dyn Stream>` allows us to store the stream in heap memory and treat it as a "pointer to the trait
      object". This is important because streams can have different implementations behind a uniform interface.
    - `Send` means that this stream can be safely transferred across threads. This is important because we are working
      with async code, and streams can be awaited and executed across different threads.

- `Pin<Box<dyn Stream<Item = uszie> + Send>`
    - `Pin<T>` ensures that the value inside it is not moved in memory. THis is crucial for types that involve
      self-referential or recursive structure, such as `Stream` and `Future` types, because they might need to ensure
      that their memory location stays fixed.
    - Streams(and futures) can be dynamic sizes, and their internal state can change over time. In many cases, this is
      fine, but `Pin` guarantees that the object's memory location is `pinned(fixed)` and will not be moved around
      during its lifetime. This is necessary because moving the memory of a stream or future could invalidate pointers
      or references to its internal state.

---

## Reference

- [A Guided Tour of Streams in Rust](https://www.qovery.com/blog/a-guided-tour-of-streams-in-rust/)
