use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::{Display, Error, Formatter};

use super::problem::Problem;

#[derive(Debug)]
pub struct Chromosome {
    pub jobs: Vec<u32>,
    pub makespan: Option<u32>,
}

impl Chromosome {
    pub fn new(problem: &Problem) -> Chromosome {
        let mut jobs: Vec<u32> = (0..problem.n_jobs).collect();
        jobs.shuffle(&mut thread_rng());

        Chromosome {
            jobs,
            makespan: None,
        }
    }

    pub fn toy_chromosome(problem: &Problem) -> Chromosome {
        let jobs: Vec<u32> = (0..problem.n_jobs).collect();

        Chromosome {
            jobs,
            makespan: None,
        }
    }

    pub fn makespan(&mut self, problem: &Problem) {
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
                for job in self.jobs.iter() {
                    // If job has no processing time in this stage, skip directly to next stage
                    if problem.processing_times[*job as usize][stage] == 0 {
                        job_completions[stage][*job as usize] = 0;
                        continue;
                    }
                    // Find best machine for current job
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
                                        + problem.processing_times[*job as usize][stage]
                                }
                                None => problem.processing_times[*job as usize][stage],
                            };
                        // Compare with currently best completion time, and update if better
                        if completion_time < earliest_completion.0 {
                            earliest_completion = (completion_time, machine);
                        }
                    }
                    // Add completion time to both machine and job completions vectors
                    machine_completions[stage][earliest_completion.1 as usize]
                        .push((*job, earliest_completion.0));
                    job_completions[stage][*job as usize] = earliest_completion.0;
                }
            } else {
                /*
                    Stage > 0: jobs are now processed in order of ascending ready times
                    For each job in sorted order, find its completion time for each machine in current stage
                    If two jobs are ready at the same time, they should be processed in same order as of previous stage

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

                // Indices of jobs in sorted order: [3, 2, 1, 4]
                let ordered_jobs: Vec<usize> = sorted.iter().map(|&(_, idx)| idx).collect();

                for &job in ordered_jobs.iter() {
                    /*
                        Here, earliest completion time is found from both job and machine ready times
                        We find the maximum of the two above-mentioned, this is the earliest the job can start
                        In addition, sequence dependent setup time is needed based on a machine's prvious job, if any
                        Finally, the processing time must be accounted for
                        completion_time = max{machine ready time, job ready time} + setup time + processing time
                    */

                    // If job has no processing time in this stage, skip directly to next stage
                    // Completion time becomes completion time of previous stage
                    if problem.processing_times[job][stage] == 0 {
                        job_completions[stage][job] = job_completions[stage - 1][job];
                        continue;
                    }

                    let mut earliest_completion: (u32, u32) = (u32::MAX, 0);

                    for machine in 0..*m_machines {
                        // Find current machine's previous run, if any, and extract job type and ready time
                        let (prev_job, machine_ready_time) =
                            match machine_completions[stage][machine as usize].iter().last() {
                                Some((prev_job, ready_time)) => (*prev_job, *ready_time),
                                None => (u32::MAX, 0),
                            };

                        // Find ready time for job as maximum of the job and current machine's ready time
                        let ready_time =
                            std::cmp::max(job_completions[stage - 1][job], machine_ready_time);

                        let mut completion_time = ready_time + problem.processing_times[job][stage];

                        // Add setup time if this is not the first machine run of current machine
                        if prev_job != u32::MAX {
                            completion_time += problem.setup_times[stage][prev_job as usize];
                        }

                        if completion_time < earliest_completion.0 {
                            earliest_completion = (completion_time, machine);
                        }
                    }
                    // Add completion time to both machine and job completions vectors
                    machine_completions[stage][earliest_completion.1 as usize]
                        .push((job as u32, earliest_completion.0));
                    job_completions[stage][job] = earliest_completion.0;
                }
            }
        }

        // Find and return makespan as maximum of all completion times
        self.makespan = Some(*job_completions.iter().flatten().max().unwrap());
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

        match self.makespan {
            Some(m) => dash_separated.push_str(&format!(" (makespan: {})", m)[..]),
            _ => (),
        };

        write!(f, "{}", dash_separated)
    }
}
