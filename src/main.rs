mod genetic_algorithm;

use genetic_algorithm::{ga::GA, params::PROBLEM_FILE};

fn main() {
    let mut ga = GA::new(PROBLEM_FILE);
    ga.run();
}

#[cfg(test)]
mod tests {

    use crate::genetic_algorithm::{
        entities::{chromosome::Chromosome, problem::Problem},
        ga::GA,
    };

    #[test]
    fn makespan_calculation_toy_problem() {
        let mut ga = GA::new_test();
        ga.population[0].makespan(&ga.problem);

        assert_eq!(Some(338), ga.population[0].makespan);
    }

    #[test]
    fn chromosome_ordering() {
        let problem = Problem::toy_problem();

        let mut c1 = Chromosome::from(vec![0, 1, 2, 3, 4]);
        let mut c2 = Chromosome::from(vec![4, 3, 2, 1, 0]);

        c1.makespan(&problem);
        c2.makespan(&problem);

        assert!(c1 > c2);
        assert!(c1 >= c2);
        assert!(c1 != c2);
    }
}
