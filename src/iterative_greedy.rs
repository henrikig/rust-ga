/*
Algorithm for iterative greedy on the permutation flowshop problem.
R. Ruiz, T. Stu€tzle / European Journal of Operational Research 177 (2007) 2033–2049

Iterative_Greedy(Instance) {
    current_schedule = NEH(instance);
    current_schedule = local_search(schedule); (ruiz used deletion of one job)
    best_schedule = current_schedule;
    d = some number between 0 and amount of jobs; (ruiz used 3)
    while termination criteria not met {
        schedule_permutation = current_schedule;    # Destruction phase
        deleted_jobs = empty list;
        for i in 0..d {
            remove a job from schedule_permutation at random and insert into deleted_jobs
        }
        for i in 0..d {                     # Construction phase
            schedule_permutation = insert deleted_jobs[i] in the best spot;
        }
        new_schedule = local_search(schedule_permutation);
        if new_schedule has lower makespan than schedule {
            current_schedule = new_schedule;
            if current_schedule has lower makespan than best_schedule{
                best_schedule = current_schedule
            }
        }
        elif random <= exp(-(makspan(new_shedule) - makespan(current_schedule))/temperature) {
            current_schedule = new_schedule
        }
    }
    return best_schedule
}
*/

use rand::Rng;

use crate::common::{
    construction::neh::{insert_job, neh},
    makespan::Makespan,
};

// All schedules are tuples of the schedule and makespan of the schedule

pub fn iterative_greedy(makespan: &mut Makespan) -> (Vec<u32>, u32) {
    let mut current_schedule: (Vec<u32>, u32) = neh(makespan);
    current_schedule = iterative_improvement_insertion(makespan, &current_schedule.0);
    let mut best_schedule: (Vec<u32>, u32) = (current_schedule.0.clone(), current_schedule.1);
    let t: f64 = 0.2; // Ruiz used 0, 0.1, 0.2, 0.3, 0.4, and 0.5
    let temp: f64 = find_temp(&makespan, t);
    let mut rng = rand::thread_rng();
    let d = 3;
    while makespan.count < 5000 {
        let mut schedule_permutation = current_schedule.clone();
        let mut deleted_jobs: Vec<u32> = Vec::with_capacity(makespan.instance.jobs as usize);
        for _ in 0..d {
            let (reduced_schedule, deleted_job) = remove_random(&schedule_permutation.0);
            let reduced_schedule_makspan = makespan.makespan(&reduced_schedule).0;
            schedule_permutation = (reduced_schedule, reduced_schedule_makspan).clone();
            deleted_jobs.push(deleted_job.clone())
        }
        for job in deleted_jobs.iter() {
            schedule_permutation = insert_job(makespan, &schedule_permutation.0, job);
        }
        let new_schedule: (Vec<u32>, u32) =
            iterative_improvement_insertion(makespan, &schedule_permutation.0);
        if current_schedule.1 > new_schedule.1 {
            current_schedule = (new_schedule.0.clone(), new_schedule.1);
            if best_schedule.1 > current_schedule.1 {
                best_schedule = (current_schedule.0.clone(), current_schedule.1);
            }
        } else if rng.gen::<f64>()
            <= (-(current_schedule.1 as f64 - schedule_permutation.1 as f64) / temp).exp()
        {
            current_schedule = (schedule_permutation.0.clone(), schedule_permutation.1);
        }
        println!(
            "Makespan count: {}, current_schedule makespan: {}, best_schedule makespan: {}",
            makespan.count, current_schedule.1, best_schedule.1
        );
    }
    return best_schedule;
}

fn find_temp(makespan: &Makespan, t: f64) -> f64 {
    let mut total_production_time: f64 = 0.0;
    for jobs in makespan.instance.processing_times.iter() {
        for processing_time in jobs.iter() {
            total_production_time += *processing_time as f64;
        }
    }
    let denominator: f64 =
        makespan.instance.jobs as f64 * makespan.instance.stages as f64 * 10 as f64;
    return t * total_production_time * denominator;
}

// Local search removing one job and finding a better place for it. Runs until solution is not improving
pub fn iterative_improvement_insertion(
    makespan: &mut Makespan,
    schedule: &Vec<u32>,
) -> (Vec<u32>, u32) {
    let mut improvement = true;
    // n is the amout of tries before each check of improvement. Should not be larger than the amount of jobs
    let n = schedule.len() / 5;
    // current_schedule keeps track of the best schedule and the makespan of it
    let mut current_schedule: (Vec<u32>, u32) = (schedule.clone(), makespan.makespan(schedule).0);
    // Perform the local search as long as it is improving
    while improvement == true {
        improvement = false;
        // jobs_removed keeps track of the jobs that has been reallocated in the for-loop, so they are not repeated
        let mut jobs_removed = Vec::with_capacity(n);
        for _ in 0..n {
            // destroyed keeps track of the schedule after removing a job and the number of the removed job
            let mut destroyed: (Vec<u32>, u32) = (Vec::with_capacity(schedule.len()), u32::MAX);
            // remove_random() has to be run until the job it removes is unique
            while destroyed.0.is_empty() || jobs_removed.contains(&destroyed.1) {
                destroyed = remove_random(&current_schedule.0);
            }
            // Add the chosen job to jobs_removed
            jobs_removed.push(destroyed.1);
            // Insert the removed job in the position that gives the lowest makespan
            let new_schedule = insert_job(makespan, &destroyed.0, &destroyed.1);
            // If the new_schedule is better than the current_schedule, update the current best to the new.
            if new_schedule.1 < current_schedule.1 {
                current_schedule = new_schedule
            }
        }
    }
    // return the best schedule found
    return current_schedule;
}

// Removes a random job from a schedule and returns the new schedule and the job
fn remove_random(schedule: &Vec<u32>) -> (Vec<u32>, u32) {
    // Clone schedule to make sure it operates in steady state
    let mut new_schedule = schedule.clone();
    // let index: u32 = rand::random::<u32>() % schedule.len();
    // Choose a random job from the schedule
    let job_index = rand::thread_rng().gen_range(0..schedule.len());
    let job = schedule[job_index];
    // Remove the job from the schedule
    new_schedule.remove(job_index as usize);
    // Return both
    return (new_schedule, job);
}

#[cfg(test)]
mod ig_tests {

    use super::*;
    use crate::common::{instance::parse, instance::Instance, makespan::Makespan};

    #[test]
    fn iterative_improvement_insertion_test() {
        let i: Instance = parse("instances\\ruiz\\json\\n20m2-1.json").unwrap();
        let mut m: Makespan = Makespan::new(&i);
        let schedule: Vec<u32> = (0..20).collect();

        let (local_search, _) = iterative_improvement_insertion(&mut m, &schedule);
        println!(
            "Initial schedule makspan: {}, after iii: {}, makespan calculations: {}",
            m.makespan(&schedule).0,
            m.makespan(&local_search).0,
            m.count
        );
        assert!(m.makespan(&schedule) >= m.makespan(&local_search));
    }

    #[test]
    fn iterative_greedy_test() {
        let i: Instance = parse("instances\\ruiz\\json\\n20m2-1.json").unwrap();
        let mut m: Makespan = Makespan::new(&i);

        let ig = iterative_greedy(&mut m);

        let schedule: Vec<u32> = (0..20).collect();
        let schedule_makespan = m.makespan(&schedule).0;

        println!(
            "Makespan of arbitraty schedule: {}, makespan of ig: {}, makespan calculations: {}",
            schedule_makespan, ig.1, m.count
        )
    }
}
