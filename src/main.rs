#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;
extern crate afterparty;
extern crate hyper;

use afterparty::{Delivery, Hub};
use clap::App;
use hyper::Server;

pub fn main() {
    env_logger::init().unwrap();
    let _ = App::new("afterparty")
        .version(env!("CARGO_PKG_VERSION"))
        .about("github webhook server")
        .get_matches();
    let addr = format!("0.0.0.0:{}", 4567);
    let mut hub = Hub::new();
    hub.handle("*", |delivery: &Delivery| {
        println!("rec delivery {:?}", delivery)
    });
    let srvc = Server::http(&addr[..]).unwrap()
        .handle(hub);
    println!("listening on {}", addr);
    srvc.unwrap();
}
