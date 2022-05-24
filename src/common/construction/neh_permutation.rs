use crate::{
    common::{instance::Instance, makespan::Makespan},
    iterated_greedy::options::Options,
};

use super::{neh, solver::Solver};

pub struct NehP {}

impl NehP {
    pub fn neh_p(makespan: &mut Makespan) -> (Vec<u32>, u32) {
        let instance: &Instance = &makespan.instance;
        let job_order: Vec<u32> = neh::sort_jobs(instance);
        // Order jobs in decending order of total processing time
        let n_jobs: usize = job_order.len();
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
            prev_stage_completions.push((job_order[job], 0));
        }

        // Iterate permutation, find best machine for jobs in the initial order and assign it
        for stage in 0..n_stages {
            for job in job_order.iter() {
                let prev_stage_completion_time = prev_stage_completions
                    .iter()
                    .filter(|(j, _)| job == j)
                    .next()
                    .unwrap()
                    .1;
                let time_machine = Makespan::choose_machine_for_job(
                    job,
                    &(stage as u32),
                    &prev_stage_completion_time,
                    &machine_completions,
                    instance,
                );

                // update job and machine completions
                job_completions[stage].push((*job, time_machine.0));
                machine_completions[stage][time_machine.1 as usize].push((*job, time_machine.0));
            }
            prev_stage_completions = job_completions[stage].clone();
        }

        println!("{:?}", machine_completions);

        (
            job_order,
            *job_completions
                .iter()
                .flatten()
                .map(|(_j, t)| t)
                .max()
                .unwrap(),
        )
    }
}

impl Solver for NehP {
    fn run(makespan: &mut Makespan, _: Option<Options>) -> u32 {
        let res = NehP::neh_p(makespan);
        res.1
    }
}

#[cfg(test)]
mod test {
    use crate::common::{construction::solver::Solver, instance::parse, makespan::Makespan};

    use super::NehP;

    #[test]

    fn filter_jobs() {
        let order = vec![(1, 3), (2, 8), (3, 12)];

        let job_3_completion = order.iter().filter(|(j, _)| &3 == j).next().unwrap().1;

        assert_eq!(job_3_completion, 12);
    }

    #[test]
    fn test_neh_permutation() {
        let instance = parse("./instances/ruiz/json/n20m2-01.json").unwrap();

        let mut makespan = Makespan::new(&instance);

        let (_, mks) = NehP::neh_p(&mut makespan);

        println!("Makespan: {}", mks);
    }

    #[test]
    fn run_all() {
        NehP::run_all("./solutions/neh_permutation");
    }
}
