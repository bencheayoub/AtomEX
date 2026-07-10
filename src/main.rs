use std::io;

fn file_path() -> String
{
    println!("Enter the file path: ");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Failed to read line");
    println!("The file path is: {}", path);
    return path;
}


fn main(){
file_path();
}
