use std::error::Error;
use reqwest::blocking::get;

fn main() -> Result<(), Box<dyn Error>> {
  let url = "http://www.rustinaction.com/";
  let response = get(url)?;
  let content = response.text()?;
  println!("{}", content);

  Ok(())
}
