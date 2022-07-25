use rand::RngCore;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
    let octets = &self.0;
    write!(
      formatter,
      "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
      octets[0], octets[1], octets[2],
      octets[3], octets[4], octets[5]
    )
  }
}

impl MacAddress {
  fn new() -> Self {
    let mut octets: [u8; 6] = [0; 6];
    rand::thread_rng().fill_bytes(&mut octets);
    octets[0] |= 0b_0000_0011;
    MacAddress { 0: octets }
  }

  fn is_local(&self) -> bool {
    (self.0[0] & 0b_0000_0010) == 0b_0000_0010
  }

  fn is_unicast(&self) -> bool {
    (self.0[0] & 0b_0000_0001) == 0b_0000_0001
  }
}

fn main() {
  let mac = MacAddress::new();
  assert!(mac.is_local());
  assert!(mac.is_unicast());
  println!("MAC: {}", mac);
}
