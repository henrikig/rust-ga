use rand::{
    prelude::{SliceRandom, StdRng},
    Rng,
};
use serde_derive::Serialize;

use crate::genetic_algorithm::entities::chromosome::Chromosome;

#[allow(dead_code)]
#[derive(Clone, Serialize, Copy)]
pub enum DTYPE {
    ExactMatch,
    DeviationDistance,
}

pub trait Distance {
    fn distance(c1: &Chromosome, c2: &Chromosome) -> i32;
}

pub struct ExactMatch;
pub struct DeviationDistance;

impl Distance for ExactMatch {
    fn distance(c1: &Chromosome, c2: &Chromosome) -> i32 {
        c1.jobs
            .iter()
            .zip(c2.jobs.iter())
            .filter(|(j1, j2)| j1 == j2)
            .count() as i32
    }
}

impl Distance for DeviationDistance {
    fn distance(c1: &Chromosome, c2: &Chromosome) -> i32 {
        let mut c1_idx = vec![0; c1.jobs.len()];
        let mut c2_idx = vec![0; c1.jobs.len()];

        // Get two vectors of index position for each job
        c1.jobs
            .iter()
            .zip(c2.jobs.iter())
            .enumerate()
            .for_each(|(i, (j1, j2))| {
                c1_idx[*j1 as usize] = i;
                c2_idx[*j2 as usize] = i;
            });

        // Calculate sum of absolute value of differences for vectors
        c1_idx.iter().zip(c2_idx.iter()).fold(0, |sum, (i1, i2)| {
            sum + (*i1 as f64 - *i2 as f64).abs() as i32
        })
    }
}

pub fn survivor_selection(
    children: &[Chromosome],
    parents: &[Chromosome],
    scale: f64,
    dtype: DTYPE,
    rng: &mut StdRng,
) -> [Chromosome; 2] {
    /*
    If d(p1, c1) + d(p2, c2) < d(p1, c2) + d(p2, c1)
        p1 <- winner of competition between p1 and c1,
        p2 <- winner of competition between p2 and c2.
    else
        p1 <- winner of competition between p1 and c2,
        p2 <- winner of competition between p2 and c1,
    */

    let p1;
    let p2;

    let d = match dtype {
        DTYPE::ExactMatch => |c1: &Chromosome, c2: &Chromosome| ExactMatch::distance(c1, c2),
        DTYPE::DeviationDistance => {
            |c1: &Chromosome, c2: &Chromosome| DeviationDistance::distance(c1, c2)
        }
    };

    if d(&parents[0], &children[0]) + d(&parents[1], &children[1])
        < d(&parents[0], &children[1]) + d(&parents[1], &children[0])
    {
        p1 = find_winner(&parents[0], &children[0], scale, rng);
        p2 = find_winner(&parents[1], &children[1], scale, rng);
    } else {
        p1 = find_winner(&parents[0], &children[1], scale, rng);
        p2 = find_winner(&parents[1], &children[0], scale, rng);
    };

    [p1, p2]
}

fn find_winner(
    parent: &Chromosome,
    child: &Chromosome,
    scale: f64,
    rng: &mut StdRng,
) -> Chromosome {
    let scale_is_zero = (0.0 / scale).is_nan();

    // A scale of 0 implies deterministic crowding
    if scale_is_zero {
        if parent.makespan.unwrap() < child.makespan.unwrap() {
            // Parent is fitter
            return parent.clone();
        } else if child.makespan.unwrap() < parent.makespan.unwrap() {
            // Child is fitter
            return child.clone();
        } else {
            // fitness values equal, choose uniformly
            match vec![true, false].choose(rng).unwrap() {
                true => return child.clone(),
                false => return parent.clone(),
            }
        }
    }

    // If scale > 0, we get a probabilistic case, where the probability depends on the scale
    let e = 1.0_f64.exp();
    let logistic_f = |x: f64| 1f64 / (1f64 + e.powf(-x));
    let scale_f = |diff| scale.powf(logistic_f(diff));

    // Find inverse of fitness to account for minimization of makespan
    let c_fitness = 1f64 / child.makespan.unwrap() as f64;
    let p_fitness = 1f64 / parent.makespan.unwrap() as f64;

    let prob_child = (scale_f(p_fitness - c_fitness) * c_fitness)
        / (scale_f(p_fitness - c_fitness) * c_fitness + scale_f(c_fitness - p_fitness) * p_fitness);

    if rng.gen::<f64>() < prob_child {
        child.clone()
    } else {
        parent.clone()
    }
}

// Returns index of parent to replace if child wins replacement competition, else None
pub fn k_nearest_replacement(
    c: &Chromosome,
    pop: &[Chromosome],
    k: usize,
    scale: f64,
    dtype: DTYPE,
    rng: &mut StdRng,
) -> Option<usize> {
    let dist = match dtype {
        DTYPE::ExactMatch => |c1: &Chromosome, c2: &Chromosome| ExactMatch::distance(c1, c2),
        DTYPE::DeviationDistance => {
            |c1: &Chromosome, c2: &Chromosome| DeviationDistance::distance(c1, c2)
        }
    };

    // Calculate distance from c to every individual in pop
    //[(0, 17), (1, 14), ..., (N, 12)]
    let mut distances = pop
        .iter()
        .map(|o| dist(c, o))
        .enumerate()
        .collect::<Vec<(usize, i32)>>();

    distances.sort_by(|(_, d1), (_, d2)| d1.cmp(d2));

    // Find k closest individuals
    let k_nearest = &distances[0..k];

    // Find least fit of the k closest individuals
    let least_fit = k_nearest
        .iter()
        .max_by_key(|(idx, _)| pop.get(*idx).unwrap().makespan.unwrap())
        .unwrap()
        .0;

    // Arrange tournament between c and individual
    let winner = find_winner(&c, pop.get(least_fit).unwrap(), scale, rng);

    // If c wins, return index of least fit individual, else None
    if &winner == c {
        Some(least_fit)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use rand::{prelude::StdRng, SeedableRng};

    use crate::genetic_algorithm::{
        entities::chromosome::Chromosome,
        operators::crowding::{
            k_nearest_replacement, DeviationDistance, Distance, ExactMatch, DTYPE,
        },
    };

    use super::survivor_selection;

    #[test]
    fn exact_distance() {
        let c1 = Chromosome::from(vec![0, 1, 2, 3, 4, 5]);
        let c2 = Chromosome::from(vec![0, 1, 2, 4, 3, 5]);

        assert_eq!(ExactMatch::distance(&c1, &c2), 4);
    }

    #[test]
    fn deviation_distance() {
        let c1 = Chromosome::from(vec![5, 1, 2, 3, 4, 0]);
        let c2 = Chromosome::from(vec![0, 1, 2, 4, 3, 5]);

        assert_eq!(DeviationDistance::distance(&c1, &c2), 12);
    }

    #[test]
    fn match_p1_c1() {
        let mut p1 = Chromosome::from(vec![5, 4, 3, 2, 1, 0]);
        let mut p2 = Chromosome::from(vec![0, 1, 2, 3, 4, 5]);

        let mut c1 = Chromosome::from(vec![5, 4, 3, 1, 2, 0]);
        let mut c2 = Chromosome::from(vec![1, 0, 2, 3, 4, 5]);

        p1.makespan = Some(10);
        p2.makespan = Some(20);

        c1.makespan = Some(15);
        c2.makespan = Some(5);

        let mut rng = StdRng::seed_from_u64(123);

        // Scale = 0 should imply deterministic crowding
        let res = survivor_selection(
            &mut [c1, c2],
            &[p1, p2],
            0.0_f64,
            DTYPE::DeviationDistance,
            &mut rng,
        );

        let mut p1 = Chromosome::from(vec![5, 4, 3, 2, 1, 0]);
        let mut c2 = Chromosome::from(vec![1, 0, 2, 3, 4, 5]);

        p1.makespan = Some(10);
        c2.makespan = Some(5);

        assert_eq!(res, [p1, c2]);
    }

    #[test]
    fn match_p1_c2() {
        let mut p1 = Chromosome::from(vec![5, 4, 3, 2, 1, 0]);
        let mut p2 = Chromosome::from(vec![0, 1, 2, 3, 4, 5]);

        let mut c1 = Chromosome::from(vec![1, 0, 3, 2, 4, 5]);
        let mut c2 = Chromosome::from(vec![4, 5, 3, 1, 2, 0]);

        p1.makespan = Some(10);
        p2.makespan = Some(20);

        c1.makespan = Some(15);
        c2.makespan = Some(5);

        let mut rng = StdRng::seed_from_u64(123);

        // Scale = 0 should imply deterministic crowding
        let res = survivor_selection(
            &mut [c1, c2],
            &[p1, p2],
            0.0_f64,
            DTYPE::DeviationDistance,
            &mut rng,
        );

        let mut c1 = Chromosome::from(vec![1, 0, 3, 2, 4, 5]);
        let mut c2 = Chromosome::from(vec![4, 5, 3, 1, 2, 0]);

        c1.makespan = Some(15);
        c2.makespan = Some(5);

        assert_eq!(res, [c2, c1]);
    }

    #[test]
    fn knn_replacement_k1() {
        let mut p1 = Chromosome::from(vec![0, 1, 2, 3, 4, 5]);
        let mut p2 = Chromosome::from(vec![0, 1, 3, 2, 4, 5]);
        let mut p3 = Chromosome::from(vec![5, 4, 3, 2, 1, 0]);
        let mut c = Chromosome::from(vec![4, 5, 3, 2, 1, 0]);

        p1.makespan = Some(10);
        p2.makespan = Some(12);
        p3.makespan = Some(20);
        c.makespan = Some(15);

        let pop = vec![p1, p2, p3];

        let mut rng = StdRng::seed_from_u64(123);

        // Scale = 0 should imply deterministic crowding
        let replacement_idx =
            k_nearest_replacement(&c, &pop, 1, 0.0_f64, DTYPE::DeviationDistance, &mut rng);

        assert_eq!(replacement_idx, Some(2));
    }

    #[test]
    fn knn_replacement_k2() {
        let mut p1 = Chromosome::from(vec![0, 1, 2, 3, 4, 5]);
        let mut p2 = Chromosome::from(vec![0, 1, 3, 2, 4, 5]);
        let mut p3 = Chromosome::from(vec![5, 4, 3, 2, 1, 0]);
        let mut c = Chromosome::from(vec![4, 5, 3, 2, 1, 0]);

        p1.makespan = Some(20);
        p2.makespan = Some(20);
        p3.makespan = Some(10);
        c.makespan = Some(15);

        let pop = vec![p1, p2, p3];
        let mut rng = StdRng::seed_from_u64(123);

        // Scale = 0 should imply deterministic crowding
        let replacement_idx =
            k_nearest_replacement(&c, &pop, 2, 0.0_f64, DTYPE::DeviationDistance, &mut rng);

        assert_eq!(replacement_idx, Some(1));
    }

    #[test]
    fn knn_no_replacement() {
        let mut p1 = Chromosome::from(vec![0, 1, 2, 3, 4, 5]);
        let mut p2 = Chromosome::from(vec![0, 1, 3, 2, 4, 5]);
        let mut p3 = Chromosome::from(vec![5, 4, 3, 2, 1, 0]);
        let mut c = Chromosome::from(vec![4, 5, 3, 2, 1, 0]);

        p1.makespan = Some(20);
        p2.makespan = Some(12);
        p3.makespan = Some(10);
        c.makespan = Some(15);

        let pop = vec![p1, p2, p3];

        let mut rng = StdRng::seed_from_u64(123);

        // Scale = 0 should imply deterministic crowding
        let replacement_idx =
            k_nearest_replacement(&c, &pop, 2, 0.0_f64, DTYPE::DeviationDistance, &mut rng);

        assert_eq!(replacement_idx, None);
    }
}
