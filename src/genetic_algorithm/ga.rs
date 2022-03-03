use super::entities::chromosome::Chromosome;
use super::entities::problem::Problem;
use super::params::*;

pub struct GA {
    pub problem: Problem,
    pub population: Vec<Chromosome>,
}

impl GA {
    pub fn new(problem_file: &str) -> GA {
        let problem = Problem::init(problem_file);

        let mut population: Vec<Chromosome> = Vec::with_capacity(POPULATION_SIZE);

        for _ in 0..POPULATION_SIZE {
            population.push(Chromosome::new(&problem));
        }

        return GA {
            problem,
            population,
        };
    }

    pub fn new_test() -> GA {
        let problem = Problem::toy_problem();

        let mut population: Vec<Chromosome> = Vec::with_capacity(1);

        population.push(Chromosome::toy_chromosome(&problem));

        return GA {
            problem,
            population,
        };
    }

    pub fn run(&mut self) {
        for (i, c) in self.population.iter_mut().enumerate() {
            c.makespan(&self.problem);
            println!("{i} {c}");
        }
    }
}
