//! Simulating files one step at a time.

use std::fmt;
use std::fmt::{Display};

trait Read {
  fn read(
    self: &Self,
    save_to: &mut Vec<u8>,
  ) -> Result<usize, String>;
}

#[derive(Debug, PartialEq)]
pub enum FileState {
  Open,
  Closed,
}

/// Represents a "file",
/// which probably lives in a file system.
#[derive(Debug)]
pub struct File {
  name: String,
  data: Vec<u8>,
  pub state: FileState,
}

impl Display for FileState {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> std::fmt::Result {
    match *self {
      FileState::Open => write!(formatter, "Open"),
      FileState::Closed => write!(formatter, "Closed"),
    }
  }
}

impl Display for File {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> std::fmt::Result {
    write!(formatter, "<{} ({})>", self.name, self.state)
  }
}

impl Read for File {
  fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
    if self.state != FileState::Open {
      return Err(String::from("File must be open for reading"));
    }
    let mut tmp = self.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    Ok(read_length)
  }
}

impl File {
  /// Creates a new, empty `File`.
  ///
  /// # Examples
  ///
  /// ```
  /// let f = File::new("f1.txt");
  /// ```
  pub fn new(name: &str) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
      state: FileState::Closed,
    }
  }

  /// Returns the file's length in bytes.
  pub fn len(&self) -> usize {
    self.data.len()
  }

  /// Returns the file's name.
  pub fn name(&self) -> String {
    self.name.clone()
  }
}

fn open(mut file: File) -> Result<File, String> {
  file.state = FileState::Open;
  Ok(file)
}

fn close(mut file: File) -> Result<File, String> {
  file.state = FileState::Closed;
  Ok(file)
}


fn main() {
  let mut file = File::new("new-file.txt");

  let mut buffer: Vec<u8> = vec![];

  if file.read(&mut buffer).is_err() {
    println!("Error checking is working");
  }

  file = open(file).unwrap();
  file.read(&mut buffer).unwrap();
  file = close(file).unwrap();

  let text = String::from_utf8_lossy(&buffer);

  let file_name = file.name();
  let file_length = file.len();

  println!("{:?}", file);
  println!("{}", file);
  println!("{} is {} bytes long", file_name, file_length);
  println!("{}", text);
}
