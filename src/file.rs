use std::path::PathBuf;
use std::io;
use std::fs;


// Function to get the file path from the user.
pub fn file_path() -> String
{
    println!("Enter the file path: ");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Failed to read line.");
    let path = path.trim().to_string();
    println!("The file path is: {} .", path);
    path
}

pub fn lead_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(path)
}

pub fn save_file(path: &str, data: &[u8]) -> Result<(), std::io::Error> {
    fs::write(path, data)
}