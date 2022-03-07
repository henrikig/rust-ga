use crate::genetic_algorithm::entities::chromosome::Chromosome;

use rand::Rng;

pub trait Crossover {
    fn apply(p1: &Chromosome, p2: &Chromosome, k: Option<usize>) -> (Chromosome, Chromosome);
}

pub struct SJOX;

impl Crossover for SJOX {
    fn apply(p1: &Chromosome, p2: &Chromosome, k: Option<usize>) -> (Chromosome, Chromosome) {
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

        for i in k..c1.len() {
            if c1[i] == u32::MAX {
                c1[i] = p2_order.remove(0);
                c2[i] = p1_order.remove(0);
            }
        }

        return (Chromosome::from(c1), Chromosome::from(c2));
    }
}
