use super::instance::Instance;
use serde_derive::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::ErrorKind;

#[derive(Serialize, Deserialize)]
pub struct Solution {
    jobs: u32,
    stages: u32,
    machines: Vec<u32>,
    makespan: u32,
    machine_completions: Vec<Vec<Vec<(u32, u32)>>>,
}

impl Solution {
    pub fn new(
        machine_completions: Vec<Vec<Vec<(u32, u32)>>>,
        makespan: u32,
        instance: &Instance,
    ) -> Solution {
        Solution {
            jobs: instance.jobs,
            stages: instance.stages,
            machines: instance.machines.to_vec(),
            makespan: makespan,
            machine_completions,
        }
    }

    pub fn write(&self, path: String) {
        let f = File::open(&path);

        match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(&path) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                _ => panic!("Could not open or create file: {}", &path),
            },
        };

        let s = serde_json::to_string(self).unwrap();
        fs::write(&path, s).expect("Could not write solution to file");
    }
}
