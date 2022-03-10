use crate::genetic_algorithm::entities::chromosome::Chromosome;

pub mod mddr;

pub enum Construction {
    _Random,
    _MDDR(usize),
}

pub trait Constructor {
    fn create(&self) -> Chromosome;
}
