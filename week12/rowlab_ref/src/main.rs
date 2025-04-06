use rowlab_ref::aggregate;

/// The output file to write the measurements into.
const MEASUREMENTS_FILE: &str = "../measurements.txt";

fn main() {
    let res = aggregate(MEASUREMENTS_FILE);
    println!("{}", res.into_string())
}
