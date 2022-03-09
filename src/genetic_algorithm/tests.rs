#[cfg(test)]
mod tests {

    use rand::thread_rng;

    use crate::{
        common::{instance::Instance, parser::parse},
        genetic_algorithm::{entities::chromosome::Chromosome, ga::GA},
    };

    #[test]
    fn makespan_calculation_toy_problem() {
        let instance = test_instance();

        let mut population: Vec<Chromosome> = Vec::with_capacity(1);
        let mating_pool: Vec<Chromosome> = Vec::with_capacity(1);

        // [0, 1, 2, 3, 4]
        population.push(test_chromosome(&instance));

        let mut ga = GA {
            instance,
            population,
            mating_pool,
            rng: thread_rng(),
        };

        ga.population[0].makespan(&ga.instance);

        assert_eq!(Some(333), ga.population[0].makespan);
    }

    #[test]
    fn chromosome_ordering() {
        let problem = test_instance();

        let mut c1 = Chromosome::from(vec![0, 1, 2, 3, 4]);
        let mut c2 = Chromosome::from(vec![4, 3, 2, 1, 0]);

        c1.makespan(&problem);
        c2.makespan(&problem);

        assert!(c1 > c2);
        assert!(c1 >= c2);
        assert!(c1 != c2);
    }

    #[test]
    fn parse_problem() {
        let _instance = parse("./instances/ruiz/json/n20m2-1.json").unwrap();
    }

    fn test_instance() -> Instance {
        Instance {
            jobs: 5,
            stages: 2,
            machines: vec![2, 1],
            processing_times: vec![
                vec![71, 98],
                vec![51, 54],
                vec![0, 49],
                vec![94, 28],
                vec![29, 90],
            ],
            setup_times: vec![
                vec![
                    vec![1, 2, 3, 4, 5],
                    vec![1, 2, 3, 4, 5],
                    vec![1, 2, 3, 4, 5],
                    vec![1, 2, 3, 4, 5],
                    vec![5, 2, 3, 4, 5],
                ],
                vec![
                    vec![1, 2, 3, 4, 5],
                    vec![1, 2, 7, 4, 5],
                    vec![1, 2, 3, 4, 5],
                    vec![1, 2, 3, 4, 5],
                    vec![3, 2, 3, 4, 5],
                ],
            ],
        }
    }

    pub fn test_chromosome(instance: &Instance) -> Chromosome {
        let jobs: Vec<u32> = (0..instance.jobs).collect();

        Chromosome {
            jobs,
            makespan: None,
        }
    }
}
