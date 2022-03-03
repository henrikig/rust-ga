mod genetic_algorithm;

use genetic_algorithm::{ga::GA, params::PROBLEM_FILE};

fn main() {
    let mut ga = GA::new(PROBLEM_FILE);
    ga.run();
}

#[cfg(test)]
mod tests {
    use crate::genetic_algorithm::ga::GA;

    #[test]
    fn makespan_calculation_toy_problem() {
        let mut ga = GA::new_test();
        ga.population[0].makespan(&ga.problem);

        assert_eq!(Some(338), ga.population[0].makespan);
    }
}
