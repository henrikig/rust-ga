use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Instance {
    pub products: u32,
    pub stages: u32,
    pub machines: Vec<u32>,
    pub production_times: Vec<Vec<u32>>,
    pub setup_times: Vec<Vec<Vec<u32>>>,
}
