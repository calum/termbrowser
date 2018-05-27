extern crate actix_web;
extern crate ansi_term;
extern crate image;

mod img;

use actix_web::{server, App, HttpRequest, Responder};
use ansi_term::Color::Red;

fn greet(req: HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!\n", Red.paint(to))
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/{name}", |r| r.f(greet))
    }).bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run();
}
