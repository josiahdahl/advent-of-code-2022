use std::fs;
use std::path;
pub fn get_input(date: &str) -> String {
    // reading from the repo root (where Cargo is)
    let path = format!("./src/inputs/{date}.txt");
    let file_path = path::Path::new(&path);
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}
