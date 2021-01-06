use std::{error::Error, process::exit};

use app::create_app;

mod app;

fn main() {
    let app = create_app();
    let _args = app.get_matches();
    if let Err(e) = run() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    Ok(())
}
