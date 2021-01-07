use std::thread::sleep;
use std::time::Duration;
use std::{error::Error, process::exit};

use app::create_app;
use bluetooth::find_ruuvi_devices;

mod app;
mod bluetooth;

fn main() {
    let app = create_app();
    let _args = app.get_matches();
    if let Err(e) = run() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    loop {
        for device in find_ruuvi_devices()? {
            println!("{:?}", device.get_acceleration());
        }
        sleep(Duration::from_secs(1));
    }
}
