use crate::common::makespan::Makespan;
use crate::genetic_algorithm::entities::chromosome::Chromosome;
use crate::iterated_greedy::iterated_greedy;

// Iterated greedy
pub fn ls_ig(chromosome: &mut Chromosome, makespan: &mut Makespan, approx_calc: u32) {
    let original_schedule: Option<(Vec<u32>, u32)> = Some((
        chromosome.jobs.clone(),
        makespan.makespan(&chromosome.jobs).0,
    ));
    let new_schedule: (Vec<u32>, u32) = iterated_greedy(makespan, original_schedule, approx_calc);
    chromosome.jobs = new_schedule.0
}
