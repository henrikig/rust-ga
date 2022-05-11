use itertools::iproduct;

#[derive(Clone, Debug)]
pub struct Options {
    // Temperature
    pub temp: f64,

    // Number of jobs to remove
    pub block_size: i32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            temp: 0.1,
            block_size: 3,
        }
    }
}

// Struct containing all possible parameter values for each option
pub struct OptionsGrid {
    // Temperature
    pub temp: Vec<f64>,

    // Number of jobs to remove
    pub block_size: Vec<i32>,
}

// Set the default values
impl Default for OptionsGrid {
    fn default() -> OptionsGrid {
        OptionsGrid {
            temp: vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5],
            block_size: vec![2, 3, 5, 10],
        }
    }
}

impl OptionsGrid {
    pub fn get_options(self) -> Vec<Options> {
        iproduct!(self.temp, self.block_size)
            .map(|opt| Options {
                temp: opt.0,
                block_size: opt.1,
            })
            .collect()
    }
}
