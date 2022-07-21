use std::error::Error;
use std::future::poll_fn;
use reqwest;

async fn do_reqwest() -> Result<(), Box<dyn Error>> {
  let url = "http://www.rustinaction.com/";
  let response = reqwest::get(url).await?;

  let content = response.text().await?;
  println!("{}", content);

  Ok(())
}

fn main() {
  poll_fn(do_reqwest());
}
