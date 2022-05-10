use crate::{
    common::{best_insertion::find_best_insertion, makespan::Makespan},
    genetic_algorithm::{entities::chromosome::Chromosome, params},
};

use rand::{prelude::StdRng, Rng};
use serde_derive::Serialize;

#[derive(Clone, Serialize)]
pub enum MTYPE {
    Shift,
    Reverse,
    Swap,
    Greedy,
}

pub trait Mutation {
    fn apply(c: &mut Chromosome, m: &mut Makespan, rng: &mut StdRng);
}

pub struct SHIFT;
pub struct Reverse;
pub struct Swap;
pub struct Greedy;

impl Mutation for SHIFT {
    // Move a job from one location to another random location
    fn apply(c: &mut Chromosome, _m: &mut Makespan, rng: &mut StdRng) {
        // Find random job (index) and new location
        let from = rng.gen_range(0..c.jobs.len());
        let to = rng.gen_range(0..c.jobs.len());

        // Remove job from job permutation
        let job = c.jobs.remove(from);
        // Insert it into new place
        c.jobs.insert(to, job);
        c.updated = true;
    }
}

impl Mutation for Reverse {
    // Change order of jobs in the chosen range
    fn apply(c: &mut Chromosome, _m: &mut Makespan, rng: &mut StdRng) {
        let size = c.jobs.len() / params::REVERSAL_PERCENT;

        let start = rng.gen_range(0..c.jobs.len() - size);

        c.jobs[start..start + size].reverse();
        c.updated = true;
    }
}

impl Mutation for Swap {
    // Change order of jobs in the chosen range
    fn apply(c: &mut Chromosome, _m: &mut Makespan, rng: &mut StdRng) {
        let j1 = rng.gen_range(0..c.jobs.len());
        let mut j2 = rng.gen_range(0..c.jobs.len());

        // Choose job 2 again if we picked the same jobs
        while j1 == j2 {
            j2 = rng.gen_range(0..c.jobs.len());
        }

        c.jobs.swap(j1, j2);
        c.updated = true;
    }
}

impl Mutation for Greedy {
    fn apply(c: &mut Chromosome, m: &mut Makespan, rng: &mut StdRng) {
        let rand_job = rng.gen_range(0..c.jobs.len());

        let job = c.jobs.remove(rand_job);

        let (new_jobs, makespan) = find_best_insertion(c.jobs.to_vec(), &[job], m, false, rng);

        c.jobs = new_jobs;
        c.makespan = Some(makespan);
    }
}

#[cfg(test)]
mod test {
    use rand::{prelude::StdRng, SeedableRng};

    use crate::{
        common::makespan::Makespan,
        genetic_algorithm::{
            entities::chromosome::Chromosome,
            operators::mutation::{Mutation, Reverse, Swap},
            tests::tests::test_instance,
        },
    };

    use super::Greedy;

    #[test]
    fn test_reverse() {
        // Generate two identical chromosomes
        let mut jobs_mutate = Chromosome::from((0..20).collect::<Vec<u32>>());
        let jobs_compare = Chromosome::from((0..20).collect::<Vec<u32>>());

        assert_eq!(jobs_mutate.jobs.len(), 20);

        let mut rng = StdRng::seed_from_u64(123);

        // Apply reversal mutation
        Reverse::apply(
            &mut jobs_mutate,
            &mut Makespan::new(&test_instance()),
            &mut rng,
        );

        // Count the number of jobs that are now equal
        let count = jobs_mutate
            .jobs
            .iter()
            .zip(jobs_compare.jobs.iter())
            .filter(|(&j1, &j2)| j1 == j2)
            .count();

        // Two of the jobs in the first chromosomes should have changed
        // Thus, 18 of the jobs should equal
        assert_eq!(count, 18);
    }

    #[test]
    fn test_swap() {
        let mut c = Chromosome::from((0..10).collect::<Vec<u32>>());

        let j1 = 0;
        let j2 = 9;

        c.jobs.swap(j1, j2);

        assert_eq!(c.jobs, vec![9, 1, 2, 3, 4, 5, 6, 7, 8, 0]);

        let mut jobs_mutate = Chromosome::from((0..20).collect::<Vec<u32>>());
        let jobs_compare = Chromosome::from((0..20).collect::<Vec<u32>>());

        let mut rng = StdRng::seed_from_u64(123);

        Swap::apply(
            &mut jobs_mutate,
            &mut Makespan::new(&test_instance()),
            &mut rng,
        );

        // Count the number of jobs that are now equal
        let count = jobs_mutate
            .jobs
            .iter()
            .zip(jobs_compare.jobs.iter())
            .filter(|(&j1, &j2)| j1 == j2)
            .count();

        // Two of the jobs in the first chromosome should have changed
        // Thus, 18 of the jobs should equal
        assert_eq!(count, 18);
    }

    #[test]
    fn test_greedy() {
        let instance = test_instance();
        let mut makespan = Makespan::new(&instance);
        let mut c = Chromosome::from((0..instance.jobs).collect::<Vec<u32>>());

        // Calculate makespan before mutation
        let (makespan_before, _) = makespan.makespan(&c.jobs);

        let mut rng = StdRng::seed_from_u64(123);

        // Apply mutation
        Greedy::apply(&mut c, &mut makespan, &mut rng);

        // Calculate makespan after mutation
        let (makespan_after, _) = makespan.makespan(&c.jobs);

        // Makespan after mutation should be _at least_ as good as makespan before mutation
        assert!(makespan_before >= makespan_after);
    }
}
