# Serde Topic in Rust

## What is **Serde**?

Serde is a library that sort of serialization and deserialzation.
The goal of serde is not actually to provide an particular serialization and deserialization format.
Instead, it's goal is to provide the infrastructure for doing serializaiton and desdeerialization especially for Rust datastructures.

Serde has a bunch of concetps that are really useful:

#### **Serde Data Model**

Serde has a data model which consists of three parts:

- data type:
  - this is the data type that rust uses (serialize, deserialize)
  - visitor mode
- ## serde data model,
  - the goal for serde data model is trying to provide the mapping relationship between the **data type** and **data format**.
  - **serde data model** trying to provide an encapsulation, so that there is a separation between the **data type** and the **data format**.
- data format:
  - (serializer, deserializer)

---

---

## References

- [Decrusting the serde crate](https://www.youtube.com/watch?v=BI_bHCGRgMY&t=17s)
- [serde.rs](https://serde.rs/)
- [serde doc](https://docs.rs/serde)
