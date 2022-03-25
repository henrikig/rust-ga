use crate::common::instance::{Instance, Solution};
use crate::common::makespan::Makespan;
use crate::genetic_algorithm::entities::options::Args;

use super::entities::chromosome::Chromosome;
use super::entities::options::{Options, OptionsGrid};
use super::operators::crossover::{Crossover, BCBX, SB2OX, SJ2OX, XTYPE};
use super::operators::mutation::{self, Greedy, Mutation, Reverse, Swap, MTYPE, SHIFT};
use super::params;

use clap::StructOpt;
use csv::Writer;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::borrow::Cow;
use std::error::Error;
use std::{fs, path::PathBuf};

pub struct GA {
    pub instance: Instance,
    pub population: Vec<Chromosome>,
    pub mating_pool: Vec<Chromosome>,
    pub makespan: Makespan,
    pub options: Options,
    pub rng: ThreadRng,
}

impl Default for GA {
    fn default() -> Self {
        Options::default().build()
    }
}

impl GA {
    pub fn run(&mut self) {
        for iteration in 0..self.options.iterations {
            // calculate makespan
            self.makespan();

            // Selection - fill up mating pool to be used for next generation
            self.mating_pool.clear();

            for _ in 0..(self.options.pop_size - self.options.elitism) {
                let winner = self.tournament();
                self.mating_pool.push(winner);
            }

            for p in self.mating_pool.chunks_exact_mut(2) {
                if self.rng.gen::<f32>() < self.options.xover_prob {
                    // Crossover
                    let (c1, c2) = match self.options.xover_type {
                        XTYPE::_SJ2OX => SJ2OX::apply(&p[0], &p[1], None, &mut self.makespan),
                        XTYPE::_SB2OX => SB2OX::apply(&p[0], &p[1], None, &mut self.makespan),
                        XTYPE::_BCBX => BCBX::apply(&p[0], &p[1], None, &mut self.makespan),
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
                if self.rng.gen::<f32>() < self.options.mutation_prob {
                    match self.options.mutation_type {
                        MTYPE::_Shift => SHIFT::apply(c, &mut self.makespan),
                        MTYPE::_Reverse => Reverse::apply(c, &mut self.makespan),
                        MTYPE::_Swap => Swap::apply(c, &mut self.makespan),
                        MTYPE::_Greedy => Greedy::apply(c, &mut self.makespan),
                    }
                }
            });

            // Elitism
            self.population.sort();
            for c in self.population.iter().take(self.options.elitism) {
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
        for iteration in 0..self.options.iterations {
            // Select two individuals from tournament selection
            let p1 = self.tournament();
            let p2 = self.tournament();

            // Crossover
            let (mut c1, mut c2) = match self.options.xover_type {
                XTYPE::_SJ2OX => SJ2OX::apply(&p1, &p2, None, &mut self.makespan),
                XTYPE::_SB2OX => SB2OX::apply(&p1, &p2, None, &mut self.makespan),
                XTYPE::_BCBX => BCBX::apply(&p1, &p2, None, &mut self.makespan),
            };

            // Mutate
            let mut mutate = |c| {
                if self.rng.gen::<f32>() < self.options.mutation_prob {
                    match self.options.mutation_type {
                        mutation::MTYPE::_Shift => mutation::SHIFT::apply(c, &mut self.makespan),
                        mutation::MTYPE::_Reverse => {
                            mutation::Reverse::apply(c, &mut self.makespan)
                        }
                        mutation::MTYPE::_Swap => mutation::Swap::apply(c, &mut self.makespan),
                        mutation::MTYPE::_Greedy => mutation::Greedy::apply(c, &mut self.makespan),
                    }
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
        // Choose best in 'keep_best' % of the time, random otherwise
        let winner = if self.rng.gen::<f32>() < self.options.keep_best {
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

pub fn main() {
    // Parse arguments (run steady state, run all problems, test all parameters)
    let args = Args::parse();

    if args.run_all {
        run_all(&args);
    } else {
        run_one(&args);
    }
}

pub fn run_all(args: &Args) {
    // Get problem files, either the default, or all problem files
    let problem_files = get_problem_files(true);
    let problem_files_consumed = get_problem_files(true);
    let num_problems = problem_files.len();

    let mut results: Vec<Vec<String>> = Vec::with_capacity(problem_files.len());

    for (i, problem_file) in problem_files.into_iter().enumerate() {
        let options = Options {
            problem_file: Cow::Owned(problem_file),
            run_all: args.run_all,
            steady_state: args.steady_state,
            all_params: args.all_params,
            ..Options::default()
        };

        let all_options = OptionsGrid::default().get_options(options);
        let row_len = all_options.len() + 1;

        // If we test all parameter combinations, store result from each in vector
        let mut row = Vec::with_capacity(row_len);
        row.push(String::from(
            problem_files_consumed.get(i).unwrap().to_str().unwrap(),
        ));

        all_options.into_iter().for_each(|options| {
            row.push(run(options, false).to_string());
        });

        results.push(row);

        println!("Problem {} / {}", i + 1, num_problems);
    }

    write_results(&PathBuf::from("solutions/results.csv"), &results).unwrap();
    println!("All problems run, results are stored in `solutions/results.csv`");
}

pub fn run_one(args: &Args) {
    let options = Options {
        problem_file: Cow::Owned(PathBuf::from(params::PROBLEM_FILE)),
        run_all: args.run_all,
        steady_state: args.steady_state,
        all_params: args.all_params,
        ..Options::default()
    };

    let makespan = run(options, true);
    println!("GA completed with best makespan: {makespan}");
}

pub fn run(options: Options, run_one: bool) -> u32 {
    let mut ga = options.build();

    if ga.options.steady_state {
        ga.run_steady_state();
    } else {
        ga.run();
    }

    // Find the best solution
    let winner = ga.population.into_iter().min().unwrap();
    let (best_makespan, machine_completions) = ga.makespan.makespan(&winner.jobs);

    // We store the best solution if we only run one problem
    if run_one {
        let solution: Solution = Solution::new(machine_completions, best_makespan, &ga.instance);

        let problem = ga
            .options
            .problem_file
            .as_ref()
            .to_str()
            .unwrap()
            .split("/")
            .last()
            .unwrap();

        let path = String::from("solutions/ga/") + problem;
        solution.write(path);
    }

    best_makespan
}

fn get_problem_files(run_all: bool) -> Vec<PathBuf> {
    match run_all {
        true => fs::read_dir("./instances/ruiz/json")
            .unwrap()
            .into_iter()
            .map(|p| p.unwrap().path())
            .collect(),
        false => vec![PathBuf::from(params::PROBLEM_FILE)],
    }
}

fn write_results(filename: &PathBuf, records: &Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(filename)?;

    for record in records {
        wtr.write_record(record)?;
    }

    wtr.flush()?;
    Ok(())
}
