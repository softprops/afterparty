#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;
extern crate afterparty;
extern crate hyper;

use afterparty::Hub;
use clap::App;
use hyper::Server;

pub fn main() {
    env_logger::init().unwrap();
    let _ = App::new("afterparty")
        .about("github webhook server")
        .get_matches();
    let addr = format!("0.0.0.0:{}", 4567);
    let hub = Hub::new();
    let srvc = Server::http(&addr[..]).unwrap()
        .handle(hub);
    println!("listening on {}", addr);
    srvc.unwrap();
}
