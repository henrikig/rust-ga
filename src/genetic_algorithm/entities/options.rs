use clap::Parser;
use rand::thread_rng;
use std::{borrow::Cow, path::PathBuf};

use crate::{
    common::{
        construction::{mddr::MDDR, Construction},
        instance::parse,
        makespan::Makespan,
    },
    genetic_algorithm::{
        ga::GA,
        operators::{crossover::XTYPE, mutation::MTYPE},
        params,
    },
};

use super::chromosome::Chromosome;

/// Genetic algorithm configuration
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Run all problem instances
    #[clap(short, long)]
    pub run_all: bool,

    /// Run through all possible parameter values
    #[clap(short, long)]
    pub all_params: bool,

    /// Steady state generational scheme
    #[clap(short, long)]
    pub steady_state: bool,
}

#[derive(Clone)]
pub struct Options {
    // Path to input file defining problem
    pub problem_file: Cow<'static, PathBuf>,

    // Runs through all problem files if true
    pub run_all: bool,

    // Run through all possible parameter values
    pub all_params: bool,

    // Runs through all problem files if true
    pub steady_state: bool,

    // Size of the population
    pub pop_size: usize,

    // Number of iterations in main loop
    pub iterations: usize,

    // Number of best individuals unconditionally proceeding to next generation
    pub elitism: usize,

    // Probability the best offspring is kept
    pub keep_best: f32,

    // Probability crossover is performed
    pub xover_prob: f32,

    // Crossover type to be used
    pub xover_type: XTYPE,

    // Construction heuristic used for initial population
    pub construction: Construction,

    // Probability an individual undergoes mutation
    pub mutation_prob: f32,

    // Mutation type to be used
    pub mutation_type: MTYPE,

    // How large portion of the jobs to be reversed in reversal mutation
    pub reversal_percent: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            problem_file: Cow::Owned(PathBuf::from(params::PROBLEM_FILE)),
            run_all: false,
            all_params: false,
            steady_state: false,
            pop_size: params::POPULATION_SIZE,
            iterations: params::ITERATIONS,
            elitism: params::ELITISM,
            keep_best: params::KEEP_BEST,
            xover_prob: params::XOVER_PROB,
            xover_type: params::XOVER,
            construction: params::CONSTRUCTION,
            mutation_prob: params::MUTATION_PROB,
            mutation_type: params::MTYPE,
            reversal_percent: params::REVERSAL_PERCENT,
        }
    }
}

impl Options {
    pub fn build(self) -> GA {
        // Parse specified instance
        let instance = parse(self.problem_file.as_ref()).unwrap();

        // Create makespan struct from instance
        let mut makespan = Makespan::new(&instance);

        // Initiate population and mating pools
        let mut population = Vec::with_capacity(self.pop_size);
        let mating_pool = Vec::with_capacity(self.pop_size);

        // Add number of chromosomes from MDDR constructor as specified
        match self.construction {
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
        while population.len() < self.pop_size {
            population.push(Chromosome::new(&instance));
        }

        let makespan = Makespan::new(&instance);

        let rng = thread_rng();

        return GA {
            instance,
            population,
            mating_pool,
            makespan,
            options: self,
            rng,
        };
    }
}
