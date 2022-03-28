use crate::common::instance::{Instance, Solution};
use crate::common::makespan::Makespan;
use crate::genetic_algorithm::entities::options::Args;

use super::entities::chromosome::Chromosome;
use super::entities::options::{Options, OptionsGrid, Params};
use super::operators::crossover::{Crossover, BCBX, SB2OX, SJ2OX, XTYPE};
use super::operators::crowding;
use super::operators::local_search::ls_ig;
use super::operators::mutation::{self, Greedy, Mutation, Reverse, Swap, MTYPE, SHIFT};
use super::params;

use clap::StructOpt;
use csv::Writer;
use lexical_sort::natural_lexical_cmp;
use rand::{prelude::ThreadRng, seq::SliceRandom, Rng};
use std::borrow::Cow;
use std::{error::Error, fs, path::PathBuf};

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
        let mut non_improvement_counter: usize = 0;
        for iteration in 0..self.options.iterations {
            // Replace the chromosomes with the worst fit if there has been no improvement in the best fit for y iterations
            if non_improvement_counter >= self.options.non_improving_iterations {
                for index in self.options.allways_keep..self.options.pop_size {
                    self.population[index] = Chromosome::new(&self.instance);
                }
            }

            // Selection - fill up mating pool to be used for next generation
            self.mating_pool.clear();

            for _ in 0..(self.options.pop_size - self.options.elitism) {
                let winner = self.tournament();
                self.mating_pool.push(winner);
            }

            for p in self.mating_pool.chunks_exact_mut(2) {
                if self.rng.gen::<f32>() < self.options.xover_prob {
                    // Crossover
                    let (mut c1, mut c2) = match self.options.xover_type {
                        XTYPE::SJ2OX => SJ2OX::apply(&p[0], &p[1], None, &mut self.makespan),
                        XTYPE::SB2OX => SB2OX::apply(&p[0], &p[1], None, &mut self.makespan),
                        XTYPE::BCBX => BCBX::apply(&p[0], &p[1], None, &mut self.makespan),
                    };

                    if params::PERFORM_CROWDING {
                        c1.makespan(&mut self.makespan);
                        c2.makespan(&mut self.makespan);

                        let [winner1, winner2] =
                            crowding::survivor_selection(&[c1, c2], p, params::CROWDING_SCALE);

                        p[0] = winner1;
                        p[1] = winner2;
                    } else {
                        for (i, parent) in p.iter_mut().enumerate() {
                            if i == 0 {
                                *parent = Chromosome::from(c1.jobs.to_vec());
                            } else {
                                *parent = Chromosome::from(c2.jobs.to_vec());
                            }
                        }
                    }
                }
            }

            // Perform mutation
            self.mating_pool.iter_mut().for_each(|c| {
                if self.rng.gen::<f32>() < self.options.mutation_prob {
                    match self.options.mutation_type {
                        MTYPE::Shift => SHIFT::apply(c, &mut self.makespan),
                        MTYPE::Reverse => Reverse::apply(c, &mut self.makespan),
                        MTYPE::Swap => Swap::apply(c, &mut self.makespan),
                        MTYPE::Greedy => Greedy::apply(c, &mut self.makespan),
                    }
                }
            });

            // Local search
            if self.options.local_search {
                self.mating_pool
                    .iter_mut()
                    .for_each(|c| ls_ig(c, &mut self.makespan, self.options.approx_calc as u32));
            }

            // Check if any of the new chromosomes are improvements to the current best
            // Sort population for elitism
            self.population.sort();
            // Calculate makespan for new individuals in mating pool
            self.makespan();

            // Check if any of the new chromosomes are improvements to the current best
            if self.population.first().unwrap() < self.mating_pool.iter().min().unwrap() {
                non_improvement_counter += 1;
            } else {
                non_improvement_counter = 0;
            }

            // Elitism
            for c in self.population.iter().take(self.options.elitism) {
                let mut elite = Chromosome::from(c.jobs.to_vec());
                elite.makespan = c.makespan;
                elite.updated = false;
                self.mating_pool.push(elite);
            }

            if iteration % 1000 == 0 {
                self.generation_status(iteration);
            }

            self.population.clear();

            self.mating_pool.iter().for_each(|c| {
                let mut new_c = Chromosome::from(c.jobs.to_vec());
                new_c.makespan = c.makespan;
                new_c.updated = false;
                self.population.push(new_c);
            })
        }
    }

    pub fn run_steady_state(&mut self) {
        // Calculate makespan for all individuals in population
        self.population.sort();
        let mut non_improvement_counter: usize = 0;

        // Go through generations
        for iteration in 0..self.options.iterations {
            // Replace the chromosomes with the worst fit if there has been no improvement in the best fit for y iterations
            if non_improvement_counter >= self.options.non_improving_iterations {
                for index in self.options.allways_keep..self.options.pop_size {
                    let mut new_c = Chromosome::new(&self.instance);
                    new_c.makespan(&mut self.makespan);
                    self.population[index] = new_c;
                }
            }

            // Select two individuals from tournament selection
            let p1 = self.tournament();
            let p2 = self.tournament();

            // Crossover
            let (mut c1, mut c2) = match self.options.xover_type {
                XTYPE::SJ2OX => SJ2OX::apply(&p1, &p2, None, &mut self.makespan),
                XTYPE::SB2OX => SB2OX::apply(&p1, &p2, None, &mut self.makespan),
                XTYPE::BCBX => BCBX::apply(&p1, &p2, None, &mut self.makespan),
            };

            // Mutate
            let mut mutate = |c| {
                if self.rng.gen::<f32>() < self.options.mutation_prob {
                    match self.options.mutation_type {
                        mutation::MTYPE::Shift => mutation::SHIFT::apply(c, &mut self.makespan),
                        mutation::MTYPE::Reverse => mutation::Reverse::apply(c, &mut self.makespan),
                        mutation::MTYPE::Swap => mutation::Swap::apply(c, &mut self.makespan),
                        mutation::MTYPE::Greedy => mutation::Greedy::apply(c, &mut self.makespan),
                    }
                }
            };
            mutate(&mut c1);
            mutate(&mut c2);

            // Local search
            if self.options.local_search {
                ls_ig(&mut c1, &mut self.makespan, self.options.approx_calc as u32);
                ls_ig(&mut c2, &mut self.makespan, self.options.approx_calc as u32)
            }

            let mut makespan = |c: &mut Chromosome| c.makespan(&mut self.makespan);
            makespan(&mut c1);
            makespan(&mut c2);

            // If non of the new chromosomes are better than the current best, the count increases
            if c1 < *self.population.first().unwrap() || c2 < *self.population.first().unwrap() {
                non_improvement_counter = 0;
            } else {
                non_improvement_counter += 1;
            }
            // Check if individuals are better than current worst & not already in population
            let mut replace = |c: Chromosome| {
                if &c < self.population.iter().last().unwrap()
                    && !self
                        .population
                        .iter()
                        .map(|o| &o.jobs)
                        .collect::<Vec<&Vec<u32>>>()
                        .contains(&&c.jobs)
                {
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
        self.mating_pool
            .iter_mut()
            .filter(|c| c.updated)
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
        winner_clone.updated = false;
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
    // Parse arguments (run steady state (-s), run all problems (-r), test all parameters (-a))
    let args = Args::parse();

    // Based on arguments, we either run all problems or a single one
    if args.run_all {
        run_all(&args);
    } else {
        run_one(&args);
    }
}

// Run all problems for all parameter combinations
pub fn run_all(args: &Args) {
    // Get vector of all problem files (twice as we have to consume them)
    let problem_files = get_problem_files(true);
    let problem_files_consumed = get_problem_files(true);

    // Make sure problem files are in same order
    assert_eq!(
        &problem_files, &problem_files_consumed,
        "Order of problem files does not equal"
    );

    let num_problems = problem_files.len();

    // Initiate 2D vector of results: results[problem_file][parameter_combination]
    let mut results: Vec<Vec<String>> = Vec::with_capacity(problem_files.len());

    // Iterate all problem files
    for (i, problem_file) in problem_files.into_iter().enumerate() {
        println!(
            "Running instance {} ({} / {})",
            &problem_files_consumed[i].display(),
            i + 1,
            num_problems
        );

        // Get default options to be used in constructing OptionsGrid
        let options = Options {
            problem_file: Cow::Owned(problem_file),
            run_all: args.run_all,
            steady_state: args.steady_state,
            all_params: args.all_params,
            ..Options::default()
        };

        // Get vector of all option combinations possible
        let all_options = OptionsGrid::default().get_options(options);

        if i == 0 {
            write_params_to_file(
                String::from(params::SOLUTION_FOLDER) + "/params.csv",
                &all_options,
            )
            .unwrap();
        }

        // Store filename and result from each parameter combination in vector
        let mut row = Vec::with_capacity(all_options.len() + 1);

        row.push(String::from(
            problem_files_consumed.get(i).unwrap().to_str().unwrap(),
        ));

        all_options.into_iter().for_each(|options| {
            row.push(run(options, false).to_string());
        });

        results.push(row);
    }

    results.sort_by(|a, b| natural_lexical_cmp(&a[0], &b[0]));

    write_results(
        String::from(params::SOLUTION_FOLDER) + "/results.csv",
        &results,
    )
    .unwrap();
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

fn write_results(filename: String, records: &Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(filename)?;

    for record in records {
        wtr.write_record(record)?;
    }

    wtr.flush()?;
    Ok(())
}

fn write_params_to_file(
    filename: String,
    all_options: &Vec<Options>,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(filename).unwrap();
    all_options
        .iter()
        .map(|o| Params::from(o))
        .for_each(|p| wtr.serialize(p).unwrap());

    wtr.flush()?;
    Ok(())
}
