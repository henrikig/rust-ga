use crate::common::{instance::Instance, makespan::Makespan};

// Implements the NEH construction heuristic by an instance of makespan
pub fn neh(makespan: &mut Makespan) -> Vec<u32> {
    // Order jobs in decending order of total processing time
    let job_order: Vec<u32> = sort_jobs(&makespan.instance);
    let mut schedule: Vec<u32> = Vec::with_capacity(makespan.instance.jobs as usize);
    for job in job_order.iter() {
        schedule = insert_job(makespan, &schedule, *job);
    }
    return schedule;
}

// Sort jobs in an instance in decending order of total processing times
fn sort_jobs(instance: &Instance) -> Vec<u32> {
    // Make a list of total production time: production_time[job]
    let mut processing_times: Vec<(u32, u32)> = Vec::with_capacity(instance.jobs as usize);
    // Adds processing times for all jobs from the instance to get the total processing times
    for job in 0..instance.jobs {
        processing_times.push((job, instance.processing_times[job as usize].iter().sum()));
    }
    // Sort jobs in decending order of total processing time
    processing_times.sort_by_key(|&(_job, processing_time)| processing_time);
    processing_times.reverse();
    // Return only the job numbers in decending order of total processing times
    let sorted: Vec<u32> = processing_times
        .iter()
        .map(|(job, _processing_time)| *job)
        .collect::<Vec<u32>>();
    return sorted;
}

// Returns the schedule with the lowest makespan after inserting the next job in all positions
fn insert_job(makespan: &mut Makespan, schedule: &Vec<u32>, next_job: u32) -> Vec<u32> {
    // Make tuple to keep track of shortest makespan and the corresponding schedule
    let mut min_time: (u32, Vec<u32>) = (u32::MAX, Vec::with_capacity(schedule.len() + 1));
    // Loop thorough all positions the next job can be inserted into
    for index in 0..schedule.len() + 1 {
        // Make an instance of the schedule to test
        let mut test_schedule: Vec<u32> = schedule.clone();
        test_schedule.insert(index, next_job);
        // Find the makespan of the test schedule
        let (time, _) = makespan.makespan(&test_schedule);
        // If the test schedule has a makespan lower than the current best, update the time and set the new schedule as the current best
        if min_time.0 > time {
            min_time = (time, test_schedule);
        }
    }
    // Return the schedule with the shortest makespan
    return min_time.1;
}

#[cfg(test)]
mod test {
    use crate::common::{instance::parse, instance::Instance, makespan::Makespan};
    use std::env;

    use super::{insert_job, neh, sort_jobs};

    #[test]
    fn sort_jobs_test() {
        let path = match env::consts::OS {
            "windows" => "instances\\ruiz\\json\\n20m2-1.json",
            "macos" => "./instances/ruiz/json/n20m2-1.json",
            _ => "./instances/ruiz/json/n20m2-1.json",
        };
        let i: Instance = parse(path).unwrap();
        let m: Makespan = Makespan {
            count: 1,
            instance: i,
        };
        let order = sort_jobs(&m.instance);
        println!("{:?}", order);
    }

    #[test]
    fn insert_job_test() {
        let path = match env::consts::OS {
            "windows" => "instances\\ruiz\\json\\n20m2-1.json",
            "macos" => "./instances/ruiz/json/n20m2-1.json",
            _ => "./instances/ruiz/json/n20m2-1.json",
        };
        let i: Instance = parse(path).unwrap();
        let mut m: Makespan = Makespan {
            count: 1,
            instance: i,
        };
        let order: Vec<u32> = sort_jobs(&m.instance);
        let schedule: Vec<u32> = order[0..4].to_vec();
        let _new_schedule = insert_job(&mut m, &schedule, order[5]);
    }

    #[test]
    fn neh_test_instance1() {
        let makespan = neh_from_file("./instances/ruiz/json/n20m2-1.json");
        assert_eq!(makespan, 675);
    }

    #[test]
    fn neh_test_instance2() {
        let makespan = neh_from_file("./instances/ruiz/json/n20m2-2.json");
        assert_eq!(makespan, 673);
    }

    #[test]
    fn neh_test_instance3() {
        let makespan = neh_from_file("./instances/ruiz/json/n20m2-3.json");
        assert_eq!(makespan, 590);
    }

    #[test]
    fn neh_test_instance4() {
        let makespan = neh_from_file("./instances/ruiz/json/n20m2-4.json");
        assert_eq!(makespan, 598);
    }

    #[test]
    fn schedule() {
        let i: Instance = parse("./instances/ruiz/json/n20m2-1.json").unwrap();
        let jobs = (0..i.jobs).collect();
        let mut m = Makespan::new(&i);

        let (make, mc) = m.makespan(&jobs);
        dbg!(make);
        for stage in mc {
            println!("{:?}", stage);
        }
    }

    fn neh_from_file(path: &str) -> u32 {
        let i: Instance = parse(path).unwrap();
        let mut m = Makespan::new(&i);

        let schedule = neh(&mut m);
        let (make, _) = m.makespan(&schedule);
        make
    }
}
