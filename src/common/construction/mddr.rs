use crate::{common::makespan::Makespan, iterated_greedy::options::Options};

use super::solver::Solver;

pub struct MDDR {}

impl Solver for MDDR {
    fn run(makespan: &mut Makespan, _: Option<Options>) -> u32 {
        let result = MDDR::mddr(makespan);

        result.0
    }
}

impl MDDR {
    pub fn mddr(makespan: &mut Makespan) -> (u32, Vec<Vec<Vec<(u32, u32)>>>) {
        let instance = &makespan.instance;

        let n_jobs = instance.jobs as usize;
        let n_stages = instance.stages as usize;

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

        // Previous stage completions initially set to zero
        let mut prev_stage_completions: Vec<(u32, u32)> = Vec::with_capacity(n_jobs as usize);

        for job in 0..n_jobs {
            prev_stage_completions.push((job as u32, 0));
        }

        for stage in 0..n_stages {
            for _ in 0..n_jobs {
                let stage = stage as u32;
                Makespan::earliest_completion_time(
                    &stage,
                    &mut prev_stage_completions,
                    &mut job_completions,
                    &mut machine_completions,
                    &makespan.instance,
                )
            }

            prev_stage_completions = job_completions[stage as usize].clone();
        }

        // Return makespan and machine completions
        (
            *job_completions
                .iter()
                .flatten()
                .map(|(_job, time)| time)
                .max()
                .unwrap(),
            machine_completions,
        )
    }
}

#[cfg(test)]
mod mddr_test {
    use crate::common::{instance::Instance, makespan::Makespan};

    use super::MDDR;

    #[test]
    fn test_mddr() {
        let instance = mini_instance();
        let mut makespan = Makespan::new(&instance);
        let (mks, machine_completions) = MDDR::mddr(&mut makespan);

        println!("{:?}", machine_completions);
        println!("{}", mks);
    }

    fn mini_instance() -> Instance {
        Instance {
            jobs: 3,
            stages: 2,
            machines: vec![1, 1],
            processing_times: vec![vec![1, 1], vec![20, 0], vec![10, 10]],
            setup_times: vec![
                vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
                vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
            ],
        }
    }
}
