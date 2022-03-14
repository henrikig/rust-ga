#[cfg(test)]
pub mod tests {

    use crate::{
        common::{instance::Instance, makespan::Makespan, parser::parse},
        genetic_algorithm::entities::chromosome::Chromosome,
    };

    #[test]
    fn chromosome_ordering() {
        let problem = test_instance();

        let mut c1 = Chromosome::from(vec![0, 1, 2, 3, 4]);
        let mut c2 = Chromosome::from(vec![4, 3, 2, 1, 0]);

        let mut makespan = Makespan {
            count: 0,
            instance: problem.clone(),
        };

        c1.makespan(&mut makespan);
        c2.makespan(&mut makespan);

        assert!(c1 > c2);
        assert!(c1 >= c2);
        assert!(c1 != c2);
    }

    #[test]
    fn parse_problem() {
        let _instance = parse("./instances/ruiz/json/n20m2-1.json").unwrap();
    }

    pub fn test_instance() -> Instance {
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
}
