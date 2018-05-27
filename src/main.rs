extern crate actix_web;
extern crate ansi_term;
extern crate image;

mod img;

use actix_web::{server, App, HttpRequest, Responder};
use image::GenericImage;
use img::print_image;

const SIZE: u32 = 50;

fn display(req: HttpRequest) -> impl Responder {
    let image = image::open("imgs/happy_ferris.png").unwrap();
    let height = SIZE;
    let width = (image.width() * height) / image.height();
    print_image(image, true, width, height)
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(display)))
        .bind("0.0.0.0:8000")
        .expect("Can not bind to port 8000")
        .run();
}
