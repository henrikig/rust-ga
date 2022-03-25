use crate::common::construction::Construction;

use super::operators::{crossover::XTYPE, mutation::MTYPE};

pub const PROBLEM_FILE: &str = "./instances/ruiz/json/n20m2-1.json";
pub const POPULATION_SIZE: usize = 100;
pub const ITERATIONS: usize = 500;
pub const ELITISM: usize = 2;
pub const KEEP_BEST: f32 = 0.8;
pub const NON_IMPROVING_ITERATIONS: usize = 50; // use 50, 100, 150 (because of implementation)
pub const GENOSIDE_SIZE: usize = 80; // Has to be smaller than the population_size
pub const XOVER_PROB: f32 = 0.5;
pub const XOVER: XTYPE = XTYPE::SJ2OX;
pub const CONSTRUCTION: Construction = Construction::MDDR(0.5);

// MUTATION
pub const MUTATION_PROB: f32 = 0.05;
pub const MTYPE: MTYPE = MTYPE::Greedy;
pub const REVERSAL_PERCENT: usize = 10;

// Solution folder for parameter grid search
pub const SOLUTION_FOLDER: &str = "./solutions/";
