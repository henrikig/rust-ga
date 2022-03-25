use clap::Parser;
use itertools::iproduct;
use rand::thread_rng;
use serde_derive::Serialize;
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

    // Runs in steady state if true
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

    // Iterations to run in steady state before changing the less fit chromosomes
    pub non_improving_iterations: usize,

    // Chromosomes to keep in case of a genocide
    // Must be smaller than pop_size
    pub allways_keep: usize,
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
            non_improving_iterations: params::NON_IMPROVING_ITERATIONS,
            allways_keep: params::GENOSIDE_SIZE,
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

// Struct containing all possible parameter values for each option
pub struct OptionsGrid {
    // Size of the population
    pub pop_sizes: Vec<usize>,

    // Number of best individuals unconditionally proceeding to next generation
    pub elitism: Vec<usize>,

    // Probability the best offspring is kept
    pub keep_best: Vec<f32>,

    // Probability crossover is performed
    pub xover_prob: Vec<f32>,

    // Crossover type to be used
    pub xover_type: Vec<XTYPE>,

    // Construction heuristic used for initial population
    pub construction: Vec<Construction>,

    // Probability an individual undergoes mutation
    pub mutation_prob: Vec<f32>,

    // Mutation type to be used
    pub mutation_type: Vec<MTYPE>,

    // How large portion of the jobs to be reversed in reversal mutation
    pub reversal_percent: Vec<usize>,

    pub non_improving_iterations: Vec<usize>,

    pub allways_keep: Vec<usize>,
}

// Set the default values
impl Default for OptionsGrid {
    fn default() -> OptionsGrid {
        OptionsGrid {
            pop_sizes: vec![50, 100],
            elitism: vec![2],
            keep_best: vec![0.8, 1.0],
            xover_prob: vec![0.2, 0.4],
            xover_type: vec![XTYPE::_SJ2OX, XTYPE::_SB2OX, XTYPE::_BCBX],
            construction: vec![
                // Construction::_MDDR(20),
                Construction::_MDDR(50),
                Construction::_Random,
            ],
            mutation_prob: vec![0.2],
            mutation_type: vec![MTYPE::_Greedy, MTYPE::_Shift, MTYPE::_Swap, MTYPE::_Reverse],
            reversal_percent: vec![10],
            non_improving_iterations: vec![50, 100, 150],
            allways_keep: vec![40],
        }
    }
}

impl OptionsGrid {
    pub fn get_options(self, options: Options) -> Vec<Options> {
        iproduct!(
            self.pop_sizes,
            self.elitism,
            self.keep_best,
            self.xover_prob,
            self.xover_type,
            self.construction,
            self.mutation_prob,
            self.mutation_type,
            self.reversal_percent,
            self.non_improving_iterations,
            self.allways_keep
        )
        .map(|opt| Options {
            pop_size: opt.0,
            elitism: opt.1,
            keep_best: opt.2,
            xover_prob: opt.3,
            xover_type: opt.4,
            construction: opt.5,
            mutation_prob: opt.6,
            mutation_type: opt.7,
            reversal_percent: opt.8,
            non_improving_iterations: opt.9,
            allways_keep: opt.10,
            problem_file: Cow::Owned(options.problem_file.as_ref().clone()),
            ..options
        })
        .collect()
    }
}

#[derive(Clone, Serialize)]
pub struct Params {
    // Runs in steady state if true
    pub steady_state: bool,

    // Size of the population
    pub pop_size: usize,

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

impl From<&Options> for Params {
    fn from(options: &Options) -> Self {
        Params {
            steady_state: options.steady_state,
            pop_size: options.pop_size,
            elitism: options.elitism,
            keep_best: options.keep_best,
            xover_prob: options.xover_prob,
            xover_type: match options.xover_type {
                XTYPE::_SJ2OX => XTYPE::_SJ2OX,
                XTYPE::_SB2OX => XTYPE::_SB2OX,
                XTYPE::_BCBX => XTYPE::_BCBX,
            },
            construction: match options.construction {
                Construction::_Random => Construction::_Random,
                Construction::_MDDR(num) => Construction::_MDDR(num),
            },
            mutation_prob: options.mutation_prob,
            mutation_type: match options.mutation_type {
                MTYPE::_Shift => MTYPE::_Shift,
                MTYPE::_Reverse => MTYPE::_Reverse,
                MTYPE::_Swap => MTYPE::_Swap,
                MTYPE::_Greedy => MTYPE::_Greedy,
            },
            reversal_percent: options.reversal_percent,
        }
    }
}
