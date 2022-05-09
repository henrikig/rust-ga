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

pub fn get_test_problems() -> Vec<PathBuf> {
    return vec![
        PathBuf::from("n20m2-01.json"),
        PathBuf::from("n20m2-02.json"),
        PathBuf::from("n20m2-03.json"),
        PathBuf::from("n20m2-04.json"),
        PathBuf::from("n20m2-05.json"),
        //
        PathBuf::from("n20m4-01.json"),
        PathBuf::from("n20m4-02.json"),
        PathBuf::from("n20m4-03.json"),
        PathBuf::from("n20m4-04.json"),
        PathBuf::from("n20m4-05.json"),
        //
        PathBuf::from("n50m2-01.json"),
        PathBuf::from("n50m2-02.json"),
        PathBuf::from("n50m2-03.json"),
        PathBuf::from("n50m2-04.json"),
        PathBuf::from("n50m2-05.json"),
        //
        PathBuf::from("n50m4-01.json"),
        PathBuf::from("n50m4-02.json"),
        PathBuf::from("n50m4-03.json"),
        PathBuf::from("n50m4-04.json"),
        PathBuf::from("n50m4-05.json"),
        //
        PathBuf::from("n50m8-01.json"),
        PathBuf::from("n50m8-02.json"),
        PathBuf::from("n50m8-03.json"),
        PathBuf::from("n50m8-04.json"),
        PathBuf::from("n50m8-05.json"),
        //
        PathBuf::from("n80m2-01.json"),
        PathBuf::from("n80m2-02.json"),
        PathBuf::from("n80m2-03.json"),
        PathBuf::from("n80m2-04.json"),
        PathBuf::from("n80m2-05.json"),
        //
        PathBuf::from("n80m4-01.json"),
        PathBuf::from("n80m4-02.json"),
        PathBuf::from("n80m4-03.json"),
        PathBuf::from("n80m4-04.json"),
        PathBuf::from("n80m4-05.json"),
        //
        PathBuf::from("n80m4-41.json"),
        PathBuf::from("n80m4-42.json"),
        PathBuf::from("n80m4-43.json"),
        PathBuf::from("n80m4-44.json"),
        PathBuf::from("n80m4-45.json"),
        //
        PathBuf::from("n80m8-01.json"),
        PathBuf::from("n80m8-02.json"),
        PathBuf::from("n80m8-03.json"),
        PathBuf::from("n80m8-04.json"),
        PathBuf::from("n80m8-05.json"),
        //
        PathBuf::from("n80m8-21.json"),
        PathBuf::from("n80m8-22.json"),
        PathBuf::from("n80m8-23.json"),
        PathBuf::from("n80m8-24.json"),
        PathBuf::from("n80m8-25.json"),
        //
        PathBuf::from("n80m8-41.json"),
        PathBuf::from("n80m8-42.json"),
        PathBuf::from("n80m8-43.json"),
        PathBuf::from("n80m8-44.json"),
        PathBuf::from("n80m8-45.json"),
        //
        PathBuf::from("n80m8-61.json"),
        PathBuf::from("n80m8-62.json"),
        PathBuf::from("n80m8-63.json"),
        PathBuf::from("n80m8-64.json"),
        PathBuf::from("n80m8-65.json"),
        //
        PathBuf::from("n120m2-01.json"),
        PathBuf::from("n120m2-02.json"),
        PathBuf::from("n120m2-03.json"),
        PathBuf::from("n120m2-04.json"),
        PathBuf::from("n120m2-05.json"),
        //
        PathBuf::from("n120m2-41.json"),
        PathBuf::from("n120m2-42.json"),
        PathBuf::from("n120m2-43.json"),
        PathBuf::from("n120m2-44.json"),
        PathBuf::from("n120m2-45.json"),
        //
        PathBuf::from("n120m4-01.json"),
        PathBuf::from("n120m4-02.json"),
        PathBuf::from("n120m4-03.json"),
        PathBuf::from("n120m4-04.json"),
        PathBuf::from("n120m4-05.json"),
        //
        PathBuf::from("n120m4-21.json"),
        PathBuf::from("n120m4-22.json"),
        PathBuf::from("n120m4-23.json"),
        PathBuf::from("n120m4-24.json"),
        PathBuf::from("n120m4-25.json"),
        //
        PathBuf::from("n120m4-41.json"),
        PathBuf::from("n120m4-42.json"),
        PathBuf::from("n120m4-43.json"),
        PathBuf::from("n120m4-44.json"),
        PathBuf::from("n120m4-45.json"),
        //
        PathBuf::from("n120m4-61.json"),
        PathBuf::from("n120m4-62.json"),
        PathBuf::from("n120m4-63.json"),
        PathBuf::from("n120m4-64.json"),
        PathBuf::from("n120m4-65.json"),
        //
        PathBuf::from("n120m8-01.json"),
        PathBuf::from("n120m8-02.json"),
        PathBuf::from("n120m8-03.json"),
        PathBuf::from("n120m8-04.json"),
        PathBuf::from("n120m8-05.json"),
        //
        PathBuf::from("n120m8-11.json"),
        PathBuf::from("n120m8-12.json"),
        PathBuf::from("n120m8-13.json"),
        PathBuf::from("n120m8-14.json"),
        PathBuf::from("n120m8-15.json"),
        //
        //
        PathBuf::from("n120m8-21.json"),
        PathBuf::from("n120m8-22.json"),
        PathBuf::from("n120m8-23.json"),
        PathBuf::from("n120m8-24.json"),
        PathBuf::from("n120m8-25.json"),
        //
        PathBuf::from("n120m8-31.json"),
        PathBuf::from("n120m8-32.json"),
        PathBuf::from("n120m8-33.json"),
        PathBuf::from("n120m8-34.json"),
        PathBuf::from("n120m8-35.json"),
        //
        PathBuf::from("n120m8-41.json"),
        PathBuf::from("n120m8-42.json"),
        PathBuf::from("n120m8-43.json"),
        PathBuf::from("n120m8-44.json"),
        PathBuf::from("n120m8-45.json"),
        //
        PathBuf::from("n120m8-51.json"),
        PathBuf::from("n120m8-52.json"),
        PathBuf::from("n120m8-53.json"),
        PathBuf::from("n120m8-54.json"),
        PathBuf::from("n120m8-55.json"),
        //
        PathBuf::from("n120m8-61.json"),
        PathBuf::from("n120m8-62.json"),
        PathBuf::from("n120m8-63.json"),
        PathBuf::from("n120m8-64.json"),
        PathBuf::from("n120m8-65.json"),
        //
        PathBuf::from("n120m8-71.json"),
        PathBuf::from("n120m8-72.json"),
        PathBuf::from("n120m8-73.json"),
        PathBuf::from("n120m8-74.json"),
        PathBuf::from("n120m8-75.json"),
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

pub fn write_makespan_improvement(records: &Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(params::IMPROVEMENT_FILE)?;

    for record in records {
        wtr.write_record(record)?;
    }

    wtr.flush()?;
    Ok(())
}
