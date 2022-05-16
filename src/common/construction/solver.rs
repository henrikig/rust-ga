use lexical_sort::natural_lexical_cmp;
use rayon::prelude::*;
use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

use crate::{
    common::{
        instance::{parser::parse, Instance},
        makespan::Makespan,
        utils,
    },
    genetic_algorithm::params,
    iterated_greedy::options::{Options, OptionsGrid},
};

pub trait Solver {
    fn run(makespan: &mut Makespan, options: Option<Options>) -> u32;

    fn run_all(result_folder: &str) {
        // Get vector of all problem files (twice as we have to consume them)
        let problem_files = utils::get_test_problems();
        let problem_files_consumed = utils::get_test_problems();

        // Make sure problem files are in same order
        assert_eq!(
            &problem_files, &problem_files_consumed,
            "Order of problem files does not equal"
        );

        let num_problems = problem_files.len();

        // Initiate 2D vector of results: results[problem_file][parameter_combination]
        let results: Arc<Mutex<Vec<Vec<String>>>> =
            Arc::new(Mutex::new(Vec::with_capacity(problem_files.len())));

        let pb = utils::create_progress_bar(num_problems as u64);

        rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build_global()
            .unwrap();

        // Iterate all problem files
        problem_files
            .into_par_iter()
            .enumerate()
            .for_each(|(idx, problem_file)| {
                // Store filename and result from each parameter combination in vector
                let mut row = Vec::new();

                let i: Instance = parse(problem_files_consumed.get(idx).unwrap()).unwrap();

                row.push(String::from(
                    problem_files_consumed.get(idx).unwrap().to_str().unwrap(),
                ));

                let options = Options {
                    problem_file: Cow::Owned(problem_file),
                    ..Options::default()
                };

                let all_options = OptionsGrid::default().get_options(options);

                let mut m: Makespan = Makespan::new(&i);

                all_options.into_iter().for_each(|option| {
                    let makespan = Self::run(&mut m, Some(option));

                    row.push(makespan.to_string());
                });

                results.lock().unwrap().push(row);

                pb.inc(1);
            });

        pb.finish_with_message("Done");

        results
            .lock()
            .unwrap()
            .sort_by(|a, b| natural_lexical_cmp(&a[0], &b[0]));

        utils::write_results(result_folder, &results.lock().unwrap()).unwrap();

        println!(
            "All problems run, results are stored in `{}`",
            String::from(result_folder) + "/results.csv"
        );
    }
}
