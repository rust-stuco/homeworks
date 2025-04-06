use rowlab_ref::aggregate;

/// The output file to write the measurements into.
const MEASUREMENTS_FILE: &str = "../measurements.txt";

fn main() {
    aggregate(MEASUREMENTS_FILE);
}
