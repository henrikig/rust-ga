use crate::genetic_algorithm::entities::chromosome::Chromosome;

use rand::Rng;

pub trait Mutation {
    fn apply(c: &mut Chromosome);
}

pub struct SHIFT;

impl Mutation for SHIFT {
    fn apply(c: &mut Chromosome) {
        // Find random job (index) and new location
        let from = rand::thread_rng().gen_range(0..c.jobs.len());
        let to = rand::thread_rng().gen_range(0..c.jobs.len());

        // Remove job from job permutation
        let job = c.jobs.remove(from);
        // Insert it into new place
        c.jobs.insert(to, job);
    }
}
