use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};

use crate::genetic_algorithm::params;

use super::instance::Instance;

#[allow(dead_code)]
pub fn get_problem_files(run_all: bool) -> Vec<PathBuf> {
    match run_all {
        true => fs::read_dir("./instances/ruiz/json")
            .unwrap()
            .into_iter()
            .map(|p| p.unwrap().path())
            .collect(),
        false => vec![PathBuf::from(params::PROBLEM_FILE)],
    }
}

pub fn get_duration(instance: &Instance) -> u64 {
    let n_jobs = instance.jobs as f64;
    let m_stages = instance.stages as f64;

    let scale = 1.7;

    (n_jobs.powf(scale) * m_stages * 1.5 * 2.0) as u64
}

pub fn get_test_problems() -> Vec<PathBuf> {
    return vec![
        PathBuf::from("./instances/ruiz/json/n20m2-01.json"),
        PathBuf::from("./instances/ruiz/json/n20m2-02.json"),
        PathBuf::from("./instances/ruiz/json/n20m2-03.json"),
        PathBuf::from("./instances/ruiz/json/n20m2-04.json"),
        PathBuf::from("./instances/ruiz/json/n20m2-05.json"),
        PathBuf::from("./instances/ruiz/json/n20m2-06.json"),
        PathBuf::from("./instances/ruiz/json/n20m2-07.json"),
        PathBuf::from("./instances/ruiz/json/n20m2-08.json"),
        //
        PathBuf::from("./instances/ruiz/json/n20m4-01.json"),
        PathBuf::from("./instances/ruiz/json/n20m4-02.json"),
        PathBuf::from("./instances/ruiz/json/n20m4-03.json"),
        PathBuf::from("./instances/ruiz/json/n20m4-04.json"),
        PathBuf::from("./instances/ruiz/json/n20m4-05.json"),
        PathBuf::from("./instances/ruiz/json/n20m4-06.json"),
        PathBuf::from("./instances/ruiz/json/n20m4-07.json"),
        PathBuf::from("./instances/ruiz/json/n20m4-08.json"),
        //
        PathBuf::from("./instances/ruiz/json/n20m8-01.json"),
        PathBuf::from("./instances/ruiz/json/n20m8-02.json"),
        PathBuf::from("./instances/ruiz/json/n20m8-03.json"),
        PathBuf::from("./instances/ruiz/json/n20m8-04.json"),
        PathBuf::from("./instances/ruiz/json/n20m8-05.json"),
        PathBuf::from("./instances/ruiz/json/n20m8-06.json"),
        PathBuf::from("./instances/ruiz/json/n20m8-07.json"),
        PathBuf::from("./instances/ruiz/json/n20m8-08.json"),
        //
        PathBuf::from("./instances/ruiz/json/n50m2-01.json"),
        PathBuf::from("./instances/ruiz/json/n50m2-02.json"),
        PathBuf::from("./instances/ruiz/json/n50m2-03.json"),
        PathBuf::from("./instances/ruiz/json/n50m2-04.json"),
        PathBuf::from("./instances/ruiz/json/n50m2-05.json"),
        PathBuf::from("./instances/ruiz/json/n50m2-06.json"),
        PathBuf::from("./instances/ruiz/json/n50m2-07.json"),
        PathBuf::from("./instances/ruiz/json/n50m2-08.json"),
        //
        PathBuf::from("./instances/ruiz/json/n50m4-01.json"),
        PathBuf::from("./instances/ruiz/json/n50m4-02.json"),
        PathBuf::from("./instances/ruiz/json/n50m4-03.json"),
        PathBuf::from("./instances/ruiz/json/n50m4-04.json"),
        PathBuf::from("./instances/ruiz/json/n50m4-05.json"),
        PathBuf::from("./instances/ruiz/json/n50m4-06.json"),
        PathBuf::from("./instances/ruiz/json/n50m4-07.json"),
        PathBuf::from("./instances/ruiz/json/n50m4-08.json"),
        //
        PathBuf::from("./instances/ruiz/json/n50m8-01.json"),
        PathBuf::from("./instances/ruiz/json/n50m8-02.json"),
        PathBuf::from("./instances/ruiz/json/n50m8-03.json"),
        PathBuf::from("./instances/ruiz/json/n50m8-04.json"),
        PathBuf::from("./instances/ruiz/json/n50m8-05.json"),
        PathBuf::from("./instances/ruiz/json/n50m8-06.json"),
        PathBuf::from("./instances/ruiz/json/n50m8-07.json"),
        PathBuf::from("./instances/ruiz/json/n50m8-08.json"),
        //
        //
        PathBuf::from("./instances/ruiz/json/n80m2-01.json"),
        PathBuf::from("./instances/ruiz/json/n80m2-02.json"),
        PathBuf::from("./instances/ruiz/json/n80m2-03.json"),
        PathBuf::from("./instances/ruiz/json/n80m2-04.json"),
        PathBuf::from("./instances/ruiz/json/n80m2-05.json"),
        PathBuf::from("./instances/ruiz/json/n80m2-06.json"),
        PathBuf::from("./instances/ruiz/json/n80m2-07.json"),
        PathBuf::from("./instances/ruiz/json/n80m2-08.json"),
        //
        PathBuf::from("./instances/ruiz/json/n80m4-01.json"),
        PathBuf::from("./instances/ruiz/json/n80m4-02.json"),
        PathBuf::from("./instances/ruiz/json/n80m4-03.json"),
        PathBuf::from("./instances/ruiz/json/n80m4-04.json"),
        PathBuf::from("./instances/ruiz/json/n80m4-41.json"),
        PathBuf::from("./instances/ruiz/json/n80m4-42.json"),
        PathBuf::from("./instances/ruiz/json/n80m4-43.json"),
        PathBuf::from("./instances/ruiz/json/n80m4-44.json"),
        //
        PathBuf::from("./instances/ruiz/json/n80m8-01.json"),
        PathBuf::from("./instances/ruiz/json/n80m8-02.json"),
        PathBuf::from("./instances/ruiz/json/n80m8-21.json"),
        PathBuf::from("./instances/ruiz/json/n80m8-22.json"),
        PathBuf::from("./instances/ruiz/json/n80m8-41.json"),
        PathBuf::from("./instances/ruiz/json/n80m8-42.json"),
        PathBuf::from("./instances/ruiz/json/n80m8-61.json"),
        PathBuf::from("./instances/ruiz/json/n80m8-62.json"),
        //
        //
        PathBuf::from("./instances/ruiz/json/n120m2-01.json"),
        PathBuf::from("./instances/ruiz/json/n120m2-02.json"),
        PathBuf::from("./instances/ruiz/json/n120m2-03.json"),
        PathBuf::from("./instances/ruiz/json/n120m2-04.json"),
        PathBuf::from("./instances/ruiz/json/n120m2-41.json"),
        PathBuf::from("./instances/ruiz/json/n120m2-42.json"),
        PathBuf::from("./instances/ruiz/json/n120m2-43.json"),
        PathBuf::from("./instances/ruiz/json/n120m2-44.json"),
        //
        PathBuf::from("./instances/ruiz/json/n120m4-01.json"),
        PathBuf::from("./instances/ruiz/json/n120m4-02.json"),
        PathBuf::from("./instances/ruiz/json/n120m4-21.json"),
        PathBuf::from("./instances/ruiz/json/n120m4-22.json"),
        PathBuf::from("./instances/ruiz/json/n120m4-41.json"),
        PathBuf::from("./instances/ruiz/json/n120m4-42.json"),
        PathBuf::from("./instances/ruiz/json/n120m4-61.json"),
        PathBuf::from("./instances/ruiz/json/n120m4-61.json"),
        //
        PathBuf::from("./instances/ruiz/json/n120m8-01.json"),
        PathBuf::from("./instances/ruiz/json/n120m8-11.json"),
        PathBuf::from("./instances/ruiz/json/n120m8-21.json"),
        PathBuf::from("./instances/ruiz/json/n120m8-31.json"),
        PathBuf::from("./instances/ruiz/json/n120m8-41.json"),
        PathBuf::from("./instances/ruiz/json/n120m8-51.json"),
        PathBuf::from("./instances/ruiz/json/n120m8-61.json"),
        PathBuf::from("./instances/ruiz/json/n120m8-71.json"),
        //
        //
    ];
}

pub fn create_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);

    pb.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({pos}/{len}, ETA {eta})",
    ));

    pb
}

pub fn write_results(folder_name: &str, records: &Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    match Path::new(folder_name).is_dir() {
        false => fs::create_dir_all(folder_name)?,
        _ => (),
    }

    let mut wtr = Writer::from_path(String::from(folder_name) + "/results.csv")?;

    for record in records {
        wtr.write_record(record)?;
    }

    wtr.flush()?;
    Ok(())
}

pub fn write_makespan_improvement(
    filename: PathBuf,
    records: &Vec<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let filename = PathBuf::from("./solutions/improvement").join(filename);

    let parent_folder = Path::new(&filename).parent().unwrap();

    match Path::new(parent_folder).is_dir() {
        false => fs::create_dir_all(parent_folder)?,
        _ => (),
    }

    let mut wtr = Writer::from_path(filename)?;

    for record in records {
        wtr.write_record(record)?;
    }

    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::common::instance::parse;

    use super::{get_duration, get_test_problems};

    #[test]
    fn iterative_improvement_insertion_test() {
        let i = parse("./instances/ruiz/json/n120m8-01.json").unwrap();
        let duration = get_duration(&i) as u64;

        assert!(duration > 40000);
        assert!(duration < 50000);
    }

    #[test]
    fn test_files() {
        let files = get_test_problems();
        let files_consumed = get_test_problems();

        files
            .into_iter()
            .enumerate()
            .for_each(|(i, problem_file)| match parse(problem_file) {
                Err(_) => eprintln!("Could not open {:?}", files_consumed.get(i)),
                Ok(_) => (),
            })
    }
}
