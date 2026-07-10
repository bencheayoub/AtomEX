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

fn file_to_bytes(path: String) -> Vec<u8>
{
    let data = fs::read(&path).expect("Unable to read file");
    data
}

fn main(){
    let path = file_path();
    let data = file_to_bytes(path);
    println!("Find a File with size {}" , data.len());
    println!("What option do you want?");
    println!("1. Read the bytes of the file");
    println!("2. Write the bytes of the file");
    println!("3. Exit");
    let mut option = String::new();
    loop {
        option.clear();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        match option.trim().parse::<u32>(){
            Ok(1) => println!("The file data is: {:?}", data),
            Ok(2) => println!("i'll add this option soon."),
            Ok(3) => {
                println!("Exiting the program...");
                thread::sleep(std::time::Duration::from_secs(1));
                process::exit(0);
            }
            _ => println!("Invalid option"),
        }
    }

}

