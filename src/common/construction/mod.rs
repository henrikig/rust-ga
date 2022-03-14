use crate::genetic_algorithm::entities::chromosome::Chromosome;

pub mod mddr;
pub mod neh;

pub enum Construction {
    _Random,
    _MDDR(usize),
}

pub trait Constructor {
    fn create(&mut self) -> Chromosome;
}
