#[macro_use]
extern crate rouille;

use rouille::Request;
use rouille::Response;

use std::net::ToSocketAddrs;

fn handle_request(request: &Request) -> Response {
    println!("Received request: {:?}", request);
    router!(request,
        (GET) (/) => {
            Response::text("Hello world!")
        },
        (GET) (/goodbye) => {
            Response::text("Goodbye world!")
        },
        (POST) (/on) => {
            println!("Turning backlight on...");
            Response::text("Turning backlight on")
        },
        (POST) (/off) => {
            println!("Turning backlight off...");
            Response::text("Turning backlight off")
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
