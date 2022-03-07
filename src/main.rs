mod common;
mod genetic_algorithm;
use genetic_algorithm::{ga::GA, params::PROBLEM_FILE};
fn main() {
    let mut ga = GA::new(PROBLEM_FILE);
    ga.run()
}

/*
mod genetic_algorithm;

use genetic_algorithm::{ga::GA, params::PROBLEM_FILE};
@@ -6,3 +11,5 @@ fn main() {
    let mut ga = GA::new(PROBLEM_FILE);
    ga.run();
}
*/
