use std::io;
use std::fs;
use std::process;
use std::thread;

fn file_path() -> String
{
    println!("Enter the file path: ");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Failed to read line");
    let path = path.trim().to_string();
    println!("The file path is: {}", path);
    path
}

fn file_to_bytes(path: &str) -> Vec<u8>
{
    let data = fs::read(path).expect("Unable to read file");
    data
}

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

fn bytes_to_hex(data: &[u8]) -> String
{
    data.iter()
    .map(|b| format!("{:02x}", b))
    .collect::<Vec<String>>()
    .join(" ")
    
}

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


fn main(){
    let path = file_path();
    let data = file_to_bytes(&path);
    println!("Find a File with size {} byte" , data.len());
    let mut option = String::new();
    loop {
        println!("What option do you want?");
        println!("1. Read the bytes of the file");
        println!("2. Write the bytes of the file");
        println!("3. Hex dump of the file");
        println!("4. Exit");
        option.clear();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        match option.trim().parse::<u32>(){
            Ok(1) => println!("The file data is: {:?}", data),
            Ok(2) => println!("i'll add this option soon."),
            Ok(3) => { 
                let dump = hex_dump(&data);
                println!("the hex dump of the file is: \n{}", dump); 
            },
            Ok(4) => {
                println!("Exiting the program...");
                thread::sleep(std::time::Duration::from_secs(1));
                process::exit(0);
            }
            _ => println!("Invalid option"),
        }
    }
    

}

