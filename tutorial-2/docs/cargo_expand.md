# Cargo Expand

## What's cargo expand ?

`cargo expand` is a tool in the Rust ecosystem that allows you to **expand Rust macros** to see what code they generate. It is particularly useful when you are working with **procedural macros, attribute macros,** or even derive macros like `#[derive(Debug)]`.

`cargo expand` helps us understand what happens **under the hood** when macros are expanded during compilation.

## What is cargo expand useful

**Understanding Macro-Generated Code**:
**Debugging Macros**
**Leanring and Exploration**
**Custom Macros Development**

## How to Install && Usage

- Install

```shell
cargo install cargo-expand
```

- Run expand for default target

```shell
cargo expand
```

- Run exapnd for specific binary target

```shell
cargo expand --bin serdewhatnow
```

- Run expand for library target

```shell

cargo expand --lib
```
