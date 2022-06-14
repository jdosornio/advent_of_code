use std::fs;
use std::io;

pub mod y2015;

pub fn get_text(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}
