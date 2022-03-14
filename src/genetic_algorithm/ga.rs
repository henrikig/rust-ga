use std::env;

use crate::common::construction::{mddr::MDDR, Construction};
use crate::common::instance::Instance;
use crate::common::makespan::Makespan;
use crate::common::parser::parse;
use crate::common::solution::Solution;

use super::entities::chromosome::Chromosome;
use super::operators::crossover::{Crossover, BCBC, SB2OX, SJOX, XTYPE};
use super::operators::mutation::{self, Mutation};
use super::params;

use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub struct GA {
    pub instance: Instance,
    pub population: Vec<Chromosome>,
    pub mating_pool: Vec<Chromosome>,
    pub makespan: Makespan,
    pub rng: ThreadRng,
}

impl GA {
    pub fn new() -> GA {
        let instance = parse(params::PROBLEM_FILE).unwrap();
        let mut makespan = Makespan::new(&instance);

        let mut population = Vec::with_capacity(params::POPULATION_SIZE);
        let mating_pool = Vec::with_capacity(params::POPULATION_SIZE);

        // Add number of chromosomes from MDDR constructor as specified
        match params::CONSTRUCTION {
            Construction::_MDDR(num) => {
                let mut constructed: Vec<Chromosome> = MDDR {
                    makespan: &mut makespan,
                }
                .take(num)
                .collect();

                population.append(&mut constructed);
            }
            _ => (),
        }

        // Remaining chromosomes are added randomly
        while population.len() < params::POPULATION_SIZE {
            population.push(Chromosome::new(&instance));
        }

        let makespan = Makespan::new(&instance);

        let rng = thread_rng();

        return GA {
            instance,
            population,
            mating_pool,
            makespan,
            rng,
        };
    }

    pub fn run(&mut self) {
        for iteration in 0..params::ITERATIONS {
            // calculate makespan
            self.makespan();

            // Selection - fill up mating pool to be used for next generation
            self.mating_pool.clear();

            for _ in 0..(params::POPULATION_SIZE - params::ELITISM) {
                let winner = self.tournament();
                self.mating_pool.push(winner);
            }

            for p in self.mating_pool.chunks_exact_mut(2) {
                if self.rng.gen::<f32>() < params::XOVER_PROB {
                    // Crossover
                    let (c1, c2) = match params::XOVER {
                        XTYPE::_SJOX => SJOX::apply(&p[0], &p[1], None, &mut self.makespan),
                        XTYPE::_SB2OX => SB2OX::apply(&p[0], &p[1], None, &mut self.makespan),
                        XTYPE::_BCBC => BCBC::apply(&p[0], &p[1], None, &mut self.makespan),
                    };

                    for (i, parent) in p.iter_mut().enumerate() {
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

            if iteration % 1000 == 0 {
                self.generation_status(iteration);
            }

            self.population.clear();

            self.mating_pool
                .iter()
                .for_each(|c| self.population.push(Chromosome::from(c.jobs.to_vec())));
        }
    }

    pub fn run_steady_state(&mut self) {
        // Calculate makespan for all individuals in population
        self.makespan();
        self.population.sort();

        // Go through generations
        for iteration in 0..params::ITERATIONS {
            // Select two individuals from tournament selection
            let p1 = self.tournament();
            let p2 = self.tournament();

            // Crossover
            let (mut c1, mut c2) = match params::XOVER {
                XTYPE::_SJOX => SJOX::apply(&p1, &p2, None, &mut self.makespan),
                XTYPE::_SB2OX => SB2OX::apply(&p1, &p2, None, &mut self.makespan),
                XTYPE::_BCBC => BCBC::apply(&p1, &p2, None, &mut self.makespan),
            };

            // Mutate
            let mut mutate = |c| {
                if self.rng.gen::<f32>() < params::MUTATION_PROB {
                    mutation::SHIFT::apply(c)
                }
            };
            mutate(&mut c1);
            mutate(&mut c2);

            let mut makespan = |c: &mut Chromosome| c.makespan(&mut self.makespan);
            makespan(&mut c1);
            makespan(&mut c2);

            // Check if individuals are better than current worst & not already in population
            let mut replace = |c: Chromosome| {
                if &c < self.population.iter().last().unwrap() && !self.population.contains(&c) {
                    // Replace if so (inserting into correct position)
                    self.population.remove(self.population.len() - 1);
                    let idx = self.population.binary_search(&c).unwrap_or_else(|x| x);
                    self.population.insert(idx, c);
                }
            };
            replace(c1);
            replace(c2);

            if iteration % 1000 == 0 {
                self.generation_status(iteration);
            }
        }
    }

    pub fn makespan(&mut self) {
        self.population
            .iter_mut()
            .for_each(|c| c.makespan(&mut self.makespan));
    }

    fn tournament(&mut self) -> Chromosome {
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
        winner_clone
    }

    fn generation_status(&self, iteration: usize) {
        println!(
            "{}: {}-{}",
            iteration,
            self.population[0].makespan.unwrap(),
            self.population.iter().last().unwrap().makespan.unwrap()
        );
    }
}

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let mut ga = GA::new();

    // Flag `-s` indicates a steady state generational scheme
    if args.len() > 1 && args[1] == String::from("-s") {
        ga.run_steady_state();
    } else {
        ga.run();
    }

    // Find the best solution and write it to file
    let winner = ga.population.into_iter().min().unwrap();
    let (m, machine_completions) = ga.makespan.makespan(&winner.jobs);

    println!("Makespan count: {}", ga.makespan.count);

    let solution: Solution = Solution::new(machine_completions, m, &ga.instance);

    let problem = params::PROBLEM_FILE.split("/").last().unwrap();
    let path = String::from("solutions/ga/") + problem;
    solution.write(path);
}
