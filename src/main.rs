extern crate actix_web;
extern crate ansi_term;
extern crate image;

mod img;

use actix_web::{server, App, HttpRequest, Responder};
use ansi_term::Color::Red;
use img::print_image;

fn display(req: HttpRequest) -> impl Responder {
    let image = image::open("imgs/happy_ferris.pgn").unwrap();
    print_image(image, true, 100, 100)
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(display)))
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run();
}
