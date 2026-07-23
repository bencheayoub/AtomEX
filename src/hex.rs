use std::fs;
use std::io;

// Function to read the file and return its contents as a vector of bytes.
pub fn file_to_bytes(path: &str) -> Result<Vec<u8>, io::Error>
{
    fs::read(path)
}

// Function to convert bytes to an ASCII string representation.
pub fn bytes_to_ascii(data: &[u8]) -> String
{
    data.iter()
    .map(|&b| {
        if b>= 32 && b <= 126 {
            b as char
        } else {
            '.'
        }
    }).collect()
}

// Function to convert bytes to a hex string representation.
pub fn bytes_to_hex(data: &[u8]) -> String
{
    data.iter()
    .map(|b| format!("{:02x}", b))
    .collect::<Vec<String>>()
    .join(" ")
}

// Function to create a hex dump of the file data.
pub fn hex_dump(data: &[u8]) -> String
{
    let hex= bytes_to_hex(data);
    let ascii = bytes_to_ascii(data);

    let hex_parts: Vec<&str> = hex.split_whitespace().collect();
    let ascii_parts: Vec<char> = ascii.chars().collect();

    let mut dump = String::new();
    let chunk_size = 16;
    for i in 0..(data.len() +chunk_size - 1) /chunk_size{
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, data.len());
        
        let hex_chunk = hex_parts[start .. end].join(" ");
        let hex_padded = format!("{:<48}", hex_chunk);

        let ascii_chunk:String  = ascii_parts[start .. end].iter().collect();
        let line = format!("{:08x} {} {}\n", start, hex_padded, ascii_chunk);
        dump.push_str(&line);
    }
    dump
}