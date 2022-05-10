use crate::{common::makespan::Makespan, genetic_algorithm::entities::chromosome::Chromosome};

use crate::common::best_insertion::find_best_insertion;

use rand::Rng;
use serde_derive::Serialize;

#[allow(dead_code)]
#[derive(Clone, Serialize)]
pub enum XTYPE {
    SJ2OX,
    SB2OX,
    BCBX,
    PMX,
}

pub trait Crossover {
    fn apply(
        p1: &Chromosome,
        p2: &Chromosome,
        k: Option<usize>,
        makespan: &mut Makespan,
    ) -> (Chromosome, Chromosome);
}

pub struct SJ2OX;
pub struct SB2OX;
pub struct BCBX;
pub struct PMX;

impl Crossover for SJ2OX {
    fn apply(
        p1: &Chromosome,
        p2: &Chromosome,
        k: Option<usize>,
        _makespan: &mut Makespan,
    ) -> (Chromosome, Chromosome) {
        // Generate new permutations based on parents
        let (mut c1, mut c2) = generate_children(p1, p2);

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

        // Copy over blocks of size > 1 from parents to children
        match_blocks(p1, p2, &mut c1, &mut c2, &mut p1_order, &mut p2_order);

        // Insert the jobs that are yet not allocated in order of opposite parent
        insert_remaining(k, &mut c1, p2_order, &mut c2, p1_order);

        return (Chromosome::from(c1), Chromosome::from(c2));
    }
}

impl Crossover for SB2OX {
    fn apply(
        p1: &Chromosome,
        p2: &Chromosome,
        _k: Option<usize>,
        _makespan: &mut Makespan,
    ) -> (Chromosome, Chromosome) {
        // Generate new permutations based on parents
        let (mut c1, mut c2) = generate_children(p1, p2);

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

        // Copy over blocks of size > 1 from parents to children
        match_blocks(p1, p2, &mut c1, &mut c2, &mut p1_order, &mut p2_order);

        // Insert the jobs that are yet not allocated in order of opposite parent
        insert_remaining(0, &mut c1, p2_order, &mut c2, p1_order);

        return (Chromosome::from(c1), Chromosome::from(c2));
    }
}

impl Crossover for BCBX {
    fn apply(
        p1: &Chromosome,
        p2: &Chromosome,
        k: Option<usize>,
        makespan: &mut Makespan,
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
        let (c1, m1) = find_best_insertion(c1, block2, makespan, true);
        let (c2, m2) = find_best_insertion(c2, block1, makespan, true);

        // Return new chromosomes
        (
            Chromosome::new_with_makespan(c1, m1),
            Chromosome::new_with_makespan(c2, m2),
        )
    }
}

impl Crossover for PMX {
    fn apply(
        p1: &Chromosome,
        p2: &Chromosome,
        _k: Option<usize>,
        _makespan: &mut Makespan,
    ) -> (Chromosome, Chromosome) {
        // The `pmx` function returns only one child, so we call it twice with
        // p1 and p2 swapping positions in the input
        let c1 = pmx(&p1.jobs, &p2.jobs);
        let c2 = pmx(&p2.jobs, &p1.jobs);

        (Chromosome::from(c1), Chromosome::from(c2))
    }
}

fn pmx(p1: &[u32], p2: &[u32]) -> Vec<u32> {
    let n_jobs = p1.len();
    let x1 = rand::thread_rng().gen_range(0..n_jobs - 1);
    let x2 = rand::thread_rng().gen_range(x1..n_jobs);

    let mut child = vec![0; n_jobs];

    let mut mapping: Vec<Option<usize>> = vec![None; n_jobs];

    for i in x1..x2 {
        child[i] = p2[i];

        mapping[p2[i] as usize] = Some(p1[i] as usize);
    }

    let mut map_from = |start: usize, stop: usize| {
        for i in start..stop {
            let mut o = mapping[p1[i] as usize];
            let mut last = None;

            while o.is_some() {
                last = o;
                o = mapping[o.unwrap()];
            }

            child[i] = if let Some(v) = last { v as u32 } else { p1[i] };
        }
    };

    map_from(0, x1);
    map_from(x2, n_jobs);

    child
}

fn generate_children(p1: &Chromosome, p2: &Chromosome) -> (Vec<u32>, Vec<u32>) {
    let c1: Vec<u32> = vec![u32::MAX; p1.jobs.len()];
    let c2: Vec<u32> = vec![u32::MAX; p2.jobs.len()];
    (c1, c2)
}

fn match_blocks(
    p1: &Chromosome,
    p2: &Chromosome,
    c1: &mut Vec<u32>,
    c2: &mut Vec<u32>,
    p1_order: &mut Vec<u32>,
    p2_order: &mut Vec<u32>,
) {
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
}

fn insert_remaining(
    k: usize,
    c1: &mut Vec<u32>,
    mut p2_order: Vec<u32>,
    c2: &mut Vec<u32>,
    mut p1_order: Vec<u32>,
) {
    for i in k..c1.len() {
        if c1[i] == u32::MAX {
            c1[i] = p2_order.remove(0);
            c2[i] = p1_order.remove(0);
        }
    }
}

#[cfg(test)]
mod xover_test {
    use itertools::Itertools;

    use crate::common::best_insertion::find_best_insertion;
    use crate::common::instance::Instance;
    use crate::common::makespan::Makespan;
    use crate::genetic_algorithm::operators::crossover::{self, Chromosome, Crossover};

    #[test]
    fn test_block_insertion() {
        // jobs, block, instance

        let instance = test_instance();
        let mut makespan = Makespan::new(&instance);
        let jobs = test_chromosome(&instance);
        let block = &[2, 3];

        // Remove the jobs from the opposite parent
        let filter = |jobs: Vec<u32>, block: &[u32]| -> Vec<u32> {
            jobs.into_iter().filter(|&j| !block.contains(&j)).collect()
        };

        let jobs = filter(jobs.jobs.to_vec(), block);
        let (jobs, _) = find_best_insertion(jobs, block, &mut makespan, true);

        // Possible permutations
        // [2, 3, 0, 1, 4]
        // [0, 2, 3, 1, 4]
        // [0, 1, 2, 3, 4]
        // [0, 1, 4, 2, 3]

        assert_eq!(jobs, &[2, 3, 0, 1, 4]);
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

        let (c1, c2) =
            crossover::SJ2OX::apply(&p1, &p2, Some(8), &mut Makespan::new(&test_instance()));

        assert_eq!(
            c2.jobs,
            vec![3, 17, 9, 15, 14, 11, 13, 16, 8, 19, 6, 1, 18, 5, 4, 2, 10, 7, 20, 12]
        );
        assert_eq!(
            c1.jobs,
            vec![3, 15, 17, 8, 14, 11, 13, 16, 9, 6, 18, 5, 19, 7, 4, 2, 1, 10, 20, 12]
        );
    }

    #[test]
    fn crossover_pmx() {
        let p1 = Chromosome::from(vec![9, 8, 4, 5, 6, 7, 1, 3, 2, 10]);
        let p2 = Chromosome::from(vec![8, 7, 1, 2, 3, 10, 9, 5, 4, 6]);

        let (c1, c2) = crossover::PMX::apply(&p1, &p2, None, &mut Makespan::new(&test_instance()));

        println!("{:?}", c1);
        println!("{:?}", c2);
    }

    #[test]
    fn crossover_pmx_overlap() {
        let p1 = Chromosome::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let p2 = Chromosome::from(vec![4, 3, 5, 8, 1, 0, 6, 7, 2]);

        let (c1, c2) = crossover::PMX::apply(&p1, &p2, None, &mut Makespan::new(&test_instance()));

        // Cut points: x1 = 2, x2 = 6
        // assert_eq(c1.jobs, vec![2, 4, 5, 8, 1, 0, 6, 7, 3]);
        // assert_eq!(c2.jobs, vec![1, 8, 2, 3, 4, 5, 6, 7, 0]);

        // Assert that there are no jobs that appear twice or more
        itertools::assert_equal(c1.jobs.iter().unique(), &c1.jobs);
        itertools::assert_equal(c2.jobs.iter().unique(), &c2.jobs);
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
            updated: true,
        }
    }
}
