# Chapter 7: File and storage

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

---
`String::from_utf8_lossy()` converts bytes to a string encoded with UTF-8.
