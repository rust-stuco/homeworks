use rowlab::aggregate;
use std::time::Instant;

/// The output file to write the measurements into.
const MEASUREMENTS_FILE: &str = "../measurements.txt";

fn main() {
    let start = Instant::now();

    let res = aggregate(MEASUREMENTS_FILE);
    println!("{}", res.into_string());

    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}
