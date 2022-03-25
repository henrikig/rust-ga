use serde_derive::Serialize;

use crate::genetic_algorithm::entities::chromosome::Chromosome;

pub mod mddr;
pub mod neh;

#[derive(Debug, Clone, Serialize)]
pub enum Construction {
    Random,
    MDDR(f32),
}

pub trait Constructor {
    fn create(&mut self) -> Chromosome;
}
