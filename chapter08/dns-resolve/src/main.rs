use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use clap::{App, Arg};
use rand;
use trust_dns_client::op::{Message, MessageType, OpCode, Query};
use trust_dns_client::rr::{Name, RecordType};
use trust_dns_client::serialize::binary::{BinEncodable, BinEncoder};

fn main() {
  let app = App::new("dns-resolve")
    .about("A simple to use DNS Server")
    .arg(Arg::with_name("dns-server").short('s').default_value("1.1.1.1"))
    .arg(Arg::with_name("domain-name").required(true))
    .get_matches();

  let domain_name_raw = app.value_of("domain-name").unwrap();
  let domain_name = Name::from_ascii(domain_name_raw).unwrap();

  let dns_server_raw = app.value_of("dns-server").unwrap();
  let dns_server: SocketAddr = format!("{}:53", dns_server_raw).parse().expect("Invalid address!");

  let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
  let mut response_as_bytes: Vec<u8> = vec![0; 512];

  let mut query_message = Message::new();
  query_message
    .set_id(rand::random::<u16>())
    .set_message_type(MessageType::Query)
    .add_query(Query::query(domain_name, RecordType::A))
    .set_op_code(OpCode::Query)
    .set_recursion_desired(true);

  let mut encoder = BinEncoder::new(&mut request_as_bytes);
  query_message.emit(&mut encoder).unwrap();

  let localhost = UdpSocket::bind("0.0.0.0:0")
    .expect("Cannot bind to local socket");
  let timeout = Duration::from_secs(3);
  localhost.set_read_timeout(Some(timeout)).unwrap();
  localhost.set_nonblocking(false).unwrap();

  let _amt = localhost
    .send_to(&request_as_bytes, dns_server)
    .expect("Socket misconfigured");

  let (_amt, _remote) = localhost
    .recv_from(&mut response_as_bytes)
    .expect("Timeout reached");

  let dns_message = Message::from_vec(&response_as_bytes)
    .expect("Unable to parse response");

  let answers = dns_message.answers();

  if answers.len() == 0 {
    println!("No IP addresses were found for this domain name.");
    return;
  }

  for answer in answers {
    if answer.record_type() == RecordType::A {
      let resource = answer.data().expect("Response didn't provide valid data");
      let ip = resource
        .to_ip_addr()
        .expect("Invalid IP received");
      println!("{}", ip.to_string());
    }
  }
}
