use crate::common::best_insertion::find_best_insertion;
use crate::common::makespan::Makespan;
use crate::genetic_algorithm::entities::chromosome::Chromosome;
use rand::prelude::{SliceRandom, StdRng};

use super::Constructor;

// Greedy Construction Heuristic (GCH)
pub struct GCH<'a> {
    pub makespan: &'a mut Makespan,
    pub rng: &'a mut StdRng,
}

impl Constructor for GCH<'_> {
    fn create(&mut self) -> Chromosome {
        // Get a random input vector of jobs
        let mut jobs: Vec<u32> = (0..self.makespan.instance.jobs).collect();
        jobs.shuffle(self.rng);

        // Initialise empty vector of output jobs
        let mut c: Vec<u32> = Vec::with_capacity(self.makespan.instance.jobs as usize);
        let mut makespan = u32::MAX;

        // Add last job to output jobs (backward order due to O(n) worst-case complexity of .remove())
        c.push(jobs.remove(jobs.len() - 1));

        // For each remaining job in the random order
        while jobs.len() > 0 {
            let job = jobs.remove(jobs.len() - 1);

            // Find current job's best insertion point in output jobs
            (c, makespan) = find_best_insertion(c, &[job], &mut self.makespan, false, self.rng);
        }

        let mut c = Chromosome::from(c);
        c.makespan = Some(makespan);
        c.updated = false;
        return c;
    }
}

impl Iterator for GCH<'_> {
    type Item = Chromosome;

    fn next(&mut self) -> Option<Chromosome> {
        Some(GCH::create(self))
    }
}

#[cfg(test)]
mod test {
    use rand::{prelude::StdRng, SeedableRng};

    use crate::{
        common::{instance::parse, makespan::Makespan},
        genetic_algorithm::{entities::chromosome::Chromosome, params},
    };

    use super::GCH;

    #[test]
    fn test_heuristic_performance() {
        let instance = parse(params::PROBLEM_FILE).unwrap();

        let mut makespan = Makespan::new(&instance);

        let mut rng = StdRng::seed_from_u64(123);

        let constructor = GCH {
            makespan: &mut makespan,
            rng: &mut rng,
        };

        let mut makespan = Makespan::new(&instance);

        let mut heuristic: Vec<Chromosome> = constructor.take(10).collect();
        let mut random: Vec<Chromosome> = Vec::new();
        for _ in 0..10 {
            random.push(Chromosome::new(&instance, &mut rng));
        }

        heuristic.iter_mut().for_each(|c| c.makespan(&mut makespan));
        random.iter_mut().for_each(|c| c.makespan(&mut makespan));

        assert!(
            heuristic.iter().map(|s| s.makespan.unwrap()).sum::<u32>()
                <= random.iter().map(|s| s.makespan.unwrap()).sum::<u32>()
        )
    }

    #[test]
    fn test_number_unique_solutions() {
        let instance = parse(params::PROBLEM_FILE).unwrap();

        let mut makespan = Makespan::new(&instance);

        let mut rng = StdRng::seed_from_u64(123);

        let constructor = GCH {
            makespan: &mut makespan,
            rng: &mut rng,
        };

        let mut population: Vec<Chromosome> = constructor.take(100).collect();

        let mut already_seen = Vec::new();

        // See how many unique permutations are in the population
        population.retain(|c| match already_seen.contains(&c.jobs) {
            true => false,
            _ => {
                already_seen.push(c.jobs.to_vec());
                true
            }
        });

        // Assure there is some diversity
        assert!(population.len() > 50)
    }
}
