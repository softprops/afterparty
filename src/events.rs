//! Representations of Github events

extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

// generated Event enum goes here

/// Enumeration of availble Github events
include!(concat!(env!("OUT_DIR"), "/events.rs"));
