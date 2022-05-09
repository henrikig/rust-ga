use crate::common::construction::Construction;

use super::operators::{crossover::XTYPE, mutation::MTYPE};

pub const PROBLEM_FILE: &str = "./instances/ruiz/json/n120m8-01.json";
// pub const PROBLEM_FILE: &str = "./instances/ruiz/json/n120m8-01.json";
pub const IMPROVEMENT_FILE: &str = "./solutions/improvement/ga/pmx/n120m8-01.csv";
pub const WRITE_IMPROVEMENT: bool = true;
pub const POPULATION_SIZE: usize = 150;
pub const ITERATIONS: usize = 180;
pub const ELITISM: usize = 2;
pub const LOCAL_SEARCH: bool = false;
pub const KEEP_BEST: f32 = 0.8;
pub const K_TOURNAMENT: usize = 2;
pub const XOVER_PROB: f32 = 0.5;
pub const XOVER: XTYPE = XTYPE::PMX;
pub const CONSTRUCTION: Construction = Construction::MDDR(1.0);
pub const NON_IMPROVING_ITERATIONS: usize = 100; // use 50, 100, 150 (because of implementation)
pub const ALLWAYS_KEEP: f64 = 1.0; // Percentage of population to always keep
pub const APPROX_CALC: usize = 300;

// MUTATION
pub const MUTATION_PROB: f32 = 0.05;
pub const MTYPE: MTYPE = MTYPE::Greedy;
pub const REVERSAL_PERCENT: usize = 10;

// CROWDING
pub const PERFORM_CROWDING: bool = false;
pub const CROWDING_SCALE: f64 = 0.5;
pub const K_NEAREST: usize = 10; // Only used in steady state crowding version

// Solution folder for parameter grid search

pub const SOLUTION_FOLDER: &str = "./solutions/improvement/ga/pmx";
