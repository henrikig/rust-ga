#[cfg(test)]
mod tests {

    use crate::genetic_algorithm::{
        entities::{chromosome::Chromosome, problem::Problem},
        ga::GA,
        operators::crossover::{self, Crossover},
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

    #[test]
    fn crossover_sjox() {
        let p1 = Chromosome::from(vec![4, 7, 9, 3, 5, 2, 6, 8, 1]);
        let p2 = Chromosome::from(vec![9, 2, 4, 5, 7, 8, 6, 3, 1]);

        let (c1, c2) = crossover::SJOX::apply(&p1, &p2, Some(4));

        assert_eq!(c1.jobs, vec![4, 7, 9, 3, 2, 5, 6, 8, 1]);
        assert_eq!(c2.jobs, vec![9, 2, 4, 5, 7, 3, 6, 8, 1]);
    }
}
