use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
  thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
}

impl File {
  fn new(name: &str) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
    }
  }

  fn new_with_data(name: &str, data: &Vec<u8>) -> File {
    let mut file = File::new(name);
    file.data = data.clone();
    file
  }

  fn read(
    self: &File,
    save_to: &mut Vec<u8>
  ) -> Result<usize, String> {
    let mut tmp = self.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    Ok(read_length)
  }
}

fn open(file: File) -> Result<File, String> {
  if one_in(4) {
    return Err(String::from("Permission denied"));
  }
  Ok(file)
}

fn close(file: File) -> Result<File, String> {
  if one_in(100_000) {
    return Err(String::from("Interrupted by signal"));
  }
  Ok(file)
}


fn main() {
  let mut file = File::new_with_data("f1.txt", &vec![114, 117, 115, 116, 33, 33]);

  let mut buffer: Vec<u8> = vec![];

  file = open(file).unwrap();
  let f1_length = file.read(&mut buffer).unwrap();
  file = close(file).unwrap();

  let text = String::from_utf8_lossy(&buffer);

  println!("{:?}", file);
  println!("{} is {} bytes long", &file.name, &f1_length);
  println!("{}", text);
}
