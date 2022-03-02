mod genetic_algorithm;

use genetic_algorithm::{ga::GA, params::PROBLEM_FILE};

fn main() {
    let ga = GA::new(PROBLEM_FILE);
    ga.run();
}
