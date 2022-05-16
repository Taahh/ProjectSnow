use std::fs;
use std::path::Path;

pub fn file_data<P: AsRef<Path>>(path: P) -> String {
    return fs::read_to_string(path).unwrap();
}
