use rand::prelude::{SliceRandom, StdRng};
use serde_derive::Serialize;

use crate::{
    common::{construction::gch, makespan::Makespan},
    genetic_algorithm::entities::chromosome::Chromosome,
};

use super::mutation::{self, Mutation};

#[allow(dead_code)]
#[derive(Clone, Serialize)]
pub enum RTYPE {
    Random,
    GCH,
    Mutate,
    NoReplacement,
}

pub struct Random;
pub struct GCH;
pub struct Mutate;
pub struct NoReplacement;

pub trait Replacement {
    fn replace(population: &mut Vec<Chromosome>, keep: f64, m: &mut Makespan, rng: &mut StdRng);
}

impl Replacement for Random {
    fn replace(population: &mut Vec<Chromosome>, keep: f64, m: &mut Makespan, rng: &mut StdRng) {
        let always_keep = (population.len() as f64 * keep) as usize;

        for index in always_keep..population.len() {
            let new_c = Chromosome::new(&m.instance, rng);
            population[index] = new_c;
        }
    }
}

impl Replacement for GCH {
    fn replace(population: &mut Vec<Chromosome>, keep: f64, m: &mut Makespan, rng: &mut StdRng) {
        let popsize = population.len();
        let always_keep = (popsize as f64 * keep) as usize;

        let mut constructed: Vec<Chromosome> = gch::GCH { makespan: m, rng }
            .take(popsize - always_keep)
            .collect();

        for index in always_keep..popsize {
            let new_c = constructed.remove(0);
            population[index] = new_c;
        }
    }
}

impl Replacement for Mutate {
    fn replace(population: &mut Vec<Chromosome>, keep: f64, m: &mut Makespan, rng: &mut StdRng) {
        let popsize = population.len();
        let always_keep = (popsize as f64 * keep) as usize;

        for index in always_keep..always_keep + (popsize - always_keep) / 2 {
            let new_c = population[0..always_keep]
                .choose(rng)
                .unwrap()
                .jobs
                .to_vec();

            let mut new_c = Chromosome::from(new_c);
            mutation::Random::apply(&mut new_c, m, rng);

            population[index] = new_c;
        }

        for index in always_keep + (popsize - always_keep) / 2..popsize {
            let new_c = Chromosome::new(&m.instance, rng);
            population[index] = new_c;
        }
    }
}

impl Replacement for NoReplacement {
    fn replace(
        _population: &mut Vec<Chromosome>,
        _keep: f64,
        _m: &mut Makespan,
        _rng: &mut StdRng,
    ) {
        return ();
    }
}
