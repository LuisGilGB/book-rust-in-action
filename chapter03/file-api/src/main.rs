#![allow(unused_variables)]

static mut ERROR: i32 = 0;

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
  unsafe { ERROR = -1; }
  true
}


fn main() {
  let mut file = File::new_with_data("f1.txt", &vec![114, 117, 115, 116, 33, 33]);

  let mut buffer: Vec<u8> = vec![];

  open(&mut file);
  let f1_length = file.read(&mut buffer);
  unsafe {
    if ERROR != 0 {
      panic!("An error has occurred while reading the file")
    }
  }
  close(&mut file);
  unsafe {
    if ERROR != 0 {
      panic!("An error has occurred while closing the file")
    }
  }

  let text = String::from_utf8_lossy(&buffer);

  println!("{:?}", file);
  println!("{} is {} bytes long", &file.name, &f1_length);
  println!("{}", text);
}
