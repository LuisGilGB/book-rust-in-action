#![allow(unused_variables,dead_code)]

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

#[derive(Debug)]
struct CubeSat {
  id: u64,
}

fn check_status(cubesat: CubeSat) -> CubeSat {
  println!("{:?}: {:?}", cubesat, StatusMessage::Ok);
  cubesat
}

fn main() {
  let sat_a = CubeSat{ id: 0 };
  let sat_b = CubeSat{ id: 1 };

  let sat_c = CubeSat{ id: 2 };

  let sat_a = check_status(sat_a);
  let sat_b = check_status(sat_b);
  let sat_c = check_status(sat_c);
  println!("a: {:?}, b: {:?}, c: {:?}", sat_a, sat_b, sat_c);

  // waiting...
  let sat_a = check_status(sat_a);
  let sat_b = check_status(sat_b);
  let sat_c = check_status(sat_c);
  println!("a: {:?}, b: {:?}, c: {:?}", sat_a, sat_b, sat_c);
}
