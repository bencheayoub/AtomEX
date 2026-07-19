use std::io;
use std::fs;
use std::process;
use std::thread;


// Function to get the file path from the user.
fn file_path() -> String
{
    println!("Enter the file path: ");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Failed to read line.");
    let path = path.trim().to_string();
    println!("The file path is: {} .", path);
    path
}

// Function to read the file and return its contents as a vector of bytes.
fn file_to_bytes(path: &str) -> Vec<u8>
{
    let data = fs::read(path).expect("Unable to read file");
    data
}

// Function to convert bytes to an ASCII string representation.
fn bytes_to_ascii(data: &[u8]) -> String
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
fn bytes_to_hex(data: &[u8]) -> String
{
    data.iter()
    .map(|b| format!("{:02x}", b))
    .collect::<Vec<String>>()
    .join(" ")
}

fn show_line(data: &[u8], offset: usize) -> String
{
    let offset = line * 16; 
    if offset > data.len() {
        println!("Offset is out of bounds.");
        return String::new();
    } else {
        let start = offset;
        let end = std::cmp::min(start + 16, data.len());
        let mut output = String::new();
        // the hex part of the line.
        let hex_parts: Vec<String> = data[start..end]
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();

        let hex_line = hex_parts.join(" ");
        let hex_padded = format!("{:<48}", hex_line);

        // the ascii part of the line.
        let ascii_part = data[start..end]
        .iter()
        .map(|&b| {
            if b >= 32 && b <= 126 {
                b as char
            } else {
                '.'
            }
        })
        .collect::<String>();

        output.push_str(&format!("{:08x} {} {}\n", start, hex_padded, ascii_part));
        output

    }
}

fn show_lines(data: &[u8], offset: usize, count: usize) -> String
{
    let mut output = String::new();
    for i in 0..count {
        let current_offset = offset + i * 16;
        if current_offset >= data.len() {
            break;
        }
        output.push_str(&show_line(data, current_offset));
    }
    output
}


// Function to create a hex dump of the file data.
fn hex_dump(data: &[u8]) -> String
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

// Main function.
fn main(){
    let path = file_path();
    let data = file_to_bytes(&path);
    println!("Find a File with size {} byte." , data.len());
    let mut option = String::new();
    loop {
        println!("What option do you want?");
        println!("1. Read the bytes of the file.");
        println!("2. Write the bytes of the file.");
        println!("3. Hex dump of the file.");
        println!("4. Search for a specific offset.");
        println!("5, show lines of the file.");
        println!("6. Show a specific line of the file.");
        println!("7. Exit the program.");
        option.clear();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        match option.trim().parse::<u32>(){
            Ok(1) => println!("The file data is: {:?}", data),
            Ok(2) => println!("i'll add this option soon."),
            Ok(3) => { 
                let dump = hex_dump(&data);
                println!("the hex dump of the file is: \n{}", dump); 
            },
            Ok(4) => { println!("i'll add this option soon.");
            }
            Ok(5) => {
                println!("what is the offset of the file you want to show?");
                let mut offset_input = String::new();
                io::stdin().read_line(&mut offset_input).expect("Failed to read line");
                let offset = offset_input.trim().parse::<usize>().unwrap_or(0);
                println!("how many lines do you want to show?");
                let mut count_input = String::new();
                io::stdin().read_line(&mut count_input).expect("Failed to read line");
                let count = count_input.trim().parse::<usize>().unwrap_or(10);
                let lines = show_lines(&data, offset, count);
                println!("the lines of the file are: \n{}", lines);
            }
            Ok(6) => {
                println!("what is the offset of the file you want to show?");
                let mut offset_input = String::new();
                io::stdin().read_line(&mut offset_input).expect("Failed to read line");
                let offset = offset_input.trim().parse::<usize>().unwrap_or(0);
                let line = show_line(&data, offset);
                println!("the line of the file is: \n{}", line);
            }
            Ok(7) => {
                println!("Exiting the program...");
                thread::sleep(std::time::Duration::from_secs(1));
                process::exit(0);
            }
            _ => println!("Invalid option. Please enter a number between 1 and 7."),
        }
    }

}

