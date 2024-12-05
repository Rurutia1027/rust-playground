# Notes for Header Map

In Rust, a HeaderMap is a collection that stores HTTP headers as key-value pairs. The `header::CACHE_CONTROL` and other HTTP headers are used to define and control the behavior of HTTP requests and responses.

## Purpose of HeaderMap

- **Storing HTTP Headers**: It is used to store and manipulate headers for a HTTP request or response in a structured way.
- **Configuring Requests or Responses**:
  > For requests, headers may include authenticaiton tokens, content type, user agent, etc.
  > For responses, headers might specify caching behavior, content disposition, or cookies.
- **Improving Performance or Security**:
  > The `CACHE_CONTROL` header, for example, tells browsers and intermediaries (like CDNs) how to cache the response.
  > Other headers, liek `Authorization`, helps with authentication.

* Example of `HeaderMap` Use:

```rust
let mut header = HeaderMap::new();

headers.insert(
    header::CACHE_CONTROL,
    HeaderValue::from_str(&format!("max-age={}, public", 3600))).unwrap(),
);
```

- `HeaderMap::new()`: Creates a new empty `HeaderMap`.
- `headers.insert(...)`: Adds a key-value pair to the `HeaderMap`.
  > The key is `header::CACHE_CONTROL`, which sets caching rules.
  > The value is constructed using `HeaderValue::from_str` and specifies `max-age=3600` which caches the response for 3600 seconds, and public this indicating the response can be cached by shared caches.

After creating the header map, it then be attached to an HTTP request or response, like this:

```rust
let response = Response::builder()
.status(StatusCode::OK)
.header_map(headers)
.body("Hello World")
.unwrap();
```
