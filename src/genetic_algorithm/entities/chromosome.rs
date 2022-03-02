use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::{Display, Error, Formatter};

use super::problem::Problem;

#[derive(Debug)]
pub struct Chromosome {
    jobs: Vec<u32>,
}

impl Chromosome {
    pub fn new(problem: &Problem) -> Chromosome {
        let mut jobs: Vec<u32> = (0..problem.n_jobs).collect();
        jobs.shuffle(&mut thread_rng());

        Chromosome { jobs }
    }

    pub fn makespan(&self, problem: &Problem) {
        /*
        1. for each stage, a vector with completion times is needed, e.g.:
        - [14, 8, 3, 19]
        - job 1: 14, job 2: 8, job 3: 3, job 4: 19

        2. for each machine, a vector with machine runs is needed, e.g.:
        - m11: [(3, 3), (2, 8), (1, 14), (4, 19)]
        - tuple: (job number, completion time)
        */
        let n = problem.n_jobs as usize;
        let m = problem.m_stages as usize;

        let mut job_completions = vec![vec![0; n]; m];
        let mut machine_completions: Vec<Vec<Vec<(u32, u32)>>> = Vec::with_capacity(m);

        /* [
            [
                [(3, 3), (1, 14)],
                [(2, 8), (4, 19)]
            ], [
                [(1, 2), (2, 8)],
                [(3, 9), (4, 18)]
            ]
        ] */
        for stage in 0..m {
            machine_completions.push(Vec::new());

            for _ in 0..problem.machines[stage] {
                machine_completions[stage].push(Vec::with_capacity(n));
            }
        }

        for (stage, m_machines) in problem.machines.iter().enumerate() {
            if stage == 0 {
                /*
                    In first stage, jobs are assigned according to job permutation
                    - [3, 2, 1, 4]
                    - job 3 -> job 2 -> job 1 -> job 4
                */
                for (i, job) in self.jobs.iter().enumerate() {
                    // Find best machine for current job
                    if i == 0 {
                        // No set-up time if this is the first job, assign to first machine
                        let processing_time = problem.processing_times[*job as usize][0];

                        // Add completion time to both machine and job completions vectors
                        machine_completions[stage][0].push((*job, processing_time));
                        job_completions[stage][i] = processing_time;
                    } else {
                        let mut earliest_completion: (u32, u32) = (u32::MAX, 0);
                        for machine in 0..*m_machines {
                            // Find which machine may process the job first based on the machine's ready time and setup time
                            // completion time is machine ready time + setup time + processing time
                            let completion_time =
                                match machine_completions[stage][machine as usize].iter().last() {
                                    // TODO: access setup times correctly
                                    Some((prev_job, ready_time)) => {
                                        problem.setup_times[stage][*prev_job as usize]
                                            + ready_time
                                            + problem.processing_times[*job as usize][0]
                                    }
                                    None => problem.processing_times[*job as usize][0],
                                };
                            // Compare with currently best completion time, and update if better
                            if completion_time < earliest_completion.0 {
                                earliest_completion = (completion_time, machine);
                            }
                        }
                        // Add completion time to both machine and job completions vectors
                        machine_completions[stage][earliest_completion.1 as usize]
                            .push((*job, earliest_completion.0));
                        job_completions[stage][i] = earliest_completion.0;
                    }
                }
            } else {
                /*
                    Stage > 0: jobs are now processed in order of ascending ready times
                    For each job in sorted order, find its completion time for each machine in current stage

                    ready times: [14, 8, 3, 19]
                    order: [3, 2, 1, 4]
                    order: job 3 -> job 2 -> job 1 -> job 4
                */
                let indices: Vec<usize> = (0..job_completions[stage - 1].len()).collect();

                let mut sorted = job_completions[stage - 1]
                    .iter()
                    .zip(indices)
                    .collect::<Vec<_>>();

                sorted.sort_by_key(|&(&val, _)| val);

                let ordered_jobs: Vec<usize> = sorted.iter().map(|&(_, idx)| idx).collect();

                for (i, &job) in ordered_jobs.iter().enumerate() {
                    // completion_time = max{machine ready time, job ready time} + setup time + processing time
                }
            }
        }
    }
}

impl Display for Chromosome {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut dash_separated = String::new();

        for num in &self.jobs[0..self.jobs.len() - 1] {
            dash_separated.push_str(&num.to_string());
            dash_separated.push_str("-");
        }

        dash_separated.push_str(&self.jobs[self.jobs.len() - 1].to_string());
        write!(f, "{}", dash_separated)
    }
}
