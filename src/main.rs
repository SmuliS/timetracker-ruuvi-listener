use std::{error::Error, process::exit};

fn main() {
    println!("Hello, world!");
    if let Err(e) = run() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    Ok(())
}
