#[macro_use]
extern crate rouille;

use rouille::Request;
use rouille::Response;

use std::net::ToSocketAddrs;
use std::process::Command;

const BL_POWER_PATH: &str = "/sys/class/backlight/rpi_backlight/bl_power";

const DUMMY_BL_POWER_PATH: &str = "dummy_bl_power";

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

    fn check_status(&self) -> BlStatus {
        let comm = Command::new("cat")
                          .args(&[format!("{}", self.bl_path)])
                          .output()
                          .expect("Failed to execute process!");

        let output = String::from_utf8(comm.stdout).expect("Failed to get output");
        return if output == "1".to_string() {
            BlStatus::Off
        } else {
            BlStatus::On
        }
    }

    fn turn_on(&self) -> Result<(), String> {
        let comm = Command::new("echo")
                          .args(&["-n", "0", &format!("> {}", self.bl_path)])
                          .output();
        match comm {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to turn on backlight!".to_string())
        }
    }

    fn turn_off(&self) -> Result<(), String> {
        let comm = Command::new("echo")
                          .args(&["-n", "1", &format!("> {}", self.bl_path)])
                          .output();
        match comm {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to turn off backlight!".to_string())
        }
    }
}

fn handle_request(request: &Request) -> Response {
    println!("Received request: {:?}", request);
    let backlight = Backlight::new(DUMMY_BL_POWER_PATH.to_string());
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
