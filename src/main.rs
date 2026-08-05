// mods.
mod data;
mod editor;
mod hex;
mod file;
mod parser;
mod view;
mod gui;


// Main function.
fn main(){
    if let Err(e) = gui::run_gui() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

