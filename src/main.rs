#[macro_use]
extern crate rouille;

use rouille::Request;
use rouille::Response;

use std::net::ToSocketAddrs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const BL_POWER_PATH: &str = "output/bl_power";

struct Backlight {
    bl_path: String
}

#[derive(Debug)]
enum BlStatus {
    On,
    Off
}

impl Backlight {
    fn new(path: String) -> Backlight {
        Backlight {
            bl_path: path
        }
    }

    fn check_status(&self) -> std::io::Result<BlStatus> {
        let mut file = File::open(self.bl_path.clone())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        return if contents.contains("1") {
            Ok(BlStatus::Off)
        } else {
            Ok(BlStatus::On)
        }
    }

    fn turn_on(&self) -> std::io::Result<()> {
        let mut file = OpenOptions::new().write(true).open(self.bl_path.clone())?;
        file.write_all(b"0")?;
        Ok(())
    }

    fn turn_off(&self) -> std::io::Result<()> {
        let mut file = OpenOptions::new().write(true).open(self.bl_path.clone())?;
        file.write_all(b"1")?;
        Ok(())
    }
}

fn handle_request(request: &Request) -> Response {
    println!("Received request: {:?}", request);
    let backlight = Backlight::new(BL_POWER_PATH.to_string());
    router!(request,
        (GET) (/) => {
            Response::text(format!("Backlight is currently: {:?}", backlight.check_status()))
        },
        (GET) (/goodbye) => {
            Response::text("Goodbye world!")
        },
        (POST) (/on) => {
            println!("Turning backlight on...");
            match backlight.turn_on() {
                Ok(_) => Response::text("Backlight turned on successfully!"),
                Err(msg) => Response::text(format!("Failed to turn on backlight: {}", msg))
            }
        },
        (POST) (/off) => {
            println!("Turning backlight off...");
            match backlight.turn_off() {
                Ok(_) => Response::text("Backlight turned off successfully!"),
                Err(msg) => Response::text(format!("Failed to turn off backlight: {}", msg))
            }
        },
        _ => Response::empty_404()
    )
}

fn main() {
    println!("Hello world!");
    let address = "0.0.0.0:6969".to_socket_addrs().unwrap();
    println!("Will start server for {:?}", address);
    rouille::start_server("0.0.0.0:6969", handle_request);
}
