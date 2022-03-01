use super::entities::chromosome::Chromosome;
use super::entities::problem::Problem;
use super::params::*;

pub struct GA {
    pub problem: Problem,
    pub population: Vec<Chromosome>,
}

impl GA {
    pub fn new(problem: &str) -> GA {
        let problem = Problem::init(problem);

        let mut population: Vec<Chromosome> = Vec::with_capacity(POPULATION_SIZE);

        for _ in 0..POPULATION_SIZE {
            population.push(Chromosome::new(&problem));
        }

        return GA {
            problem,
            population,
        };
    }

    pub fn run(&self) {
        for (i, c) in self.population.iter().enumerate() {
            println!("{}: {}", i, c);
        }
    }
}
