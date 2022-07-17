use libactionkv::ActionKV;
use std::collections::HashMap;

#[cfg(target_os = "windows")]
const USAGE: &str = "\
Usage:
  akv_mem.exe FILE get KEY
  akv_mem.exe FILE delete KEY
  akv_mem.exe FILE insert KEY VALUE
  akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "\
Usage:
  akv_mem FILE get KEY
  akv_mem FILE delete KEY
  akv_mem FILE insert KEY VALUE
  akv_mem FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;

fn store_index_on_disk(action_kv_db: &mut ActionKV, index_key: &ByteStr) {
  action_kv_db.index_map.remove(index_key);
  let index_as_bytes = bincode::serialize(&action_kv_db.index_map).unwrap();
  action_kv_db.index_map = HashMap::new();
  action_kv_db.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
  const INDEX_KEY: &ByteStr = b"+index";

  let args: Vec<String> = std::env::args().collect();
  let file_name = args.get(1).expect(&USAGE);
  let action = args.get(2).expect(&USAGE).as_ref();
  let key = args.get(3).expect(&USAGE).as_ref();
  let maybe_value = args.get(4);

  let path = std::path::Path::new(&file_name);
  let mut action_kv_db = ActionKV::open(path).expect("Unable to open file");

  action_kv_db.load().expect("Unable to load data");

  match action {
    "get" => {
      let index_as_bytes = action_kv_db.get(&INDEX_KEY)
        .unwrap()
        .unwrap();

      let decoded_index_map = bincode::deserialize(&index_as_bytes);
      let index_map: HashMap<ByteString, u64> = decoded_index_map.unwrap();

      match index_map.get(key) {
        None => eprintln!("{:?} not found", String::from_utf8_lossy(&key)),
        Some(&index) => {
          let key_value = action_kv_db.get_at(index).unwrap();
          println!("{:?}", String::from_utf8_lossy(&key_value.value.to_owned() as &[u8]))
        }
      }
    },

    "delete" => action_kv_db.delete(key).unwrap(),

    "insert" => {
      let value = maybe_value.expect(&USAGE).as_ref();
      action_kv_db.insert(key, value).unwrap();
      store_index_on_disk(&mut action_kv_db, INDEX_KEY);
    }

    "update" => {
      let value = maybe_value.expect(&USAGE).as_ref();
      action_kv_db.update(key, value).unwrap();
      store_index_on_disk(&mut action_kv_db, INDEX_KEY);
    }

    _ => eprintln!("{}", &USAGE),
  }
}