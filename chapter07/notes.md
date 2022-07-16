# Chapter 7: File and storage
## Serialization and deserialization

Demo: `serde-demo`

How to serialize and deserialize Rust values to/from distinct formats? `serde` crate.

This crate provides the `Serialize` and `Deserialize` traits.
```
#[derive(Deserialize)]
struct City {
  name: String,
  population: usize,
  latitude: f64,
  longitude: f64,
}
```

Then, a conversation to JSON can be done this way:
```
let merida = City {
  name: String::from("Mérida"),
  ...
}

let as_json = to_json(&merida).unwrap();
```
Other formats: bincode and CBOR (both are non human-readable formats, but more performant than JSON).

## Encoding bytes to UTF8
`String::from_utf8_lossy()` converts bytes to a string encoded with UTF-8.

## Extracting command line arguments
Demo: `fview`

- Use `std::env::args`
- Extract the nth argument with the `nth` method:
```let arg3 = env::args().nth(3)```
- Use `expect` instead of `unwrap` to be ready against error that may cause a panic: `let file_name = arg3.expect("Error trying to get the file name")`

