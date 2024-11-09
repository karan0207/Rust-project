use std::fs;

pub fn save_file(filename: &str, file_data: &[u8]) {
    fs::write(filename, file_data).expect("Unable to save file");
}

pub fn load_file(filename: &str) -> Vec<u8> {
    fs::read(filename).expect("Unable to read file")
}
