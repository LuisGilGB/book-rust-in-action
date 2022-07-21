# Chapter 8: Networking

## Layered models

In a layered network model, higher levels are more abstract and are agnostic to the implementations of the lower levels.
Lowe levels are unaware of how higher levels use or build data sent or received from them.

TLS (Transport Layer Security) provides encrypted messaging and unencrypted connection. It's built on top of TCP.

Examples of layered models: OSI model, TCP/IP model.

## HTTP requests with reqwest

`reqwest` methods are asynchronous. Key concepts about this: `Fut`, `async` and `await`.

Stuff that `reqwest` does for you:

- Closing HTTP connection.
- Encoding and decoding the byte stream.
- Setting port 80.
- Resolving the IP address for a given url.

## Trait objects

Trait objects are a mechanism to support polymorphism in Rust in a dynamic environment (runtime). In other words, they
allow to share interfaces across multiple types.

What are the differences between **generics** and **trait objects**?

- **Trait objects** enable polymorphism via dynamic dispatch, **generics** enables it via static dispatch.
- **Trait objects** are dynamically sized, so their implementation is based on pointers.
- **Generics** use more disk space with faster runtimes.
- **Trait objects** use less disk space but have a small performance penalty because of the runtime overhead brought by
  one level of indirection.

Trait objects can appear in three forms:

- `&dyn Trait` <- Borrowed and immutable.
- `&mut dyn Trait` <- Borrowed and mutable.
- `Box<dyn Trait>` <- Owned trait object.
