use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::{Display, Error, Formatter};

use super::problem::Problem;

#[derive(Debug)]
pub struct Chromosome {
    jobs: Vec<u32>,
}

impl Chromosome {
    pub fn new(problem: &Problem) -> Chromosome {
        let mut jobs: Vec<u32> = (0..problem.n_jobs).collect();
        jobs.shuffle(&mut thread_rng());

        Chromosome { jobs }
    }
}

impl Display for Chromosome {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut dash_separated = String::new();

        for num in &self.jobs[0..self.jobs.len() - 1] {
            dash_separated.push_str(&num.to_string());
            dash_separated.push_str("-");
        }

        dash_separated.push_str(&self.jobs[self.jobs.len() - 1].to_string());
        write!(f, "{}", dash_separated)
    }
}
