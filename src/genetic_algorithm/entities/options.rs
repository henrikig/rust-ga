use clap::Parser;
use itertools::iproduct;
use rand::{prelude::StdRng, SeedableRng};
use serde_derive::Serialize;
use std::{borrow::Cow, path::PathBuf, time::Instant};

use crate::{
    common::{
        construction::{gch::GCH, neh::NEH, Construction},
        instance::parse,
        makespan::Makespan,
    },
    genetic_algorithm::{
        ga::GA,
        operators::{crossover::XTYPE, crowding::DTYPE, mutation::MTYPE, replacement::RTYPE},
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

    // Q-Learning learning rate
    pub learning_rate: f64,

    // Q-Learning learning epsilon
    pub epsilon: f64,

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

    // Replacement scheme
    pub rtype: RTYPE,

    // Percentage of population to keep in case of a genocide
    pub allways_keep: f64,

    // Approximate makespan calculations in each local search
    pub approx_calc: usize,

    // Crowding scale
    pub crowding_scale: f64,

    // Crowding k nearest
    pub k_nearest: usize,

    // Distance metric in crowding
    pub distance_metric: DTYPE,
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
            learning_rate: params::LEARNING_RATE,
            epsilon: params::EPSILON,
            construction: params::CONSTRUCTION,
            mutation_prob: params::MUTATION_PROB,
            mutation_type: params::MTYPE,
            reversal_percent: params::REVERSAL_PERCENT,
            non_improving_iterations: params::NON_IMPROVING_ITERATIONS,
            rtype: params::RTYPE,
            allways_keep: params::ALLWAYS_KEEP,
            approx_calc: params::APPROX_CALC,
            crowding_scale: params::CROWDING_SCALE,
            k_nearest: params::K_NEAREST,
            distance_metric: params::DISTANCE_METRIC,
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

        let mut rng = StdRng::seed_from_u64(123);

        // Calculate initialization duration
        let start_time = Instant::now();

        match self.construction {
            // Add number of chromosomes from MDDR constructor as specified
            Construction::MDDR(num) => {
                let mut constructed: Vec<Chromosome> = GCH {
                    makespan: &mut makespan,
                    rng: &mut rng,
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
            population.push(Chromosome::new(&instance, &mut rng));
        }

        let mut makespan = Makespan::new(&instance);

        // Calculate makespan for initial population
        population
            .iter_mut()
            .filter(|c| c.updated)
            .for_each(|c| c.makespan(&mut makespan));

        let init_duration = start_time.elapsed();

        let mut best_makespan = Vec::new();

        best_makespan.push(vec![
            "0".to_string(),
            population
                .iter()
                .min()
                .unwrap()
                .makespan
                .unwrap()
                .to_string(),
            "0".to_string(),
            "0.0".to_string(),
        ]);

        return GA {
            instance,
            population,
            mating_pool,
            makespan,
            options: self,
            rng,
            best_makespan,
            init_duration,
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

    // Q-Learning learning rate
    pub learning_rates: Vec<f64>,

    // Q-Learning epsilon
    pub epsilons: Vec<f64>,

    // Construction heuristic used for initial population
    pub construction: Vec<Construction>,

    // Probability an individual undergoes mutation
    pub mutation_prob: Vec<f32>,

    // Mutation type to be used
    pub mutation_type: Vec<MTYPE>,

    // How large portion of the jobs to be reversed in reversal mutation
    pub reversal_percent: Vec<usize>,

    pub non_improving_iterations: Vec<usize>,

    // Replacement schemes
    pub rtypes: Vec<RTYPE>,

    pub allways_keep: Vec<f64>,

    pub approx_calc: Vec<usize>,

    // Crowding scale
    pub crowding_scale: Vec<f64>,

    // Crowding k nearest
    pub k_nearest: Vec<usize>,

    // Distance metric in crowding
    pub distance_metric: Vec<DTYPE>,
}

// Set the default values
impl Default for OptionsGrid {
    fn default() -> OptionsGrid {
        OptionsGrid {
            pop_sizes: vec![150],
            elitism: vec![2],
            keep_best: vec![0.8],
            k_tournament: vec![2],
            xover_prob: vec![0.5],
            xover_type: vec![
                // XTYPE::PMX, XTYPE::BCBX, XTYPE::SJ2OX, XTYPE::SB2OX
                XTYPE::QLearning,
            ],
            learning_rates: vec![0.2],
            epsilons: vec![0.25],
            construction: vec![
                // Construction::MDDR(0.2),
                // Construction::MDDR(0.5),
                Construction::MDDR(0.2),
                // Construction::Random,
                // Construction::NEH,
            ],
            mutation_prob: vec![0.1],
            mutation_type: vec![
                // MTYPE::Shift,
                // MTYPE::Greedy,
                // MTYPE::Swap,
                // MTYPE::Reverse,
                MTYPE::Random,
            ],
            reversal_percent: vec![10],
            non_improving_iterations: vec![0],
            rtypes: vec![
                /* RTYPE::Mutate,
                RTYPE::Random,
                RTYPE::GCH, */
                RTYPE::NoReplacement,
            ],
            allways_keep: vec![1.0],
            approx_calc: vec![0],
            crowding_scale: vec![0.0],
            k_nearest: vec![20],
            distance_metric: vec![DTYPE::DeviationDistance],
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
            // self.reversal_percent,
            self.non_improving_iterations,
            // self.crowding_scale,
            // self.k_nearest,
            // self.approx_calc,
            // self.non_improving_iterations,
            // self.learning_rates,
            // self.epsilons,
            self.rtypes,
            self.allways_keep,
            self.distance_metric
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
            // crowding_scale: opt.8,
            // k_nearest: opt.9,
            // approx_calc: opt.8,
            non_improving_iterations: opt.8,
            // learning_rate: opt.8,
            // epsilon: opt.9,
            rtype: opt.9,
            allways_keep: opt.10,
            distance_metric: opt.11,
            // k_tournament: opt.12,
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

    // Tournament size
    pub k_tournament: usize,

    // Probability crossover is performed
    pub xover_prob: f32,

    // Crossover type to be used
    pub xover_type: XTYPE,

    pub learning_rate: f64,
    pub epsilon: f64,

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

    // Replacement scheme
    pub rtype: RTYPE,

    // Percentage of population to keep in case of a genocide
    pub allways_keep: f64,

    pub approx_calc: usize,

    // Crowding scale
    pub crowding_scale: f64,

    // Crowding k nearest
    pub k_nearest: usize,

    // Distance metric in crowding
    pub distance_metric: DTYPE,
}

impl From<&Options> for Params {
    fn from(options: &Options) -> Self {
        Params {
            steady_state: options.steady_state,
            pop_size: options.pop_size,
            elitism: options.elitism,
            keep_best: options.keep_best,
            k_tournament: options.k_tournament,
            xover_prob: options.xover_prob,
            xover_type: match options.xover_type {
                XTYPE::SJ2OX => XTYPE::SJ2OX,
                XTYPE::SB2OX => XTYPE::SB2OX,
                XTYPE::BCBX => XTYPE::BCBX,
                XTYPE::PMX => XTYPE::PMX,
                XTYPE::Random => XTYPE::Random,
                XTYPE::QLearning => XTYPE::QLearning,
            },
            learning_rate: options.learning_rate,
            epsilon: options.epsilon,
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
                MTYPE::Random => MTYPE::Random,
            },
            reversal_percent: options.reversal_percent,
            non_improving_iterations: options.non_improving_iterations,
            rtype: match options.rtype {
                RTYPE::Random => RTYPE::Random,
                RTYPE::GCH => RTYPE::GCH,
                RTYPE::Mutate => RTYPE::Mutate,
                RTYPE::NoReplacement => RTYPE::NoReplacement,
            },
            allways_keep: options.allways_keep,
            approx_calc: options.approx_calc,
            crowding_scale: options.crowding_scale,
            k_nearest: options.k_nearest,
            distance_metric: match options.distance_metric {
                DTYPE::ExactMatch => DTYPE::ExactMatch,
                DTYPE::DeviationDistance => DTYPE::DeviationDistance,
            },
        }
    }
}
