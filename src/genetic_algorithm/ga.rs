use crate::common::instance::{Instance, Solution};
use crate::common::makespan::Makespan;
use crate::common::utils;
use crate::genetic_algorithm::entities::options::Args;

use super::entities::chromosome::Chromosome;
use super::entities::options::{Options, OptionsGrid, Params};
use super::operators::crossover::{
    Crossover, CrossoverFn, Qlearning, Random, BCBX, PMX, SB2OX, SJ2OX, XTYPE,
};
use super::operators::local_search::ls_ig;
use super::operators::mutation::{self, Greedy, Mutation, Reverse, Swap, MTYPE, SHIFT};
use super::operators::replacement::{Replacement, RTYPE};
use super::operators::{crowding, replacement};
use super::params;

use csv::Writer;
use lexical_sort::natural_lexical_cmp;
use rand::prelude::StdRng;
use rand::{seq::SliceRandom, Rng};
use rayon::prelude::*;
use std::borrow::Cow;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{error::Error, fs, path::PathBuf};

pub struct GA {
    pub instance: Instance,
    pub population: Vec<Chromosome>,
    pub mating_pool: Vec<Chromosome>,
    pub makespan: Makespan,
    pub options: Options,
    pub rng: StdRng,
    pub best_makespan: Vec<Vec<String>>,
    pub init_duration: Duration,
}

impl Default for GA {
    fn default() -> Self {
        Options::default().build()
    }
}

impl GA {
    pub fn run(&mut self) {
        let crossovers: Vec<CrossoverFn> =
            vec![SJ2OX::apply, BCBX::apply, SB2OX::apply, PMX::apply];
        let mut q_crossover =
            Qlearning::new(crossovers, self.options.learning_rate, self.options.epsilon);

        let mut non_improvement_counter: usize = 0;
        let mut iteration = 0;
        let start_time = Instant::now();
        let duration_millis = utils::get_duration(&self.instance);
        let duration = Duration::from_millis(duration_millis);
        let allowed_time = duration.checked_sub(self.init_duration);

        let time_to_spare = match allowed_time {
            Some(t) => t,
            None => Duration::from_millis(0),
        };

        // Go through generations
        while start_time.elapsed() < time_to_spare {
            // Replace the chromosomes with the worst fit if there has been no improvement in the best fit for y iterations
            if self.options.allways_keep < 1.0
                && non_improvement_counter >= self.options.non_improving_iterations
            {
                let always_keep =
                    (self.population.len() as f64 * self.options.allways_keep) as usize;
                for index in always_keep..self.options.pop_size {
                    self.population[index] = Chromosome::new(&self.instance, &mut self.rng);
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
                        XTYPE::SJ2OX => {
                            SJ2OX::apply(&p[0], &p[1], None, &mut self.makespan, &mut self.rng)
                        }
                        XTYPE::SB2OX => {
                            SB2OX::apply(&p[0], &p[1], None, &mut self.makespan, &mut self.rng)
                        }
                        XTYPE::BCBX => {
                            BCBX::apply(&p[0], &p[1], None, &mut self.makespan, &mut self.rng)
                        }
                        XTYPE::PMX => {
                            PMX::apply(&p[0], &p[1], None, &mut self.makespan, &mut self.rng)
                        }
                        XTYPE::Random => {
                            Random::apply(&p[0], &p[1], None, &mut self.makespan, &mut self.rng)
                        }
                        XTYPE::QLearning => q_crossover.crossover(
                            &p[0],
                            &p[1],
                            None,
                            &mut self.makespan,
                            &mut self.rng,
                        ),
                    };

                    if params::PERFORM_CROWDING {
                        c1.makespan(&mut self.makespan);
                        c2.makespan(&mut self.makespan);

                        let [winner1, winner2] = crowding::survivor_selection(
                            &[c1, c2],
                            p,
                            self.options.crowding_scale,
                            match self.options.distance_metric {
                                crowding::DTYPE::ExactMatch => crowding::DTYPE::ExactMatch,
                                crowding::DTYPE::DeviationDistance => {
                                    crowding::DTYPE::DeviationDistance
                                }
                            },
                            &mut self.rng,
                        );

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
                        MTYPE::Shift => SHIFT::apply(c, &mut self.makespan, &mut self.rng),
                        MTYPE::Reverse => Reverse::apply(c, &mut self.makespan, &mut self.rng),
                        MTYPE::Swap => Swap::apply(c, &mut self.makespan, &mut self.rng),
                        MTYPE::Greedy => Greedy::apply(c, &mut self.makespan, &mut self.rng),
                        MTYPE::Random => {
                            mutation::Random::apply(c, &mut self.makespan, &mut self.rng)
                        }
                    }
                }
            });

            // Local search
            if self.options.local_search {
                self.mating_pool.iter_mut().for_each(|c| {
                    ls_ig(
                        c,
                        &mut self.makespan,
                        self.options.approx_calc as u32,
                        &mut self.rng,
                    )
                });
            }

            // Check if any of the new chromosomes are improvements to the current best
            // Sort population for elitism
            self.population.sort();
            // Calculate makespan for new individuals in mating pool
            self.makespan();

            // Check if any of the new chromosomes are improvements to the current best
            let best_offspring = self.mating_pool.iter().min().unwrap();
            if self.population.first().unwrap() < best_offspring {
                non_improvement_counter += 1;
            } else {
                non_improvement_counter = 0;
                self.best_makespan.push(vec![
                    iteration.to_string(),
                    best_offspring.makespan.unwrap().to_string(),
                    self.makespan.count.to_string(),
                    start_time.elapsed().as_millis().to_string(),
                ]);
            }

            // Elitism
            for c in self.population.iter().take(self.options.elitism) {
                let mut elite = Chromosome::from(c.jobs.to_vec());
                elite.makespan = c.makespan;
                elite.updated = false;
                self.mating_pool.push(elite);
            }

            self.population.clear();

            self.mating_pool.iter().for_each(|c| {
                let mut new_c = Chromosome::from(c.jobs.to_vec());
                new_c.makespan = c.makespan;
                new_c.updated = false;
                self.population.push(new_c);
            });

            iteration += 1;
        }

        self.final_makespan(iteration, duration_millis);

        if params::WRITE_IMPROVEMENT {
            write_improvement(&self.options, &self.best_makespan);
        }
    }

    pub fn run_steady_state(&mut self) {
        let crossovers: Vec<CrossoverFn> =
            vec![SJ2OX::apply, BCBX::apply, SB2OX::apply, PMX::apply];

        let mut q_crossover =
            Qlearning::new(crossovers, self.options.learning_rate, self.options.epsilon);

        // Calculate makespan for all individuals in population
        self.population.sort();
        let mut non_improvement_counter: usize = 0;
        let mut iteration = 0;

        let start_time = Instant::now();
        let duration_millis = utils::get_duration(&self.instance);
        let duration = Duration::from_millis(duration_millis);
        let allowed_time = duration.checked_sub(self.init_duration);

        let time_to_spare = match allowed_time {
            Some(t) => {
                // println!(
                //     "Init took {} ms. Available time {} ms",
                //     self.init_duration.as_millis(),
                //     duration.as_millis(),
                // );
                t
            }
            None => {
                // println!(
                //     "Failing for instance `{:?}`. It took {} ms. Available time {} ms",
                //     self.options.problem_file.as_os_str(),
                //     self.init_duration.as_millis(),
                //     duration.as_millis(),
                // );
                Duration::from_millis(0)
            }
        };

        // Go through generations
        while start_time.elapsed() < time_to_spare {
            // Replace the chromosomes with the worst fit if there has been no improvement in the best fit for y iterations
            if self.options.allways_keep < 1.0
                && non_improvement_counter >= self.options.non_improving_iterations
            {
                match self.options.rtype {
                    RTYPE::Random => replacement::Random::replace(
                        &mut self.population,
                        self.options.allways_keep,
                        &mut self.makespan,
                        &mut self.rng,
                    ),
                    RTYPE::GCH => replacement::GCH::replace(
                        &mut self.population,
                        self.options.allways_keep,
                        &mut self.makespan,
                        &mut self.rng,
                    ),
                    RTYPE::Mutate => replacement::Mutate::replace(
                        &mut self.population,
                        self.options.allways_keep,
                        &mut self.makespan,
                        &mut self.rng,
                    ),
                    _ => (),
                }

                self.population.sort();
            }

            // Select two individuals from tournament selection
            let p1 = self.tournament();
            let p2 = self.tournament();

            // Crossover
            let (mut c1, mut c2) = match self.options.xover_type {
                XTYPE::SJ2OX => SJ2OX::apply(&p1, &p2, None, &mut self.makespan, &mut self.rng),
                XTYPE::SB2OX => SB2OX::apply(&p1, &p2, None, &mut self.makespan, &mut self.rng),
                XTYPE::BCBX => BCBX::apply(&p1, &p2, None, &mut self.makespan, &mut self.rng),
                XTYPE::PMX => PMX::apply(&p1, &p2, None, &mut self.makespan, &mut self.rng),
                XTYPE::Random => Random::apply(&p1, &p2, None, &mut self.makespan, &mut self.rng),
                XTYPE::QLearning => {
                    q_crossover.crossover(&p1, &p2, None, &mut self.makespan, &mut self.rng)
                }
            };

            // Mutate
            let mut mutate = |c| {
                if self.rng.gen::<f32>() < self.options.mutation_prob {
                    match self.options.mutation_type {
                        MTYPE::Shift => SHIFT::apply(c, &mut self.makespan, &mut self.rng),
                        MTYPE::Reverse => Reverse::apply(c, &mut self.makespan, &mut self.rng),
                        MTYPE::Swap => mutation::Swap::apply(c, &mut self.makespan, &mut self.rng),
                        MTYPE::Greedy => Greedy::apply(c, &mut self.makespan, &mut self.rng),
                        MTYPE::Random => {
                            mutation::Random::apply(c, &mut self.makespan, &mut self.rng)
                        }
                    }
                }
            };
            mutate(&mut c1);
            mutate(&mut c2);

            // Local search
            if self.options.local_search {
                ls_ig(
                    &mut c1,
                    &mut self.makespan,
                    self.options.approx_calc as u32,
                    &mut self.rng,
                );
                ls_ig(
                    &mut c2,
                    &mut self.makespan,
                    self.options.approx_calc as u32,
                    &mut self.rng,
                )
            }

            let mut makespan = |c: &mut Chromosome| c.makespan(&mut self.makespan);
            makespan(&mut c1);
            makespan(&mut c2);

            // If non of the new chromosomes are better than the current best, the count increases
            if c1 < *self.population.first().unwrap() || c2 < *self.population.first().unwrap() {
                non_improvement_counter = 0;
                self.best_makespan.push(vec![
                    iteration.to_string(),
                    std::cmp::min(&c1, &c2).makespan.unwrap().to_string(),
                    self.makespan.count.to_string(),
                    (start_time.elapsed() + self.init_duration)
                        .as_millis()
                        .to_string(),
                ]);
            } else {
                non_improvement_counter += 1;
            }

            /*
                REPLACEMENT
                If crowding
                    Find k nearest neighbours to c_i (by use of distance metric)
                    Replace least fit individual of these k individuals with c_i
                Else
                    Replace c_i with least fit element in whole population
            */
            if params::PERFORM_CROWDING {
                let mut replace = |c: Chromosome| {
                    let replace_idx = crowding::k_nearest_replacement(
                        &c,
                        &self.population,
                        self.options.k_nearest,
                        self.options.crowding_scale,
                        match self.options.distance_metric {
                            crowding::DTYPE::ExactMatch => crowding::DTYPE::ExactMatch,
                            crowding::DTYPE::DeviationDistance => {
                                crowding::DTYPE::DeviationDistance
                            }
                        },
                        &mut self.rng,
                    );
                    match replace_idx {
                        Some(idx) => {
                            self.population.remove(idx);
                            let idx = self.population.binary_search(&c).unwrap_or_else(|x| x);
                            self.population.insert(idx, c);
                        }
                        None => (),
                    }
                };
                replace(c1);
                replace(c2);
            } else {
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
            }

            iteration += 1;
        }

        self.final_makespan(iteration, duration_millis);

        if params::WRITE_IMPROVEMENT {
            write_improvement(&self.options, &self.best_makespan);
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
        let mut candidates = Vec::with_capacity(self.options.k_tournament);

        while candidates.len() < self.options.k_tournament {
            candidates.push(self.population.choose(&mut self.rng).unwrap());
        }

        // Choose best in 'keep_best' % of the time, random otherwise
        let winner = if self.rng.gen::<f32>() < self.options.keep_best {
            candidates.iter().min().unwrap()
        } else {
            candidates.choose(&mut self.rng).unwrap()
        };
        // Create a new chromosome from the tournament winner
        let mut winner_clone = Chromosome::from(winner.jobs.to_vec());
        winner_clone.makespan = winner.makespan;
        winner_clone.updated = false;
        winner_clone
    }

    // fn generation_status(&self, iteration: usize) {
    //     println!(
    //         "{}: {}-{}",
    //         iteration,
    //         self.population[0].makespan.unwrap(),
    //         self.population.iter().last().unwrap().makespan.unwrap()
    //     );
    // }

    fn final_makespan(&mut self, iteration: usize, elapsed_time: u64) {
        self.best_makespan.push(vec![
            iteration.to_string(),
            self.population
                .iter()
                .min()
                .unwrap()
                .makespan
                .unwrap()
                .to_string(),
            self.makespan.count.to_string(),
            elapsed_time.to_string(),
        ]);
    }
}

// Run all problems for all parameter combinations
pub fn run_all(args: &Args) {
    // Get vector of all problem files (twice as we have to consume them)
    let problem_files = utils::get_test_problems();
    let problem_files_consumed = utils::get_test_problems();

    // Make sure problem files are in same order
    assert_eq!(
        &problem_files, &problem_files_consumed,
        "Order of problem files does not equal"
    );

    let num_problems = problem_files.len();

    // Initiate 2D vector of results: results[problem_file][parameter_combination]
    let results: Arc<Mutex<Vec<Vec<String>>>> =
        Arc::new(Mutex::new(Vec::with_capacity(problem_files.len())));

    let pb = utils::create_progress_bar(num_problems as u64);

    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    // Iterate all problem files
    problem_files
        .into_par_iter()
        .enumerate()
        .for_each(|(i, problem_file)| {
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

            // Increase progress bar
            pb.inc(1);

            results.lock().unwrap().push(row);
        });

    // pb.finish_with_message("Done");

    results
        .lock()
        .unwrap()
        .sort_by(|a, b| natural_lexical_cmp(&a[0], &b[0]));

    utils::write_results(params::SOLUTION_FOLDER, &results.lock().unwrap()).unwrap();
    println!(
        "All problems run, results are stored in `{}`",
        params::SOLUTION_FOLDER
    );
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
    if run_one && params::WRITE_IMPROVEMENT {
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

fn write_params_to_file(
    filename: String,
    all_options: &Vec<Options>,
) -> Result<(), Box<dyn Error>> {
    match Path::new(params::SOLUTION_FOLDER).is_dir() {
        false => fs::create_dir_all(params::SOLUTION_FOLDER)?,
        _ => (),
    }

    let mut wtr = Writer::from_path(filename).unwrap();

    all_options
        .iter()
        .map(|o| Params::from(o))
        .for_each(|p| wtr.serialize(p).unwrap());

    wtr.flush()?;
    Ok(())
}

fn write_improvement(options: &Options, best_makespan: &Vec<Vec<String>>) {
    let mut filename = PathBuf::from(
        PathBuf::from(options.problem_file.as_os_str())
            .file_name()
            .unwrap(),
    );

    filename.set_extension("csv");

    filename = PathBuf::from("ga/init_gch2").join(filename);

    utils::write_makespan_improvement(filename, best_makespan).unwrap();
}
