use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_json::to_string as to_json;
use serde_derive::{Serialize};

#[derive(Serialize)]
struct City {
  name: String,
  population: usize,
  latitude: f64,
  longitude: f64,
}

fn main() {
  let merida = City {
    name: String::from("Mérida"),
    population: 59_000,
    latitude: -6.68,
    longitude: 38.95,
  };

  let as_json = to_json(&merida).unwrap();
  let as_cbor = to_cbor(&merida).unwrap();
  let as_bincode = to_bincode(&merida).unwrap();

  println!("json:\n{}\n", &as_json);
  println!("cbor:\n{:?}\n", &as_cbor);
  println!("bincode:\n{:?}\n", &as_bincode);
  println!("json (as UTF-8):\n{}\n", String::from_utf8_lossy(&as_json.as_bytes()));
  println!("cbor (as UTF-8):\n{:?}\n", String::from_utf8_lossy(&as_cbor));
  println!("bincode (as UTF-8):\n{:?}\n", String::from_utf8_lossy(&as_bincode));
}
