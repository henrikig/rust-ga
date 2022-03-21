#[cfg(test)]
mod common_tests {

    use crate::common::instance::parse;
    use crate::common::makespan::Makespan;

    #[test]
    pub fn common_test_makespan_and_parse() {
        let ins = parse("instances\\ruiz\\json\\n20m2-1.json").unwrap();
        let mut makespan = Makespan {
            count: 0,
            instance: ins,
        };
        let inital_order: Vec<u32> = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let (makespan_test, _) = makespan.makespan(&inital_order);
        println!(
            "Makespan of test: {}, and makespan was run {} times.",
            makespan_test, makespan.count
        );
    }
}
