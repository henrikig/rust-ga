use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{
    cmp::Ordering,
    fmt::{Display, Error, Formatter},
};

use crate::common::{instance::Instance, makespan::Makespan};

#[derive(Debug, Eq, Clone)]
pub struct Chromosome {
    pub jobs: Vec<u32>,
    pub makespan: Option<u32>,
    pub updated: bool,
}

impl Chromosome {
    pub fn new(instance: &Instance) -> Chromosome {
        let mut jobs: Vec<u32> = (0..instance.jobs).collect();
        jobs.shuffle(&mut thread_rng());

        Chromosome {
            jobs,
            makespan: None,
            updated: true,
        }
    }

    pub fn new_with_makespan(jobs: Vec<u32>, m: u32) -> Self {
        Chromosome {
            jobs: jobs.to_vec(),
            makespan: Some(m),
            updated: false,
        }
    }

    pub fn makespan(&mut self, mks: &mut Makespan) {
        let (m, _) = mks.makespan(&self.jobs);
        self.makespan = Some(m);
        self.updated = false;
    }
}

impl Ord for Chromosome {
    fn cmp(&self, other: &Self) -> Ordering {
        self.makespan.unwrap().cmp(&other.makespan.unwrap())
    }
}

impl PartialOrd for Chromosome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        self.makespan == other.makespan
    }
}

impl From<Vec<u32>> for Chromosome {
    fn from(jobs: Vec<u32>) -> Self {
        Chromosome {
            jobs: jobs.to_vec(),
            makespan: None,
            updated: true,
        }
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

        match self.makespan {
            Some(m) => dash_separated.push_str(&format!(" (makespan: {})", m)[..]),
            _ => (),
        };

        write!(f, "{}", dash_separated)
    }
}
