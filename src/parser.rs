/*
https://docs.serde.rs/serde_json/de/fn.from_reader.html
*/

use serde_derive::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Instance {
    pub products: u64,
    pub stages: u64,
    pub machines: Vec<u64>,
    pub production_times: Vec<Vec<u64>>,
    pub setup_times: Vec<Vec<Vec<u64>>>,
}

/* Example of use:
let instance = parser::parse("n20m2-1.json").unwrap();
println!("{:#?}", instance);
println!("{}", instance.production_times[0][0]);

path: "instances\\ruiz\\json\\FILENAME.json"

machines[stage]
production_times[product][stage]
setup_times:[machine][previous_job][current_job]
*/

pub fn parse<P: AsRef<Path>>(path: P) -> Result<Instance, Box<dyn Error>> {
    // Open file in read only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read content of JSON contents of the file as an instance of Instance.
    let i = serde_json::from_reader(reader)?;

    // return instance
    Ok(i)
}
