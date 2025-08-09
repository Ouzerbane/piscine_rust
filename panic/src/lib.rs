use std::fs::File;
use std::fs;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap()
    // todo!()
}
