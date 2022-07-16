use std::io::Cursor;
use byteorder::{LittleEndian};
use byteorder::{ReadBytesExt, WriteBytesExt};

fn write_numbers_to_file() -> (u32, i8, f64) {
    let mut writer = vec![];

    let one: u32 = 1;
    let two: i8 = 2;
    let three: f64 = 3.0;

    writer.write_u32::<LittleEndian>(one).unwrap();
    println!("{:?}", &writer);
    writer.write_i8(two).unwrap();
    println!("{:?}", &writer);
    writer.write_f64::<LittleEndian>(three).unwrap();
    println!("{:?}", &writer);

    (one, two, three)
}

fn read_numbers_from_file() -> (u32, i8, f64) {
    let mut reader = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
    let one_ = reader.read_u32::<LittleEndian>().unwrap();
    let two_ = reader.read_i8().unwrap();
    let three_ = reader.read_f64::<LittleEndian>().unwrap();

    (one_, two_, three_)
}

fn main() {
    let (one, two, three) = write_numbers_to_file();
    let (one_, two_, three_) = read_numbers_from_file();

    assert_eq!(one, one_);
    assert_eq!(two, two_);
    assert_eq!(three, three_);
}
