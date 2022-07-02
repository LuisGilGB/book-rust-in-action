#![allow(unused_variables, dead_code)]

#[derive(Debug)]
struct Message {
  to: u64,
  content: String,
}

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

#[derive(Debug)]
struct Mailbox {
  messages: Vec<Message>,
}

impl Mailbox {
  fn post(&mut self, message: Message) {
    self.messages.push(message)
  }
  fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
    for i in 0..self.messages.len() {
      if self.messages[i].to == recipient.id {
        let message = self.messages.remove(i);
        return Some(message);
      }
    }
    None
  }
}

#[derive(Debug)]
struct CubeSat {
  id: u64,
}

impl CubeSat {
  fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
    mailbox.deliver(&self)
  }
}

struct GroundStation;

impl GroundStation {
  fn connect(&self, sat_id: u64) -> CubeSat {
    CubeSat { id: sat_id }
  }
  fn send(
    &self,
    mailbox: &mut Mailbox,
    msg: Message,
  ) {
    mailbox.post(msg);
  }
}

fn fetch_sat_ids() -> Vec<u64> {
  vec![1,2,3]
}

fn main() {
  let mut mailbox = Mailbox { messages: vec![] };

  let base = GroundStation {};
  let sat_ids = fetch_sat_ids();

  for sat_id in sat_ids {
    base.connect(sat_id);
    let message = Message { to: sat_id, content: String::from("Beep... beep...") };

    base.send(&mut mailbox, message);
  }

  let sat_ids = fetch_sat_ids();
  for sat_id in sat_ids {
    let sat = base.connect(sat_id);
    let message = sat.recv(&mut mailbox);
    println!("{:?}: {:?}", &sat, &message);
  }
}
