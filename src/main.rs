mod common;
mod genetic_algorithm;
mod iterated_greedy;

use crate::common::construction::solver::Solver;
use clap::StructOpt;
use common::construction::{mddr::MDDR, neh::NEH};
use genetic_algorithm::{entities::options::Args, ga};
use iterated_greedy::iterated_greedy::{self as ig, IteratedGreedy};
fn main() {
    // Parse arguments (run steady state (-s), run all problems (-r), test all parameters (-a))
    let args = Args::parse();

    // Based on arguments, we either MDDR, NEH, IG or GA
    if args.mddr {
        MDDR::run_all("./solutions/mddr2");
    } else if args.neh {
        NEH::run_all("./solutions/neh2");
    } else if args.iterated_greedy {
        if args.run_all {
            IteratedGreedy::run_all("./solutions/ig");
        } else {
            ig::run_one();
        }
    }
    // If we run GA, we either run one problem file, or all problem files
    else if args.run_all {
        ga::run_all(&args);
    } else {
        ga::run_one(&args);
    }
}
