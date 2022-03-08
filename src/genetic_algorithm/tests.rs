#[cfg(test)]
mod tests {

    use rand::thread_rng;

    use crate::{
        common::{instance::Instance, parser::parse},
        genetic_algorithm::{
            entities::chromosome::Chromosome,
            ga::GA,
            operators::crossover::{self, Crossover},
        },
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
    fn crossover_sjox() {
        // let p1 = Chromosome::from(vec![4, 7, 9, 3, 5, 2, 6, 8, 1]);
        // let p2 = Chromosome::from(vec![9, 2, 4, 5, 7, 8, 6, 3, 1]);

        let p1 = Chromosome::from(vec![
            3, 15, 17, 8, 14, 11, 13, 16, 19, 6, 1, 9, 18, 5, 4, 2, 10, 7, 20, 12,
        ]);
        let p2 = Chromosome::from(vec![
            3, 17, 9, 15, 14, 11, 13, 16, 6, 18, 5, 19, 7, 8, 4, 2, 1, 10, 20, 12,
        ]);

        let (c1, c2) = crossover::SJOX::apply(&p1, &p2, Some(8), &test_instance());

        assert_eq!(
            c2.jobs,
            vec![3, 17, 9, 15, 14, 11, 13, 16, 8, 19, 6, 1, 18, 5, 4, 2, 10, 7, 20, 12]
        );
        assert_eq!(
            c1.jobs,
            vec![3, 15, 17, 8, 14, 11, 13, 16, 9, 6, 18, 5, 19, 7, 4, 2, 1, 10, 20, 12]
        );
    }

    #[test]
    fn parse_problem() {
        let _instance = parse("./instances/ruiz/json/n20m2-1.json").unwrap();
    }

    fn test_instance() -> Instance {
        Instance {
            products: 5,
            stages: 2,
            machines: vec![2, 1],
            production_times: vec![
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
        let jobs: Vec<u32> = (0..instance.products).collect();

        Chromosome {
            jobs,
            makespan: None,
        }
    }
}
