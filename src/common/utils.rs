use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};

use crate::genetic_algorithm::params;

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

pub fn write_makespan_improvement(records: &Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(params::IMPROVEMENT_FILE)?;

    for record in records {
        wtr.write_record(record)?;
    }

    wtr.flush()?;
    Ok(())
}
