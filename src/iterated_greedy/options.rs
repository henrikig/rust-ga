use std::{borrow::Cow, path::PathBuf};

use itertools::iproduct;

use crate::genetic_algorithm::params;

#[derive(Clone, Debug)]
pub struct Options {
    // Path to input file defining problem
    pub problem_file: Cow<'static, PathBuf>,

    // Temperature
    pub temp: f64,

    // Number of jobs to remove
    pub block_size: i32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            problem_file: Cow::Owned(PathBuf::from(params::PROBLEM_FILE)),
            temp: 0.5,
            block_size: 2,
        }
    }
}

// Struct containing all possible parameter values for each option
pub struct OptionsGrid {
    // Temperature
    pub temp: Vec<f64>,

    // Number of jobs to remove
    pub block_size: Vec<i32>,
}

// Set the default values
impl Default for OptionsGrid {
    fn default() -> OptionsGrid {
        OptionsGrid {
            temp: vec![0.1],
            block_size: vec![2],
        }
    }
}

impl OptionsGrid {
    pub fn get_options(self, options: Options) -> Vec<Options> {
        iproduct!(self.temp, self.block_size)
            .map(|opt| Options {
                temp: opt.0,
                block_size: opt.1,
                problem_file: Cow::Owned(options.problem_file.as_ref().clone()),
            })
            .collect()
    }
}
