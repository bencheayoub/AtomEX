use std::io;

fn file_path(){
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Failed to read line");
    printls!("File path: {}", path);

}


fn main(){

}
