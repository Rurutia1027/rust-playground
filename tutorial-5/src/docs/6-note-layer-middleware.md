# Notes for Axum: Global State Sharing and Middleware Design

In Axum, global state sharing and middleware layers allow for building scalable and thread-safe applications.

## Sharing Global State

Axum's `Extension` middleware is used to inject global state into route handlers. The state is typically structured as a single struct which holds shared resources in this project such as:
**Dataabse**/db_pool
**Health Monitor Handler**/health
**Cache System**/cache

Since these resources need to be accessed across multiple threads in a thread-safe manner, they are wrapped in an `Arc(Atomic Reference Counted)` type.

```rust
let shared_state = Arc::new(State {
    cache,
    db_pool,
    health,
});
```

This `shared_state` is then passed to the Axum application using the `Extension` layer:

```rust
app.layer(Extension(shared_state));
```

## Using Extension Middleware

`Extension` is acomponent defined in the scope of rust package of `axum`, and it allows shared data to be passed to route handlers without manual lookups. For example, we can use it like this:

```rust
let app = Router::new)(
    .route(
        "/api/v2/fees/average-eth-price",
        get(|state: StateExtension| async move) {
            cached_get(state, &CacheKey::AverageEthPrice).await
        }
    ),
).layer(Extension(shared_state));
```

- **Handler Injection**: The `StateExtension(a type alais for Extension<Arc<State>>)` is automatically injected into the route handler as a parameter.

- **Thread Safety**: Each thread working on a request gets its own `Arc` reference, maintaining safety and minimizing overhead.

This design eliminates biolerplate code, such as repeatedly fetching state from a context or global variable, and enforces type safety.

## Comparision to SpringContext

In frameworks like Spring, global states are accessed via depenency injection(DI) and the application context, e.g., `context.getBean('dbPool')`. Axum achieves a similar effect with `Extension`, but:

- Injection is type-safe and compile-time validated.
- No explicit lookups are needed in the handler; the required state is passed automatically.

## Full Lifecycle

When the application receives a request for `/api/v2/fees/average-eth-price`:

- Axum matches the URL path to the route and identifies the handler.
- The `Extension` middleware clones the shared_state for this thread.

## Middleware Layers

Axum's middleware stack, powered by the Tower crate, enables chaining of functional transformations and utilities.

```rust
ServiceBuilder::new()
.layer(middleware::from_fn(etag_middleware::middleware_fn))
.layer(CompressionLayer::new())
.layer(Extension(shared_state));
```

- `ServiceBuilder`: Constructs a stack of middleware layers.
- `etag_middleware`: Adds caching behavior based on ETag header.
- `Compressionlayer`: This compresses HTTP responses to optimize bandwidth.
- `Extension(shared_state)`: Injects the global state for use in route heandlers.

## Conclusion

The combination of Extension for state sharing and the layered middleware system enables Axum applications to deliver scalable, maintainable, and efficient RESTful APIs. This design is especially stuitable for multi-threaded environments, where resources must be shared securely and efficiently.

---

# Thread's Closure && Axum Handler's Closure Association Notes

The axum's closure patterns is similar with `thread::spawn`, but the `axum::Router` definition serve different purposes, although they are some conceptual similarities due to their functional nature.

## Similarities

Both thread & axum use closures to encapsulate logic:

- `thread::spawn`: A closure is passed to spawn a new thread and execute code concurrently.
- `axum::Router handler`: A closure(or async block) defines the logic for handling a specific route when a request matches.

## Key Differences

- **Thread Safety**:

* In `thread::spawn`, closures often interact with shared state. Using `move` ensures ownership of the captured variables is transferred into the new thread.

* In Axum, the route handler often works with shared state injected via middleware like `Extension`, which guarantees thread safety (e.g., through Arc).

- **Async Context**:

* `thread::spawn` runs a synchronous closure in a separate thread. You don't typically use async in this context unless combined with async runtimes like Tokio(e.g., tokio::spawn).
* In Axum, handlers are inherently async to handle I/O-bound tasks like database queries or network calls without blocking.

## If No Shared Variables Are Needed

If we do not need to pass any shared global variables liek `state`, we can simplify the logic:

- For `thread::spawn`

```rust
thread::spawn(|| {
    ...code...
})
```

- For `axum`

In Axum we can also write a route handler that does not depend on shared state. However, if our logic involves async operations (e.g., database queries), we must use async because Axum is built around asynchronous handlers.

```rust
let app = Router::new().route("/ping", get(|| async {"pong"}));
```

## When to Use Async

- We are suggested to use `async` in Axum whenever:
  > We perform asynchronous operations like reading from a database, making HTTP requests, or accessing a cache.
  > We want to benefit from non-blocking concurrency provided by Rust's async runtime.

## Conclusion

For Axum:

- Use async if our route handler involves any asynchronous tasks.
- Avoid using shared state if it's unnecessary. The handler can operate independently.

For `thread::spawn`:

- it is just the multi-threads operation in concurrent, but it is not non-blocking(asynchronously) if we do not use key word `async`
- if no shared variables passing to the closure, we just omitting the key word `move` is ok.
