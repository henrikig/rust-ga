use crate::common::instance::Instance;
use crate::common::makespan;
use crate::genetic_algorithm::entities::chromosome::Chromosome;
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::Constructor;

pub struct MDDR<'a> {
    pub instance: &'a Instance,
}

impl Constructor for MDDR<'_> {
    fn create(&self) -> Chromosome {
        // Get a random input vector of jobs
        let mut jobs: Vec<u32> = (0..self.instance.jobs).collect();
        jobs.shuffle(&mut thread_rng());

        // Initialise empty vector of output jobs
        let mut c: Vec<u32> = Vec::with_capacity(self.instance.jobs as usize);

        // Add last job to output jobs (backward order due to O(n) worst-case complexity of .remove())
        c.push(jobs.remove(jobs.len() - 1));

        // For each remaining job in the random order
        while jobs.len() > 0 {
            let job = jobs.remove(jobs.len() - 1);

            // Find current job's best insertion point in output jobs
            c.insert(0, job);
            let mut best_order: Vec<u32> = c.iter().cloned().collect();
            let mut best_makespan = makespan::makespan(&best_order, self.instance);

            for i in 0..c.len() - 1 {
                c[i..i + 2].rotate_right(1);

                let makespan = makespan::makespan(&c, self.instance);

                if makespan < best_makespan {
                    best_makespan = makespan;
                    best_order = c.iter().cloned().collect();
                }
            }
            // Add job to this location and proceed
            c = best_order;

            // Return a new chromosome from the given permutation
        }

        Chromosome::from(c)
    }
}

impl Iterator for MDDR<'_> {
    type Item = Chromosome;

    fn next(&mut self) -> Option<Chromosome> {
        Some(MDDR::create(self))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        common::parser::parse,
        genetic_algorithm::{
            entities::chromosome::Chromosome, params, tests::tests::test_instance,
        },
    };

    use super::MDDR;

    #[test]
    fn test_heuristic_performance() {
        let instance = parse(params::PROBLEM_FILE).unwrap();

        let constructor = MDDR {
            instance: &instance,
        };

        let mut heuristic: Vec<Chromosome> = constructor.take(10).collect();
        let mut random: Vec<Chromosome> = Vec::new();
        for _ in 0..10 {
            random.push(Chromosome::new(&instance));
        }

        heuristic.iter_mut().for_each(|c| c.makespan(&instance));
        random.iter_mut().for_each(|c| c.makespan(&instance));

        assert!(
            heuristic.iter().map(|s| s.makespan.unwrap()).sum::<u32>()
                <= random.iter().map(|s| s.makespan.unwrap()).sum::<u32>()
        )
    }
}
