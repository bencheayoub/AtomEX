use std::io;

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