use super::entities::chromosome::Chromosome;
use super::entities::problem::Problem;
use super::operators::crossover;
use super::operators::crossover::Crossover;
use super::params;

use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub struct GA {
    pub problem: Problem,
    pub population: Vec<Chromosome>,
    pub mating_pool: Vec<Chromosome>,
    pub rng: ThreadRng,
}

impl GA {
    pub fn new(problem_file: &str) -> GA {
        let problem = Problem::init(problem_file);

        let mut population: Vec<Chromosome> = Vec::with_capacity(params::POPULATION_SIZE);
        let mut mating_pool = Vec::with_capacity(problem.n_jobs as usize);

        for _ in 0..params::POPULATION_SIZE {
            population.push(Chromosome::new(&problem));
        }

        let mut rng = thread_rng();

        return GA {
            problem,
            population,
            mating_pool,
            rng,
        };
    }

    pub fn new_test() -> GA {
        let problem = Problem::toy_problem();

        let mut population: Vec<Chromosome> = Vec::with_capacity(1);
        let mut mating_pool: Vec<Chromosome> = Vec::with_capacity(1);

        population.push(Chromosome::toy_chromosome(&problem));

        return GA {
            problem,
            population,
            mating_pool,
            rng: thread_rng(),
        };
    }

    pub fn run(&mut self) {
        // Initialisation - done in GA::new()

        for _ in 0..params::ITERATIONS {
            // calculate makespan
            self.population
                .iter_mut()
                .for_each(|c| c.makespan(&self.problem));

            for (i, c) in self.population.iter().enumerate() {
                println!("{i} {c}");
            }

            // Selection - fill up mating pool to be used for next generation
            self.mating_pool.clear();

            for _ in 0..(params::POPULATION_SIZE - params::ELITISM) {
                // Select both possible parants
                let p1 = self.population.choose(&mut self.rng).unwrap();
                let p2 = self.population.choose(&mut self.rng).unwrap();

                // Chose best in params::KEEP_BEST % of the time, random otherwise
                let winner = if self.rng.gen::<f32>() < params::KEEP_BEST {
                    std::cmp::min(p1, p2)
                } else {
                    vec![p1, p2].choose(&mut self.rng).unwrap()
                };
                // Create a new chromosome from the tournament winner
                let mut winner_clone = Chromosome::from(winner.jobs.to_vec());
                winner_clone.makespan = winner.makespan;
                self.mating_pool.push(winner_clone);
            }

            // Perform crossover
            for parents in self.mating_pool.chunks(2) {
                let p1 = &parents[0];
                let p2 = &parents[1];
                let children = crossover::SJOX::apply(p1, p2, None);
            }

            // Perform mutation

            // Elitism

            // Check termination criteria, and potentially proceed to next generation
        }
    }
}
