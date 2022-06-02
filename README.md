# TIO4905

This repository contains the work on our master thesis. The source code for our algorithmic implementations is found in `/src`. This folder includes both a genetic algorithm and an iterated greedy algorithm, as well as different construction heuristics. The `/python` folder includes several scripts for visualising results from tests performed.

## Algorithms

The entry point of the algorithms are in the `main()` function in `src/main.rs`.

The program accepts five flags which can be set when calling `cargo run`. These define which and how algorithms are run:
Flag | Description
-----|-------------------------------------
`-i` | Run Iterated Greedy for all problem files
`-m` | Run MDDR for all problem files
`-n` | Run NEH for all problem files
`-r` | Run all the problem files with all possible parameter values for the GA
`-s` | Run the steady state version of the genetic algorithm

### Example usage

Run program in release mode with steady state version of GA and for all problem files and parameter value:

```sh
cargo run --release -- -s -r
```

Run program in developer mode with only steady state version:

```sh
cargo run -- -s
```

### Program flow

Depending on how the flags are specified, we either run all problem instances, or a single one, as specified in `main()`:

```rust
    // Parse arguments (run steady state (-s), run all problems (-r), test all parameters (-a))
    let args = Args::parse();

    // Based on arguments, we either MDDR, NEH, IG or GA
    if args.mddr {
        MDDR::run_all(...);
    } else if args.neh {
        NEH::run_all(...);
    } else if args.iterated_greedy {
        if args.run_all {
            IteratedGreedy::run_all(...);
        } else {
            iterated_greedy::run_one();
        }
    }
    // If we run GA, we either run one problem file, or all problem files
    else if args.run_all {
        ga::run_all(...);
    } else {
        ga::run_one(...);
    }
```

The function `main()` calls either the `run_all` or `run_one` function. In the `run_all` function, we iterate all problem files and all possible parameter combinations. For each problem and parameter combination, we create an instance of the genetic algorithm, and run it, either in steady state or regular (depending on whether the `s` flag is set). The results for each parameter combination and problem file are stored in a csv-file.

Similarly, for the `run_one` function, a single GA instance is created with the default parameter settings as defined in `src/genetic_algorithm/params.rs`. The solution to this file is stored and can be visualized with a visualization implemented in Python.

## Solution Visualization

When a single problem file is run, the corresponding solution is stored in `/solutions/ga`. This can be visualized by the Python-script in `python/visualizations/gantt.py`. Here, the `FILE` and `INSTANCE` variables must be set accordingly.

The requirements are found in `python/visualizations/requirements.txt`. A virtual environment is recommended:

```sh
python3 -m venv env
source env/bin/activate
pip install -r requirements.txt
```

### Gantt Chart Example

![Screenshot 2022-03-25 at 16 45 59](https://user-images.githubusercontent.com/47174810/160155650-36e02060-adf8-4b49-91df-03fbe82360d9.png)
