# Chapter 7: Files and storage

## Serialization and deserialization

Demo: `serde-demo`

How to serialize and deserialize Rust values to/from distinct formats? `serde` crate.

This crate provides the `Serialize` and `Deserialize` traits.

```rust
#[derive(Deserialize)]
struct City {
  name: String,
  population: usize,
  latitude: f64,
  longitude: f64,
}
```

Then, a conversation to JSON can be done this way:

```rust
let merida = City {
name: String::from("Mérida"),
//...
}

let as_json = to_json( & merida).unwrap();
```

Other formats: bincode and CBOR (both are non human-readable formats, but more performant than JSON).

## Encoding bytes to UTF-8

`String::from_utf8_lossy()` converts bytes to a string encoded with UTF-8.

## Extracting command line arguments

Demo: `fview`

- Use `std::env::args`
- Extract the nth argument with the `nth` method:
  ```let arg3 = env::args().nth(3)```
- Use `expect` instead of `unwrap` to be ready against error that may cause a
  panic: `let file_name = arg3.expect("Error trying to get the file name")`

## File operations in Rust

We use `std::fs::File` as the key library to work with files.

Methods to create a `File` object:

- `File::open` - Returns an `Err` if the file doesn't exist, otherwise, it returns an `Ok(File)` with the file in
  read-only mode.
- `File::create` - Always returns an `Ok(File)` with an edition cursor available at the beginning of the file.

`std::fs::OpenOptions` is also available for additional control at opening a file. Do we want to enable reading and
writing, creating in case of miss and appending the written content next to the old content? This is the way:

```rust
let file = OpenOptions::new()
.read(true)
.write(true)
.create(true)
.append(true)
.open(path) ?;
```

Notice how the builder pattern is used to set this options before actually opening the file.

To properly work with paths (i.e., to work without having to worry about path separators or other cross-platform
concerns), use the libraries `std::path::Path` and `std::path::PathBuf`.

```rust
let hello_address = PathBuf::from("/tmp/hello.txt")
```

These types also offer some utility methods:

```rust
hello_address.extension() // Returns the extension as a Some, Some("txt") following the previous case
```

Beware both types are not guaranteed to be UTF-8 compliant.

## Conditional compilation

`cfg` attributes can be used to tell the compiler to use or not stuff conditionally on which one is the target OS (or
other concerns such as CPU model or architecture).

Examples:

Stuff that will be compiled for Windows:

```rust
# [cfg(target_os = "windows")]
```

Stuff that will be compiled for any OS different to Windows:

```rust
# [cfg(not(target_os = "windows"))]
```

Notice that compiled files for Windows will have a `.exe` extension.

Apart from `not(...)`, there are other modifier functions to operate iver `cfg` matchers, such as `all(...)`
or `any(...)`.

Examples of conditions for cfg attributes:

- `target_arch`
- `target_os`
- `target_family`
- `target_env`
- `target_endian`,
- `target_pointer_width`
- `target_has_atomic`
- `target_vendor`
- `test`
- `debug_assertions`

## Controling endianess of reading and writing operations

The `byteorder` crate provides utils to read and write following some given endianess, so keeping the same endianess in
both operations will keep our applications comunications with files properly synced.
