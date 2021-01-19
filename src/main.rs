use std::io::prelude::Write;

use std::thread::sleep;
use std::time::Duration;
use std::{error::Error, process::exit};

use clap::{ArgMatches, Values};
use serde::Serialize;
use unix_socket::UnixStream;

use app::{create_app, OUTPUT};
use bluetooth::{find_ruuvi_devices, RuuviDevice};
use errors::SocketConnectionError;

mod app;
mod bluetooth;
mod errors;

fn main() {
    let app = create_app();
    let args = app.get_matches();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run(args: ArgMatches) -> Result<(), Box<dyn Error>> {
    let sockets = connect_to_sockets(args.values_of(OUTPUT))?;
    loop {
        for device in find_ruuvi_devices()? {
            for mut socket in &sockets {
                let mut data = serde_json::to_string(&Address::from(&device))?;
                data.push_str("\n");
                socket.write_all(data.as_bytes())?;
            }
            println!("{:?}", device.get_acceleration());
        }
        sleep(Duration::from_secs(1));
    }
}

fn connect_to_sockets(socket_paths: Option<Values>) -> Result<Vec<UnixStream>, Box<dyn Error>> {
    socket_paths
        .map(|values| {
            let mut streams: Vec<UnixStream> = vec![];
            for value in values {
                let stream = UnixStream::connect(value)
                    .map_err(|e| SocketConnectionError::from(value, &e))?;
                streams.push(stream);
            }
            Ok(streams)
        })
        .unwrap_or(Ok(vec![]))
}

#[derive(Serialize)]
struct Address {
    device_id: String,
    x: i16,
    y: i16,
    z: i16,
}

impl Address {
    fn from(device: &RuuviDevice) -> Self {
        let acc = device.get_acceleration();
        Address {
            device_id: device.get_id(),
            x: acc.0,
            y: acc.1,
            z: acc.2,
        }
    }
}
