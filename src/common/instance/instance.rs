use serde_derive::Deserialize;

/*
machines[stage]
processing_times[job][stage]
setup_times:[stage][previous_job][current_job]
Note: When previous job and current is the same, it is the inital setup time
*/

#[derive(Deserialize, Debug, Clone)]
pub struct Instance {
    pub jobs: u32,
    pub stages: u32,
    pub machines: Vec<u32>,
    pub processing_times: Vec<Vec<u32>>,
    pub setup_times: Vec<Vec<Vec<u32>>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn testing() {
        print!("yes")
    }
}
