extern crate afterparty;

//use std::env;

pub fn main() {
  //afterparty::Service {
  //  secret: env::var("GH_SECRET").ok(),
  //  port: 4567,
  //  hooks: vec![]
  //}.run()
   let debug = afterparty::hook::Hook::echo();
   debug.run(&afterparty::Event {
     name: "push".to_owned(),
     payload: "{}".to_owned()
   });
   ()
}