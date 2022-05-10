use rand::{
    prelude::{SliceRandom, StdRng},
    Rng,
};

use crate::genetic_algorithm::params;

use super::makespan::Makespan;

/// Inserts a block (size >= 1 jobs) to the best position, or a random one if desired
///
///
/// # Arguments
///
/// * jobs - The current (partial) job permutation
/// * block - The (remaining) jobs to insert into the current permutation
/// * makespan - Makespan struct for makespan calculation
/// * allow_rnd - whether to allow returning a random insertion
pub fn find_best_insertion(
    jobs: Vec<u32>,
    block: &[u32],
    makespan: &mut Makespan,
    allow_rnd: bool,
    rng: &mut StdRng,
) -> (Vec<u32>, u32) {
    let n_jobs = jobs.len();
    let mut jobs: Vec<u32> = block.iter().cloned().chain(jobs.iter().cloned()).collect();
    let (mut best_makespan, _) = makespan.makespan(&jobs);
    let k = block.len();

    // Store one random solution which may be returned
    let random_idx: usize = rng.gen_range(0..n_jobs);
    let mut random_jobs: Vec<u32> = jobs.iter().cloned().collect();
    let mut random_makespan = u32::MAX;

    // Initialise best jobs
    let mut best_jobs: Vec<Vec<u32>> = vec![jobs.iter().cloned().collect()];

    for i in 0..n_jobs {
        // Shift block one step to the right
        jobs[i..i + k + 1].rotate_right(1);
        let (new_makespan, _) = makespan.makespan(&jobs);

        // Update random solution if we are at the random index
        if allow_rnd && i == random_idx {
            random_jobs = jobs.iter().cloned().collect();
            random_makespan = new_makespan;
        }

        // Update best jobs if the current has a lower makespan
        if new_makespan < best_makespan {
            best_makespan = new_makespan;
            best_jobs = vec![jobs.iter().cloned().collect()];
        // Add current to best jobs if makespan is equal to best jobs
        } else if new_makespan == best_makespan {
            best_jobs.push(jobs.iter().cloned().collect());
        }
    }

    // Return either one of the best solutions, or the chosen random solution
    if allow_rnd && rng.gen::<f32>() >= params::KEEP_BEST {
        (random_jobs, random_makespan)
    } else {
        (best_jobs.choose(rng).unwrap().to_vec(), best_makespan)
    }
}
