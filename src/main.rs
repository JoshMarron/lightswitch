#[macro_use]
extern crate rouille;

use rouille::Request;
use rouille::Response;

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
    rouille::start_server("localhost:6969", handle_request);
}
