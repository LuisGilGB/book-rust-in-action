use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, SeekFrom};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::{Crc, CRC_32_CKSUM};
use serde_derive::{Deserialize, Serialize};

type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
  pub key: ByteString,
  pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
  file: File,
  pub index_map: HashMap<ByteString, u64>,
}

impl ActionKV {
  pub fn open(path: &Path) -> io::Result<Self> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .append(true)
      .open(path)?;
    let index_map = HashMap::new();
    Ok(ActionKV { file, index_map })
  }

  fn checksum(data: &Vec<u8>) -> u32 {
    Crc::<u32>::new(&CRC_32_CKSUM).checksum(&data)
  }

  fn process_record<R: Read>(file: &mut R) -> io::Result<KeyValuePair> {
    let saved_checksum = file.read_u32::<LittleEndian>()?;
    let key_len = file.read_u32::<LittleEndian>()?;
    let value_len = file.read_u32::<LittleEndian>()?;
    let data_len: usize = key_len as usize + value_len as usize;

    let mut data = ByteString::with_capacity(data_len);

    {
      file.by_ref()
        .take(data_len as u64)
        .read_to_end(&mut data)?;
    }
    debug_assert_eq!(data.len(), data_len);

    let checksum = ActionKV::checksum(&data);

    if checksum != saved_checksum {
      panic!(
        "data corruption encountered ({:08x} != {:08x})",
        checksum, saved_checksum
      );
    }

    let value = data.split_off(key_len as usize);
    let key = data;

    Ok(KeyValuePair { key, value })
  }

  pub fn seek_to_end(&mut self) -> io::Result<u64> {
    self.file.seek(SeekFrom::End(0))
  }

  pub fn load(&mut self) -> io::Result<()> {
    let mut file = BufReader::new(&mut self.file);

    loop {
      let current_position = file.seek(SeekFrom::Current(0))?;

      let maybe_kv = ActionKV::process_record(&mut file);
      let kv = match maybe_kv {
        Ok(kv) => kv,
        Err(err) => {
          match err.kind() {
            io::ErrorKind::UnexpectedEof => {
              break;
            },
            _ => return Err(err),
          }
        }
      };

      self.index_map.insert(kv.key, current_position);
    };

    Ok(())
  }

  pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
    let position = match self.index_map.get(key) {
      None => return Ok(None),
      Some(position) => *position,
    };

    let kv = self.get_at(position)?;

    Ok(Some(kv.value))
  }

  pub fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
    let mut file = BufReader::new(&mut self.file);
    file.seek(SeekFrom::Start(position))?;
    let kv = ActionKV::process_record(&mut file)?;

    Ok(kv)
  }

  pub fn find(&mut self, target: &ByteStr) -> io::Result<Option<(u64, ByteString)>> {
    let mut file = BufReader::new(&mut self.file);

    let mut found: Option<(u64, ByteString)> = None;

    loop {
      let position = file.seek(SeekFrom::Current(0))?;

      let maybe_kv = ActionKV::process_record(&mut file);
      let kv = match maybe_kv {
        Ok(kv) => kv,
        Err(err) => {
          match err.kind() {
            io::ErrorKind::UnexpectedEof => {
              break;
            },
            _ => return Err(err),
          }
        }
      };

      if kv.key == target {
        found = Some((position, kv.value));
      }
    }

    Ok(found)
  }

  pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
    let position = self.insert_but_ignore_index(key, value)?;

    self.index_map.insert(key.to_vec(), position);
    Ok(())
  }

  pub fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
    let mut file = BufWriter::new(&mut self.file);

    let key_len = key.len();
    let val_len = value.len();
    let mut tmp = ByteString::with_capacity(key_len + val_len);

    for byte in key {
      tmp.push(*byte);
    }

    for byte in value {
      tmp.push(*byte);
    }

    let checksum = ActionKV::checksum(&tmp);

    let next_byte = SeekFrom::End(0);
    let current_position = file.seek(SeekFrom::Current(0))?;
    file.seek(next_byte)?;
    file.write_u32::<LittleEndian>(checksum)?;
    file.write_u32::<LittleEndian>(key_len as u32)?;
    file.write_u32::<LittleEndian>(val_len as u32)?;
    file.write_all(&tmp)?;

    Ok(current_position)
  }

  #[inline]
  pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
    self.insert(key, value)
  }

  #[inline]
  pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
    self.insert(key, b"")
  }
}
