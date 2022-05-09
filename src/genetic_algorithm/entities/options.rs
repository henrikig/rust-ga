use clap::Parser;
use itertools::iproduct;
use rand::thread_rng;
use serde_derive::Serialize;
use std::{borrow::Cow, path::PathBuf};

use crate::{
    common::{
        construction::{gch::GCH, neh::NEH, Construction},
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

    /// Local search after mutation
    #[clap(short, long)]
    pub local_search: bool,

    /// Run all problem instances
    #[clap(short, long)]
    pub mddr: bool,

    /// Run all problem instances
    #[clap(short, long)]
    pub neh: bool,

    /// Run all problem instances
    #[clap(short, long)]
    pub iterated_greedy: bool,
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

    // Local search after mutations
    pub local_search: bool,

    // Size of the population
    pub pop_size: usize,

    // Number of iterations in main loop
    pub iterations: usize,

    // Number of best individuals unconditionally proceeding to next generation
    pub elitism: usize,

    // Probability the best offspring is kept
    pub keep_best: f32,

    // Number of individuals in tournament
    pub k_tournament: usize,

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
    pub allways_keep: f64,

    // Approximate makespan calculations in each local search
    pub approx_calc: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            problem_file: Cow::Owned(PathBuf::from(params::PROBLEM_FILE)),
            run_all: false,
            all_params: false,
            steady_state: false,
            local_search: params::LOCAL_SEARCH,
            pop_size: params::POPULATION_SIZE,
            iterations: params::ITERATIONS,
            elitism: params::ELITISM,
            keep_best: params::KEEP_BEST,
            k_tournament: params::K_TOURNAMENT,
            xover_prob: params::XOVER_PROB,
            xover_type: params::XOVER,
            construction: params::CONSTRUCTION,
            mutation_prob: params::MUTATION_PROB,
            mutation_type: params::MTYPE,
            reversal_percent: params::REVERSAL_PERCENT,
            non_improving_iterations: params::NON_IMPROVING_ITERATIONS,
            allways_keep: params::ALLWAYS_KEEP,
            approx_calc: params::APPROX_CALC,
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

        match self.construction {
            // Add number of chromosomes from MDDR constructor as specified
            Construction::MDDR(num) => {
                let mut constructed: Vec<Chromosome> = GCH {
                    makespan: &mut makespan,
                }
                .take((self.pop_size as f32 * num) as usize)
                .collect();

                population.append(&mut constructed);
            }
            // Add one chromosome based on NEH
            Construction::NEH => {
                let (neh_permutation, mks) = NEH::neh(&mut makespan);
                let mut neh_chromosome = Chromosome::from(neh_permutation);
                neh_chromosome.makespan = Some(mks);
                neh_chromosome.updated = false;
                population.push(neh_chromosome);
            }
            _ => (),
        }

        // Remaining chromosomes are added randomly
        while population.len() < self.pop_size {
            population.push(Chromosome::new(&instance));
        }

        let mut makespan = Makespan::new(&instance);

        // Calculate makespan for initial population
        population
            .iter_mut()
            .for_each(|c| c.makespan(&mut makespan));

        let rng = thread_rng();

        return GA {
            instance,
            population,
            mating_pool,
            makespan,
            options: self,
            rng,
            best_makespan: Vec::new(),
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

    // Number individuals in tournament
    pub k_tournament: Vec<usize>,

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

    pub allways_keep: Vec<f64>,

    pub approx_calc: Vec<usize>,
}

// Set the default values
impl Default for OptionsGrid {
    fn default() -> OptionsGrid {
        OptionsGrid {
            pop_sizes: vec![150],
            elitism: vec![2],
            keep_best: vec![0.8],
            k_tournament: vec![2, 3, 5, 8],
            xover_prob: vec![0.5],
            xover_type: vec![
                XTYPE::PMX, // XTYPE::BCBX, XTYPE::SJ2OX, XTYPE::SB2OX,
            ],
            construction: vec![
                // Construction::MDDR(0.5),
                // Construction::MDDR(0.8),
                // Construction::MDDR(1.0),
                // Construction::NEH,
                Construction::Random,
            ],
            mutation_prob: vec![0.05],
            mutation_type: vec![
                MTYPE::Swap, // MTYPE::Greedy, MTYPE::Shift, MTYPE::Reverse
            ],
            reversal_percent: vec![10],
            non_improving_iterations: vec![150],
            allways_keep: vec![1.0],
            approx_calc: vec![100],
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
            self.allways_keep,
            //self.approx_calc,
            self.k_tournament
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
            //approx_calc: opt.11,
            k_tournament: opt.11,
            problem_file: Cow::Owned(options.problem_file.as_ref().clone()),
            ..options
        })
        .collect()
    }
}

// Helper struct which is written to file when all problem files are run
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

    // Iterations to run in steady state before changing the less fit chromosomes
    pub non_improving_iterations: usize,

    // Chromosomes to keep in case of a genocide
    // Must be smaller than pop_size
    pub allways_keep: f64,

    pub approx_calc: usize,
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
                XTYPE::SJ2OX => XTYPE::SJ2OX,
                XTYPE::SB2OX => XTYPE::SB2OX,
                XTYPE::BCBX => XTYPE::BCBX,
                XTYPE::PMX => XTYPE::PMX,
            },
            construction: match options.construction {
                Construction::Random => Construction::Random,
                Construction::MDDR(num) => Construction::MDDR(num),
                Construction::NEH => Construction::NEH,
            },
            mutation_prob: options.mutation_prob,
            mutation_type: match options.mutation_type {
                MTYPE::Shift => MTYPE::Shift,
                MTYPE::Reverse => MTYPE::Reverse,
                MTYPE::Swap => MTYPE::Swap,
                MTYPE::Greedy => MTYPE::Greedy,
            },
            reversal_percent: options.reversal_percent,
            non_improving_iterations: options.non_improving_iterations,
            allways_keep: options.allways_keep,
            approx_calc: options.approx_calc,
        }
    }
}
