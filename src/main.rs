#![allow(unused_variables)]

mod package;

use std::env;

use package::Package;

fn main() {
    let cmd = env::args().nth(1).expect("No command specified");
    let target = env::args().nth(2).expect("No target specified");

    let current_dir = env::current_dir().expect("");
    let package_path = current_dir.join("package.json");

    if !package_path.exists() {
        
    }
}
