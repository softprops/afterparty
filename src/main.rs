extern crate clap;
extern crate afterparty;
extern crate rustc_serialize;

use clap::App;
use std::fs::File;
use std::io::Read;
use rustc_serialize::json;

pub fn main() {
  let matches = App::new("afterparty")
    .about("github webhook server")
    .args_from_usage(
      "<INPUT> 'json configuration'"
     ).get_matches();
  let config = matches.value_of("INPUT").unwrap();
  let contents: Option<String> =
    File::open(config).and_then(|mut f| {
      let mut s = String::new();
      f.read_to_string(&mut s).map(|_| s)
    }).ok();
  match json::decode::<afterparty::Service>(
    &contents.expect("unresolved file contents")) {
    Ok(svc) => svc.run(),
    Err(e)  => println!("invalid service definition: {}", e)
  };
}
