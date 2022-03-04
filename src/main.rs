extern crate serde;
extern crate serde_json;
use serde_json::Value as JsonValue;
#[macro_use]
extern crate serde_derive;

struct Instance {
    products: u32
    stages: u32
    machines: Vec<u64>
    production_times: Vec<Vec<u64>>
    setup_times: Vec<Vec<u64>>
}

fn main() {
    println!("Hello, world!");
}
