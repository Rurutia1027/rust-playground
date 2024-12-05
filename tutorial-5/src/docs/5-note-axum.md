# Notes for Axum Framework in Rust

## Extension in Axum

`axum::Extension` is a type used in Axum for injecting shared state or other resources into request handlers.
The purpose of `Extension` in axum is it allows handlers to access application-wide shared resources like _database connections_, _caches_, or _configuraitons_ without global variables.

** Just Like How Spring Context Managed Global Shared Variables Singletonly **

`Extension` wraps the shared resource and makes it available as a parameter in the handler function.

## Arc (Atomic Reference Counted)

Arc is short for the **Atomic Reference Counted**. It is a thread-safe, shared ownership smart pointer provided by the Rust standard library. It enables multple threads to own the same data safely.

The purpose of using `Arc` in Rust is when using `Axum`(or other asynchronous web frameworks), resources like `State` might need to accessed concurrently. Wrapping `State` in `Arc` ensures that it can be shared among multple threads without violating Rust's ownership rules.
