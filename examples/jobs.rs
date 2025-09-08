use std::time::Instant;

use clap::Parser;
use equal_parts::EqualParts;

#[derive(Parser)]
struct Args {
    /// Number of total jobs to run
    #[arg(short, long, default_value_t = 10_000_000)]
    num_jobs: usize,

    /// Number of parallel jobs
    #[arg(short = 'j', long, default_value_t = 4)]
    concurrent_jobs: usize,
}

fn main() {
    let args = Args::parse();
    let inputs = Vec::from_iter(1..=args.num_jobs);

    serial(inputs.clone());
    parallel(inputs, args.concurrent_jobs);
}

fn serial(inputs: Vec<usize>) {
    let start = Instant::now();
    let mut results = Vec::with_capacity(inputs.len());
    for input in inputs {
        let result = slow_compute(input);
        results.push(result);
    }
    let results = results;
    let elapsed = start.elapsed();

    println!(
        "Serial:   completed {} tasks in {:?}",
        results.len(),
        elapsed
    )
}

fn parallel(inputs: Vec<usize>, concurrent_jobs: usize) {
    let parts = inputs.equal_parts(concurrent_jobs);

    let start = Instant::now();

    let mut handles = Vec::with_capacity(concurrent_jobs);
    for part in parts {
        let part = Vec::from(part);
        let handle = std::thread::spawn(move || {
            let mut results = Vec::with_capacity(part.len());
            for input in part {
                let result = slow_compute(input);
                results.push(result);
            }
            results
        });
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(inputs.len());
    for handle in handles.into_iter() {
        let partial = handle.join().unwrap();
        results.extend(partial);
    }
    let results = results;
    let elapsed = start.elapsed();

    println!(
        "Parallel: completed {} tasks in {:?}",
        results.len(),
        elapsed
    )
}

/// Slowly calculates the integer square root of the input
fn slow_compute(input: usize) -> usize {
    let mut left = 1;
    let mut right = input - 1;

    while left < right {
        let mid = (left + right) / 2;
        if mid * mid == input {
            return mid;
        } else if mid * mid < input {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    left
}
