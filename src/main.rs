mod data;
mod editor;
mod hex;
mod file;
mod parser;
mod view;


use std::io;
use std::fs;
use std::process;
use std::thread;
use data::ascii::art;
use file::file_path;
use hex::file_to_bytes;
use hex::hex_dump;
use view::show_lines;
use view::show_line;
use parser::parse_hex_bytes;
use editor::write_bytes_to_file;


// Main function.
fn main(){
    println!("{}", art);
    let path = file_path();
    let (path, mut data) = loop {
        let path = file_path();
        match file_to_bytes(&path){
            Ok(data) => break (path, data),
            Err(e) => {
                println!("Couldn't open the file: {}.", e);
                println!("Please try again.");
            }
        }
    };
    println!("Find a File with size {} byte." , data.len());
    let mut option = String::new();
    loop {
        println!("What option do you want?");
        println!("1. Read the bytes of the file.");
        println!("2. Write the bytes of the file.");
        println!("3. Hex dump of the file.");
        println!("4. Show lines of the file.");
        println!("5. Show a specific line of the file.");
        println!("6. Exit the program.");
        option.clear();
        io::stdin().read_line(&mut option).expect("Failed to read line.");
        match option.trim().parse::<u32>(){
            Ok(1) => println!("The file data is: {:?}", data),
            Ok(2) => {
                // read the line.
                let mut line_input = String::new();
                let line = loop {
                    println!("what is the line u wanna edit?");
                    line_input.clear();
                    io::stdin().read_line(&mut line_input).expect("Failed to read the line.");
                    match line_input.trim().parse::<usize>() {
                        Ok(v) if v!= 0 => break v,
                        Ok(_) => println!("Line number cannot be 0."),
                        Err(_) => { println!("That's noa a valind number."); continue;}
                    };
                };
                // read the offset.
                println!("What is the offset of the byte you wanna edit?");
                println!("e.g: \"enter 4 to modify the byter \'6d'\" 6672 6f6d 2043 7279 7074 6f2e 5574 696c");
                let mut offset = String::new();
                io::stdin().read_line(&mut offset).expect("Failed to read the offset.");
                let offset: usize = match offset.trim().parse() {
                    Ok(v) => v,
                    Err(_) => { println!("That's not a valid number."); continue; }
                };
                
                // read the new data.
                println!("Enter the new data as space-separeted hex.");
                println!("e.g: \"48 65 6c 6c 6f\" .");
                let mut data_input = String::new();
                io::stdin().read_line(&mut data_input).expect("Failed to read the new data.");
                let new_bytes = match parse_hex_bytes(data_input.trim()){
                    Some(b) => b,
                    None => { println!("Couldn't parse those bytes as hex. Use pairs like '48 65 6c'."); continue;}
                };

                // read the size of data.
                let size_of_data = new_bytes.len();

                // use the function.
                match write_bytes_to_file(&path, line, offset, size_of_data, &new_bytes){
                    Ok(()) =>{
                        match file_to_bytes(&path){
                            Ok(bytes) => data = bytes,
                            Err(e) => println!("Failed te reload the file: {}.", e),
                        }
                    }
                    Err(e) => println!("Failed to write to file: {}", e)
                };
                
            }
            Ok(3) => { 
                let dump = hex_dump(&data);
                println!("the hex dump of the file is: \n{}", dump); 
            },
            Ok(4) => {
                println!("what is the line of the file you want to show?");
                println!("eg: 08 for 0x80.");
                let mut offset_input = String::new();
                io::stdin().read_line(&mut offset_input).expect("Failed to read line");
                let input = offset_input.trim();
                let input = input.strip_prefix("0x").unwrap_or(input);
                let offset = usize::from_str_radix(input, 16).unwrap_or(0);
                println!("how many lines do you want to show?");
                let mut count_input = String::new();
                io::stdin().read_line(&mut count_input).expect("Failed to read line");
                let count = count_input.trim().parse::<usize>().unwrap_or(10);
                let lines = show_lines(&data, offset, count);
                println!("the lines of the file are: \n{}", lines);
            }
            Ok(5) => {
                println!("what is the offset of the file you want to show?");
                let mut offset_input = String::new();
                io::stdin().read_line(&mut offset_input).expect("Failed to read line");
                let input = offset_input.trim();
                let input = input.strip_prefix("0x").unwrap_or(input);
                let offset = usize::from_str_radix(input, 16).unwrap_or(0);
                let line = show_line(&data, offset);
                println!("the line of the file is: \n{}", line);
            }
            Ok(6) => {
                println!("Exiting the program...");
                thread::sleep(std::time::Duration::from_secs(1));
                process::exit(0);
            }
            _ => println!("Invalid option. Please enter a number between 1 and 7."),
        }
    }
}

