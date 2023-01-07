use std::io::{self, Write};

use crate::{cmd::args::InitCommand, core::manifest::Manifest};

pub fn handle_init(args: InitCommand) {
    println!("Initializing project...");

    // Get project name from user input
    let mut name = String::new();

    print!("Type now the name by which it shall be known: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    // Remove newline from project name
    name = name.trim_end().to_string();

    // Create a new manifest with the name
    let manifest = Manifest::new(name);

    // Save manifest in the current directory
    let current_dir = std::env::current_dir().unwrap();
    let path = current_dir.as_path().join("package.json");
    
    manifest.save(path).expect("Couldn't save `package.json`");
}   
