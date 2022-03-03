pub struct Problem {
    pub n_jobs: u32,
    pub m_stages: u32,
    pub machines: Vec<u32>,
    pub processing_times: Vec<Vec<u32>>,
    pub setup_times: Vec<Vec<u32>>,
}

impl Problem {
    pub fn init(problem: &str) -> Problem {
        println!("Problem file: {}", problem);
        // Mock problem
        Problem {
            n_jobs: 20,
            m_stages: 2,
            machines: vec![2, 2],
            processing_times: vec![
                vec![71, 98],
                vec![51, 54],
                vec![0, 49],
                vec![94, 28],
                vec![29, 90],
                vec![47, 14],
                vec![46, 27],
                vec![0, 97],
                vec![67, 40],
                vec![85, 67],
                vec![48, 88],
                vec![47, 31],
                vec![34, 40],
                vec![27, 4],
                vec![41, 34],
                vec![49, 52],
                vec![15, 89],
                vec![62, 52],
                vec![96, 90],
                vec![79, 74],
            ],
            setup_times: vec![vec![1; 20]; 40],
        }
    }

    pub fn toy_problem() -> Problem {
        // Small toy problem for validation
        Problem {
            n_jobs: 5,
            m_stages: 2,
            machines: vec![2, 1],
            processing_times: vec![
                vec![71, 98],
                vec![51, 54],
                vec![0, 49],
                vec![94, 28],
                vec![29, 90],
            ],
            setup_times: vec![vec![1; 5]; 10],
        }
    }
}
