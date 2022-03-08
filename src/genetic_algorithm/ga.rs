use crate::common::instance::Instance;
use crate::common::parser::parse;

use super::entities::chromosome::Chromosome;
use super::operators::crossover;
use super::operators::crossover::Crossover;
use super::operators::mutation;
use super::operators::mutation::Mutation;
use super::params;

use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub struct GA {
    pub instance: Instance,
    pub population: Vec<Chromosome>,
    pub mating_pool: Vec<Chromosome>,
    pub rng: ThreadRng,
}

impl GA {
    pub fn new() -> GA {
        let instance = parse(params::PROBLEM_FILE).unwrap();

        let mut population = Vec::with_capacity(params::POPULATION_SIZE);
        let mating_pool = Vec::with_capacity(params::POPULATION_SIZE);

        for _ in 0..params::POPULATION_SIZE {
            population.push(Chromosome::new(&instance));
        }

        let rng = thread_rng();

        return GA {
            instance,
            population,
            mating_pool,
            rng,
        };
    }

    pub fn run(&mut self) {
        for iteration in 0..params::ITERATIONS {
            // calculate makespan
            self.population
                .iter_mut()
                .for_each(|c| c.makespan(&self.instance));

            // Selection - fill up mating pool to be used for next generation
            self.mating_pool.clear();

            for _ in 0..(params::POPULATION_SIZE - params::ELITISM) {
                // Select both possible parants
                let p1 = self.population.choose(&mut self.rng).unwrap();
                let p2 = self.population.choose(&mut self.rng).unwrap();

                // Choose best in params::KEEP_BEST % of the time, random otherwise
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

            for parents in self.mating_pool.chunks_exact_mut(2) {
                if self.rng.gen::<f32>() < params::XOVER_PROB {
                    let (c1, c2) = crossover::SJOX::apply(&parents[0], &parents[1], None);

                    for (i, parent) in parents.iter_mut().enumerate() {
                        if i == 0 {
                            *parent = Chromosome::from(c1.jobs.to_vec());
                        } else {
                            *parent = Chromosome::from(c2.jobs.to_vec());
                        }
                    }
                }
            }

            // Perform mutation
            self.mating_pool.iter_mut().for_each(|c| {
                if self.rng.gen::<f32>() < params::MUTATION_PROB {
                    mutation::SHIFT::apply(c);
                }
            });

            // Elitism
            // TODO: omit sorting, find best chromosomes in another way
            self.population.sort();
            for c in self.population.iter().take(params::ELITISM) {
                self.mating_pool.push(Chromosome::from(c.jobs.to_vec()));
            }

            println!(
                "{}: {}-{}",
                iteration,
                self.population[0].makespan.unwrap(),
                self.population.iter().last().unwrap().makespan.unwrap()
            );

            self.population.clear();

            self.mating_pool
                .iter()
                .for_each(|c| self.population.push(Chromosome::from(c.jobs.to_vec())));
        }
    }
}

pub fn run() {
    let mut ga = GA::new();
    ga.run();
}
