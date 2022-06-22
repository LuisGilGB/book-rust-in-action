#![allow(unused_variables)]

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
  ) -> usize {
    let mut tmp = self.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
  }
}

fn open(f: &mut File) -> bool {
  true
}

fn close(f: &mut File) -> bool {
  true
}


fn main() {
  let mut f1 = File::new_with_data("f1.txt", &vec![114, 117, 115, 116, 33, 33]);

  let mut buffer: Vec<u8> = vec![];

  open(&mut f1);
  let f1_length = f1.read(&mut buffer);
  close(&mut f1);

  let text = String::from_utf8_lossy(&buffer);

  println!("{:?}", f1);
  println!("{} is {} bytes long", &f1.name, &f1_length);
  println!("{}", text);
}
