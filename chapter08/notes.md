# Chapter 8: Networking

## Some Cargo notes

```shell
cargo run -q -- an-argument
```

- `-q` option mutes any intermediate output.
- `--` passes arguments on the right to the executable build by the operation on the left.

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

Sneak peak of a Rust concept: Trait objects are a form of *type erasure*.

### Trait vs type

Trait objects and type parameters may seem similar; but they are used in different places.

Example:

```rust
use rand::Rng;
use rand::rngs::ThreadRng;
```

In this example, `Rng` is a trait and `ThreadRng` is a struct. This means:

- If a function argument is defined as `&dyn Rng`, that argument is a reference of something that implements the `Rng`
  trait.
- If a function argument is defined as `&ThreadRng`, that argument is a reference of a value of `ThreadRng` type.

Common use cases for trait objects:

- Creating collections of heterogeneous objects.
- Returning a value tha might be of one of multiple types.
- Supporting dynamic dispatch (remember that trait objects enable polymorphism at runtime).

Any classical OOP concept that is close to trait objects? Perhaps, mixins.

## Error handling

Until now, we have resolved simple error scenarios with the question mark (`?`) operator. This operator is no more than
syntactic sugar for the `try!` macro. This, as it's what `try!` implements, means that:

- If the expression returns `Ok(value)`, the macro returns `value`.
- If the expression returns `Err(err)`, the macro returns `err` converted to the error type defined in the caller
  function. Tha caller function also returns early.

This implementation has the drawback of `try!`/`?` not being proper for a function that might return errors of different
types (with ones not being able to convert to others). Fortunately, there are some ways to tackle this.

```rust
fn main() -> Result<(), std::io::Error> {
  let _file = File::open("input.txt")?;

  let _localhost = "::1"
    .parse::<Ipv6Addr>()?;

  Ok(())
}
```

`File::open` might return a `std::io::Error`, the same type of error found in `main`'s signature, but `parse` would
return `std::net::AddrParseError` instead, an error type which doesn't implement the `From<AddrParseError>` trait (
unless we might do if that is a possibility). As a consequence, this code would not compile.

A valid strategy to allow this is defining the error type that might return the function dynamically by using trait
objects. In other words, the error result should be `Box<dyn Error>` (notice every error type would have implemented
the `Error` trait):

```rust
fn main() -> Result<(), Box<dyn Error>> {
  // Same function body as the previous block
}
```

This function would actually compile, although bringing a small runtime penalty. Also, reference of the original error
type might be lost outside the caller function.

Another strategy would be to define our own error type that includes the possible errors as values of an enum and
implementing the `std::convert::From` for all those error types to our custom one:

```rust
use std::fs::File;
use std::fmt;
use std::io;
use std::net;
use std::error;

#[derive(Debug)]
enum UpstreamError {
  IO(io::Error),
  Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "{:?}", self)
  }
}

impl error::Error for UpstreamError {}

impl From<io::Error> for UpstreamError {
  fn from(error: io::Error) -> Self {
    UpstreamError::IO(error)
  }
}

impl From<net::AddrParseError> for UpstreamError {
  fn from(error: net::AddrParseError) -> Self {
    UpstreamError::Parsing(error)
  }
}

fn main() -> Result<(), UpstreamError> {
  let _file = File::open("input.txt")?;
  let _localhost = "::1".parse::<net::Ipv6Addr>()?;

  Ok(())
}
```

Panicking with the use of `expect` or `unwrap` is also a possibility, but it's a discouraged solution for libraries as
it removes every possibility of control to the consumer.

## TCP

Some stuff about the TCP protocol:

- A new line is usually expressed as `"\r\n"`.
- Two blank new lines (`"\r\n\r\n"`) signify end of the request.

A `&str` defined as `b""` is actually a buffer of bytes.
