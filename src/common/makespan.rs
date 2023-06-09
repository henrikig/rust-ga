use core::cmp::max;

use super::instance::Instance;

const PROCESS_FIFO: bool = true;
const PROCESS_FIRSTCOMPLETE: bool = false;

pub struct Makespan {
    pub count: u32,
    pub instance: Instance,
}

/* Pseudo code
makespan(initial_job_order, instance) -> makespan
    let job_completions[stage](job_number, job_completion_time)
    let machine_completions[stage][machine][machine_run](job_number, completion_time)
    let mut job_order = inital_job_order
    for stage in instance.stages:
        for job in job_order:
            if PROCESS_FIFO or stage == 0 {
                find the machine that processes the job the fastest
                Schedule the job for that machine
            }
            elif PROCESS_FIRSTCOMPLETE {
                find the next job that can finish the fastest, and the corresponding machine
                schedule that job on the corresponding machine machine
            }
        let job_order = The completion order of products in the current stage
    return the largest completion time from the last stage
*/
impl Makespan {
    pub fn new(instance: &Instance) -> Makespan {
        Makespan {
            count: 0,
            instance: instance.clone(),
        }
    }

    // returns the makespan of the schedule and the machine completions
    pub fn makespan(&mut self, initial_job_order: &Vec<u32>) -> (u32, Vec<Vec<Vec<(u32, u32)>>>) {
        let instance: &Instance = &self.instance;
        let n_jobs: usize = initial_job_order.len();
        let n_stages: usize = instance.stages as usize;

        // job_completions[stage][job](job_number, completion_time)
        let mut job_completions: Vec<Vec<(u32, u32)>> = Vec::with_capacity(n_stages);
        for _stage in 0..n_stages {
            job_completions.push(Vec::with_capacity(n_jobs));
        }
        // machine_completions[stage][machine][machine_run](job_number, completion_time)
        let mut machine_completions: Vec<Vec<Vec<(u32, u32)>>> = Vec::with_capacity(n_stages);
        for stage in 0..n_stages {
            machine_completions.push(Vec::with_capacity(instance.machines[stage] as usize));
            for _machine in 0..instance.machines[stage] {
                machine_completions[stage].push(Vec::with_capacity(n_jobs));
            }
        }
        // prev_stage_completions[job](job_number, completion_time)
        // Note: this is simply the job_completion for the previous stage, but it needs to be artificially made the first time
        let mut prev_stage_completions: Vec<(u32, u32)> = Vec::with_capacity(n_jobs);
        for job in 0..n_jobs {
            prev_stage_completions.push((initial_job_order[job], 0));
        }

        for stage in 0..instance.stages {
            if PROCESS_FIFO || stage == 0 {
                // If FIFO and not in stage 0, we need to sort prev_stage_completions according to ready times of each job
                if stage != 0 {
                    prev_stage_completions.sort_by_key(|&(_, time)| time);
                }

                for (job, prev_stage_completion_time) in prev_stage_completions.iter() {
                    Self::fifo(
                        &job,
                        &stage,
                        &prev_stage_completion_time,
                        &mut job_completions,
                        &mut machine_completions,
                        &instance,
                    )
                }
            } else if PROCESS_FIRSTCOMPLETE {
                for _ in 0..n_jobs {
                    Self::earliest_completion_time(
                        &stage,
                        &mut prev_stage_completions,
                        &mut job_completions,
                        &mut machine_completions,
                        &instance,
                    )
                }
            }
            prev_stage_completions = job_completions[stage as usize].clone();
        }
        self.count += 1;
        return (
            *job_completions[job_completions.len() - 1]
                .iter()
                .map(|(_job, time)| time)
                .max()
                .unwrap(),
            machine_completions,
        );
    }

    pub fn fifo(
        job: &u32,
        stage: &u32,
        prev_stage_completion_time: &u32,
        job_completions: &mut Vec<Vec<(u32, u32)>>,
        machine_completions: &mut Vec<Vec<Vec<(u32, u32)>>>,
        instance: &Instance,
    ) {
        let time;
        let machine;
        if instance.processing_times[*job as usize][*stage as usize] != 0 {
            // Find the machine that can finish the job the quickest
            (time, machine) = Self::choose_machine_for_job(
                &job,
                &stage,
                &prev_stage_completion_time,
                &machine_completions,
                &instance,
            );
        } else {
            (_, time) = *job_completions[*stage as usize]
                .iter()
                .last()
                .unwrap_or(&(0u32, *prev_stage_completion_time));
            machine = u32::MAX;
        }
        // Update datastructures to keep track of production schedule
        job_completions[*stage as usize].push((*job, time));
        if machine != u32::MAX {
            machine_completions[*stage as usize][machine as usize].push((*job, time));
        }
    }

    // Function for choosing the machine in a given stage that completes the job the quickest
    pub fn choose_machine_for_job(
        job: &u32,
        stage: &u32,
        prev_stage_completion_time: &u32,
        machine_completions: &Vec<Vec<Vec<(u32, u32)>>>,
        instance: &Instance,
    ) -> (u32, u32) {
        // Initiate the time to the maximum and machine as the zero-th machine
        let mut time_machine: (u32, u32) = (u32::MAX, 0);
        // Loop through all machines in the stage
        for machine in 0..(machine_completions[*stage as usize].len() as usize) {
            // Get the number of the last job to complete on the machine and the time the machine is ready for the next job
            let (prev_job, machine_ready_time): (u32, u32) =
                match machine_completions[*stage as usize][machine].iter().last() {
                    Some((prev_job, machine_ready_time)) => (*prev_job, *machine_ready_time),
                    // If the machine has not processed any jobs yet, return (job, 0) because setup_times[machine][job][job] gives the initial setup time
                    None => (*job, 0),
                };
            // Compute the completion time
            let completion_time = max(machine_ready_time, *prev_stage_completion_time)
                + instance.processing_times[*job as usize][*stage as usize]
                + match instance.processing_times[*job as usize][*stage as usize] {
                    0 => 0,
                    _ => instance.setup_times[*stage as usize][prev_job as usize][*job as usize],
                };
            // If the time of completion is less than the current best found, update the machine to use and the completion time
            if time_machine.0 > completion_time {
                time_machine = (completion_time, machine as u32)
            }
        }
        // Return the best time found, and which machine it uses
        return time_machine;
    }

    pub fn earliest_completion_time(
        stage: &u32,
        jobs_outstanding: &mut Vec<(u32, u32)>,
        job_completions: &mut Vec<Vec<(u32, u32)>>,
        machine_completions: &mut Vec<Vec<Vec<(u32, u32)>>>,
        instance: &Instance,
    ) {
        // Find the job that can finish the quickest and the corresponding machine and time of completion
        let (time, job, machine, start): (u32, u32, u32, u32) =
            Self::choose_fastest_job_and_machine(
                &stage,
                &jobs_outstanding,
                &machine_completions,
                &instance,
            );
        // Update datastuctures with the new scheduled job
        job_completions[*stage as usize].push((job, time));
        machine_completions[*stage as usize][machine as usize].push((job, time));
        // Find the index of the job from the vector of jobs outstanding
        let remove_at_index: usize = jobs_outstanding
            .iter()
            .position(|(j, t)| (j, t) == (&job, &start))
            .unwrap();
        // Remove the scheduled job from the list of jobs to be scheduled
        jobs_outstanding.remove(remove_at_index);
    }

    // Determines the job that can be completed the quickest and the corresponding machine, as well as the time it starts and finishes
    fn choose_fastest_job_and_machine(
        stage: &u32,
        jobs_outstanding: &Vec<(u32, u32)>,
        machine_completions: &Vec<Vec<Vec<(u32, u32)>>>,
        instance: &Instance,
    ) -> (u32, u32, u32, u32) {
        let mut time_job_machine_start = (u32::MAX, 0, 0, u32::MAX);
        // Loop through all jobs left to be scheduled
        for (job, prev_completion_time) in jobs_outstanding.iter() {
            // Find the machine that can finish the job first
            let current = Self::choose_machine_for_job(
                &job,
                &stage,
                &prev_completion_time,
                &machine_completions,
                &instance,
            );
            // If the time to complete the job is shorter than the current best, set it to the current best
            if time_job_machine_start.0 > current.0 {
                time_job_machine_start = (current.0, *job, current.1, *prev_completion_time);
            }
        }
        return time_job_machine_start;
    }
}

#[cfg(test)]
mod makespan_tests {
    use std::time::Instant;

    use itertools::Itertools;

    use crate::{common::instance::parse, genetic_algorithm::tests::tests::test_instance};

    use super::Makespan;

    #[test]
    fn makespan_calculation() {
        let instance = test_instance();
        let mut makespan = Makespan::new(&instance);

        let jobs1 = vec![0, 1, 2, 3, 4];
        let jobs2 = vec![1, 2, 3, 4, 0];
        let jobs3 = vec![2, 3, 4, 0, 1];
        let jobs4 = vec![3, 4, 0, 1, 2];
        let jobs5 = vec![4, 0, 1, 2, 3];

        let (m, _) = makespan.makespan(&jobs1);
        dbg!(m);
        //assert_eq!(m, 391);
        let (m, _) = makespan.makespan(&jobs2);
        dbg!(m);
        let (m, _) = makespan.makespan(&jobs3);
        dbg!(m);
        let (m, _) = makespan.makespan(&jobs4);
        dbg!(m);
        let (m, _) = makespan.makespan(&jobs5);
        dbg!(m);
    }

    #[test]
    fn makspan_test() {
        let instance = parse("./instances/ruiz/json/n20m2-01.json").unwrap();
        let mut makespan = Makespan::new(&instance);
        let schedule: Vec<u32> = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ];
        let (time, machine_completions) = makespan.makespan(&schedule);
        println!("Makespan: {}", time);
        for stage in 0..instance.stages {
            for machine in 0..instance.machines[stage as usize] {
                println!(
                    "Stage {}, machine {}: {:?}",
                    stage, machine, machine_completions[stage as usize][machine as usize]
                );
            }
        }
    }

    #[test]
    fn makspan_test_unique() {
        let instance = parse("./instances/ruiz/json/n120m8-01.json").unwrap();
        let mut makespan = Makespan::new(&instance);
        let schedule = (0..120).collect::<Vec<u32>>();
        let (_, machine_completions) = makespan.makespan(&schedule);
        for stage in 0..instance.stages {
            let jobs = machine_completions[stage as usize]
                .iter()
                .flatten()
                .map(|(j, _)| *j)
                .collect::<Vec<u32>>();

            let jobs_clone = jobs.clone();

            itertools::assert_equal(jobs.into_iter().unique(), jobs_clone);
        }
    }

    #[test]
    fn makespan_time() {
        let instance = parse("./instances/ruiz/json/n120m8-01.json").unwrap();
        let mut makespan = Makespan::new(&instance);
        let schedule = (0..120).collect::<Vec<u32>>();
        let start = Instant::now();
        let (mks, _) = makespan.makespan(&schedule);
        let elapsed = start.elapsed().as_millis();
        println!("Found makespan {} in {} ms", mks, elapsed);
    }
}
