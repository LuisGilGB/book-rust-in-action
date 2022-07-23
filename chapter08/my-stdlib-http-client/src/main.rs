use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
  let host = "www.rustinaction.com";
  let port = "80";

  let mut connection = TcpStream::connect(host.to_owned() + ":" + port)?;

  connection.write_all(b"GET / HTTP/1.0")?;
  connection.write_all(b"\r\n")?;

  connection.write_all(b"Host ")?;
  connection.write_all(host.as_bytes())?;
  connection.write_all(b"\r\n\r\n")?;

  std::io::copy(
    &mut connection,
    &mut std::io::stdout()
  )?;

  Ok(())
}
