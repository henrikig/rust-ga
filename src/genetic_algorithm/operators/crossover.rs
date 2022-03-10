use crate::{
    common::{instance::Instance, makespan},
    genetic_algorithm::{entities::chromosome::Chromosome, params},
};

use rand::{prelude::SliceRandom, Rng};

pub enum XTYPE {
    _SJOX,
    _SB2OX,
    _BCBC,
}

pub trait Crossover {
    fn apply(
        p1: &Chromosome,
        p2: &Chromosome,
        k: Option<usize>,
        instance: &Instance,
    ) -> (Chromosome, Chromosome);
}

pub struct SJOX;
pub struct SB2OX;
pub struct BCBC;

impl Crossover for SJOX {
    fn apply(
        p1: &Chromosome,
        p2: &Chromosome,
        k: Option<usize>,
        _instance: &Instance,
    ) -> (Chromosome, Chromosome) {
        // Generate new permutations based on parents
        let mut c1: Vec<u32> = vec![u32::MAX; p1.jobs.len()];
        let mut c2: Vec<u32> = vec![u32::MAX; p2.jobs.len()];

        // Draw a cut point, k, from range [0, n_jobs)
        let k = match k {
            Some(k) => k,
            None => rand::thread_rng().gen_range(0..p1.jobs.len()),
        };

        // Copy elements before cut point from p1, p2 directly to respective children c1, c2
        c1[0..k].copy_from_slice(&p1.jobs[0..k]);
        c2[0..k].copy_from_slice(&p2.jobs[0..k]);

        // Store non-similar jobs in correct order
        let mut p1_order: Vec<u32> = Vec::new();
        let mut p2_order: Vec<u32> = Vec::new();

        // Fill in all building blocks - sequences where both parents have the same
        // jobs in same positions in range [k, n_jobs]
        p1.jobs
            .iter()
            .zip(p2.jobs.iter())
            .enumerate()
            .for_each(|(i, (j1, j2))| {
                if j1 == j2 {
                    c1[i] = *j1;
                    c2[i] = *j1;
                } else {
                    // TODO: do this in another way for performance boost
                    if !c2.contains(j1) {
                        p1_order.push(*j1);
                    }
                    if !c1.contains(j2) {
                        p2_order.push(*j2);
                    }
                }
            });

        // Insert the jobs that are yet not allocated in order of opposite parent
        for i in k..c1.len() {
            if c1[i] == u32::MAX {
                c1[i] = p2_order.remove(0);
                c2[i] = p1_order.remove(0);
            }
        }

        return (Chromosome::from(c1), Chromosome::from(c2));
    }
}

impl Crossover for SB2OX {
    fn apply(
        p1: &Chromosome,
        p2: &Chromosome,
        _k: Option<usize>,
        _instance: &Instance,
    ) -> (Chromosome, Chromosome) {
        // Generate new permutations based on parents
        let mut c1: Vec<u32> = vec![u32::MAX; p1.jobs.len()];
        let mut c2: Vec<u32> = vec![u32::MAX; p2.jobs.len()];

        // Draw two different cut points, k1, k2
        let k1 = rand::thread_rng().gen_range(0..p1.jobs.len());
        let k2 = rand::thread_rng().gen_range(0..p1.jobs.len());
        let start = std::cmp::min(k1, k2);
        let stop = std::cmp::max(k1, k2);

        // Copy elements between cut points from p1, p2 directly to respective children c1, c2
        c1[start..stop].copy_from_slice(&p1.jobs[start..stop]);
        c2[start..stop].copy_from_slice(&p2.jobs[start..stop]);

        // Store non-similar jobs in correct order
        let mut p1_order: Vec<u32> = Vec::new();
        let mut p2_order: Vec<u32> = Vec::new();

        // Fill in all building blocks of size > 1 - sequences where both parents have the same
        // jobs in same positions
        for (i, (j1, j2)) in p1.jobs.iter().zip(p2.jobs.iter()).enumerate() {
            if (j1 == j2)
                && ((i != 0 && p1.jobs[i - 1] == p2.jobs[i - 1])
                    || (i != c1.len() - 1 && p1.jobs[i + 1] == p2.jobs[i + 1]))
            {
                c1[i] = *j1;
                c2[i] = *j2;
            } else {
                // TODO: do this in another way for performance boost
                if !c2.contains(j1) {
                    p1_order.push(*j1);
                }
                if !c1.contains(j2) {
                    p2_order.push(*j2);
                }
            }
        }

        for i in 0..c1.len() {
            if c1[i] == u32::MAX {
                c1[i] = p2_order.remove(0);
                c2[i] = p1_order.remove(0);
            }
        }

        return (Chromosome::from(c1), Chromosome::from(c2));
    }
}

impl Crossover for BCBC {
    fn apply(
        p1: &Chromosome,
        p2: &Chromosome,
        k: Option<usize>,
        instance: &Instance,
    ) -> (Chromosome, Chromosome) {
        // Set number of jobs to extract
        let n_jobs = p1.jobs.len();

        let k = match k {
            Some(k) => k,
            None => n_jobs / 10,
        };

        // Get a random block from each parent with given block size
        let block1 = rand::thread_rng().gen_range(0..n_jobs - k);
        let block2 = rand::thread_rng().gen_range(0..n_jobs - k);

        let block1 = &p1.jobs[block1..block1 + k];
        let block2 = &p2.jobs[block2..block2 + k];

        // Remove the jobs from the opposite parent
        let filter = |jobs: Vec<u32>, block: &[u32]| -> Vec<u32> {
            jobs.into_iter().filter(|&j| !block.contains(&j)).collect()
        };
        let c1 = filter(p1.jobs.to_vec(), block2);
        let c2 = filter(p2.jobs.to_vec(), block1);

        // Test each possible insertion, record best index
        let c1 = find_best_insertion(c1, block2, instance);
        let c2 = find_best_insertion(c2, block1, instance);

        // TODO:
        // With probability x%, insert block into best position, otherwise random position

        // Return new chromosomes
        (Chromosome::from(c1), Chromosome::from(c2))
    }
}

pub fn find_best_insertion(jobs: Vec<u32>, block: &[u32], instance: &Instance) -> Vec<u32> {
    let n_jobs = jobs.len();
    let mut jobs: Vec<u32> = block.iter().cloned().chain(jobs.iter().cloned()).collect();
    let (mut makespan, _) = makespan::makespan(&jobs, instance);
    let k = block.len();

    // Store one random solution which may be returned
    let random_idx: usize = rand::thread_rng().gen_range(0..n_jobs);
    let mut random_jobs: Vec<u32> = jobs.iter().cloned().collect();

    // Initialise best jobs
    let mut best_jobs: Vec<Vec<u32>> = vec![jobs.iter().cloned().collect()];

    for i in 0..n_jobs {
        // Shift block one step to the right
        jobs[i..i + k + 1].rotate_right(1);
        let (new_makespan, _) = makespan::makespan(&jobs, instance);

        // Update random solution if we are at the random index
        if i == random_idx {
            random_jobs = jobs.iter().cloned().collect()
        }

        // Update best jobs if the current has a lower makespan
        if new_makespan < makespan {
            makespan = new_makespan;
            best_jobs = vec![jobs.iter().cloned().collect()];
        // Add current to best jobs if makespan is equal to best jobs
        } else if new_makespan == makespan {
            best_jobs.push(jobs.iter().cloned().collect());
        }
    }

    // Return either one of the best solutions, or the chosen random solution
    if rand::thread_rng().gen::<f32>() < params::KEEP_BEST {
        best_jobs.choose(&mut rand::thread_rng()).unwrap().to_vec()
    } else {
        random_jobs
    }
}

#[cfg(test)]
mod xover_test {
    use crate::common::instance::Instance;
    use crate::common::makespan;
    use crate::genetic_algorithm::operators::crossover::{self, Chromosome, Crossover};

    use super::find_best_insertion;

    #[test]
    fn test_block_insertion() {
        // jobs, block, instance

        let instance = test_instance();
        let jobs = test_chromosome(&instance);
        let block = &[2, 3];

        // Remove the jobs from the opposite parent
        let filter = |jobs: Vec<u32>, block: &[u32]| -> Vec<u32> {
            jobs.into_iter().filter(|&j| !block.contains(&j)).collect()
        };

        let jobs = filter(jobs.jobs.to_vec(), block);
        let jobs = find_best_insertion(jobs, block, &instance);

        // Possible permutations
        // [2, 3, 0, 1, 4]
        // [0, 2, 3, 1, 4]
        // [0, 1, 2, 3, 4]
        // [0, 1, 4, 2, 3]

        assert_eq!(jobs, &[0, 1, 2, 3, 4]);
        let (m, _) = makespan::makespan(&jobs, &instance);
        assert_eq!(m, 333);
    }

    #[test]
    fn crossover_sjox() {
        // let p1 = Chromosome::from(vec![4, 7, 9, 3, 5, 2, 6, 8, 1]);
        // let p2 = Chromosome::from(vec![9, 2, 4, 5, 7, 8, 6, 3, 1]);

        let p1 = Chromosome::from(vec![
            3, 15, 17, 8, 14, 11, 13, 16, 19, 6, 1, 9, 18, 5, 4, 2, 10, 7, 20, 12,
        ]);
        let p2 = Chromosome::from(vec![
            3, 17, 9, 15, 14, 11, 13, 16, 6, 18, 5, 19, 7, 8, 4, 2, 1, 10, 20, 12,
        ]);

        let (c1, c2) = crossover::SJOX::apply(&p1, &p2, Some(8), &test_instance());

        assert_eq!(
            c2.jobs,
            vec![3, 17, 9, 15, 14, 11, 13, 16, 8, 19, 6, 1, 18, 5, 4, 2, 10, 7, 20, 12]
        );
        assert_eq!(
            c1.jobs,
            vec![3, 15, 17, 8, 14, 11, 13, 16, 9, 6, 18, 5, 19, 7, 4, 2, 1, 10, 20, 12]
        );
    }

    fn test_instance() -> Instance {
        Instance {
            jobs: 5,
            stages: 2,
            machines: vec![2, 1],
            processing_times: vec![
                vec![71, 98],
                vec![51, 54],
                vec![0, 49],
                vec![94, 28],
                vec![29, 90],
            ],
            setup_times: vec![
                vec![
                    vec![1, 2, 3, 4, 5],
                    vec![1, 2, 3, 4, 5],
                    vec![1, 2, 3, 4, 5],
                    vec![1, 2, 3, 4, 5],
                    vec![5, 2, 3, 4, 5],
                ],
                vec![
                    vec![1, 2, 3, 4, 5],
                    vec![1, 2, 7, 4, 5],
                    vec![1, 2, 3, 4, 5],
                    vec![1, 2, 3, 4, 5],
                    vec![3, 2, 3, 4, 5],
                ],
            ],
        }
    }

    fn test_chromosome(instance: &Instance) -> Chromosome {
        let jobs: Vec<u32> = (0..instance.jobs).collect();

        Chromosome {
            jobs,
            makespan: None,
        }
    }
}
