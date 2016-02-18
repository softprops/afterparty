//! Representations of Github events

extern crate serde;
extern crate serde_json;

use std::collections::BTreeMap;

// generated Event enum goes here

/// Enumeration of availble Github events
include!(concat!(env!("OUT_DIR"), "/events.rs"));

// provide a sensible default for our serde_json::Value type wrapper
impl Default for Value {
    fn default() -> Value {
        Value { json: serde_json::Value::Object(BTreeMap::new()) }
    }
}
