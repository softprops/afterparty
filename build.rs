extern crate case;
extern crate hyper;
extern crate rustc_serialize;

use std::collections::HashMap;
use case::CaseExt;
use rustc_serialize::json::Json;
use hyper::Client;
use std::env;
use std::fs::File;
use std::io::{Result, Read, Write};
use std::path::Path;


/// generate an enum of Events
fn main() {

    let events = vec!["commit_comment",
                      "create",
                      "delete",
                      "deployment",
                      "deployment_status",
                      "fork",
                      "gollum",
                      "issue_comment",
                      "issues",
                      "member",
                      "membership",
                      "page_build",
                      "ping",
                      "public",
                      "pull_request",
                      "pull_request_review_comment",
                      "push",
                      "release",
                      "repository",
                      "status",
                      "team_add",
                      "watch"];

    if let Ok(_) = env::var("FETCH_PAYLOAD_DATA") {
        let _ = fetch_payload_data(&events);
    }
    let _ = generate(&events);
}

fn fetch_payload_data(events: &Vec<&str>) -> Result<()> {
    let data_dir = Path::new("data");
    let client = Client::new();
    for event in events {
        let src = format!("https://raw.githubusercontent.com/github/developer.github.\
                           com/master/lib/webhooks/{}.payload.json",
                          event);
        let mut res = client.get(&src)
                            .send()
                            .unwrap();
        let mut buf = String::new();
        try!(res.read_to_string(&mut buf));
        let outfile = data_dir.join(format!("{}.json", event));
        let mut f = try!(File::create(outfile));
        try!(f.write_all(buf.as_bytes()));
    }
    Ok(())
}

fn generate(events: &Vec<&str>) -> Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("events.rs");
    let mut f = try!(File::create(&dest_path));
    let client = Client::new();

    // generate Event enum containing definitions each hook struct definition
    try!(f.write_all(b"#[derive(Debug, RustcDecodable)]
pub enum Event {
"));
    let mut defs = HashMap::new();
    for event in events {
        let src = format!("https://raw.githubusercontent.com/github/developer.github.\
                           com/master/lib/webhooks/{}.payload.json",
                          event);
        let mut res = client.get(&src)
                            .send()
                            .unwrap();
        let mut buf = String::new();
        try!(res.read_to_string(&mut buf));
        let parsed = Json::from_str(&buf).unwrap();
        let enum_name = container_name(event);
        try!(f.write_all(format!("  /// generated from {}
  {} ",
                                 src,
                                 enum_name)
                             .as_bytes()));
        try!(f.write_all(b"{
 "));

        match parsed {
            Json::Object(obj) => {
                for (k, v) in obj {
                    try!(f.write_all(format!("   {}: {},
 ",
                                             field_name(&k),
                                             value(&enum_name, &mut defs, &k, &v))
                                         .as_bytes()))
                }
            }
            _ => (),
        }
        try!(f.write_all(b"
  },"));
    }
    try!(f.write_all(b"}

"));

    try!(print_structs(&mut f, defs, &mut vec![], 0));
    Ok(())
}

fn print_structs(f: &mut File,
                 defs: HashMap<String, Json>,
                 generated: &mut Vec<String>,
                 depth: usize)
                 -> Result<()> {
    let mut aux = HashMap::new();
    for (struct_name, json) in defs.iter() {
        if generated.contains(&struct_name) {
            continue;
        }
        println!("struct {}", struct_name);
        try!(f.write_all(format!("
#[derive(Default, Debug, RustcDecodable)]
pub struct {} ",
                                 struct_name)
                             .as_bytes()));
        try!(f.write_all(b"{"));
        match json {
            &Json::Object(ref obj) => {
                for (k, v) in obj {
                    try!(f.write_all(format!("
  pub {}: {},",
                                             field_name(&k),
                                             value(&struct_name, &mut aux, &k, &v))
                                         .as_bytes()))
                }
            }
            _ => (),
        }

        try!(f.write_all(b"
}
"));
        generated.push(struct_name.clone());
    }
    if !aux.is_empty() {
        try!(print_structs(f, aux, generated, depth + 1));
    }
    Ok(())
}

fn value(container: &String, defs: &mut HashMap<String, Json>, k: &str, j: &Json) -> String {
    match j {
        &Json::I64(_) => "i64".to_owned(),
        &Json::U64(_) => "u64".to_owned(),
        &Json::F64(_) => "f64".to_owned(),
        &Json::String(_) => "String".to_owned(),
        &Json::Boolean(_) => "bool".to_owned(),
        &Json::Array(ref jv) => {
            if jv.is_empty() {
                "Vec<String>".to_owned() // this is just a guess!
            } else {
                format!("Vec<{}>", value(&container, defs, k, &jv[0]))
            }
        }
        obj @ &Json::Object(_) => {
            if "payload" == k {
                "HashMap<String, String>".to_owned()
            } else {
                let struct_name = match container_name(k) {
                    ref recursive if recursive == container => format!("{}Inner", recursive),
                    valid => valid,
                };
                defs.insert(struct_name.clone(), obj.clone());
                struct_name
            }
        }
        &Json::Null => "Option<String>".to_owned(),
    }
}

fn container_name(field: &str) -> String {
    if "self" == field {
        "SelfLink".to_owned()
    } else {
        field.to_camel()
    }
}

/// works around conflicts with reservied words
fn field_name(s: &str) -> String {
    let reserved = vec!["ref", "self", "type"];
    if reserved.contains(&s) {
        format!("_{}", s)
    } else {
        s.to_owned()
    }
}
