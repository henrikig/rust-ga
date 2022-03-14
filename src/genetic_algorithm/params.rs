use crate::common::construction::Construction;

use super::operators::crossover::XTYPE;

pub const PROBLEM_FILE: &str = "./instances/ruiz/json/n20m2-1.json";
pub const POPULATION_SIZE: usize = 50;
pub const ITERATIONS: usize = 120;
pub const ELITISM: usize = 2;
pub const KEEP_BEST: f32 = 0.8;
pub const XOVER_PROB: f32 = 0.5;
pub const XOVER: XTYPE = XTYPE::_BCBC;
pub const MUTATION_PROB: f32 = 0.05;
pub const CONSTRUCTION: Construction = Construction::_MDDR(20);
