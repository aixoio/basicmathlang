use std::{io, fs};


pub fn read_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}
