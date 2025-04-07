use rowlab::{WeatherStations, aggregate};
use std::time::Instant;

/// One billion.
const BILLION: usize = 1_000_000_000;

fn main() {
    let start = Instant::now();

    let stations = WeatherStations::new();
    let measurements = stations.measurements();

    let res = aggregate(measurements.take(BILLION));

    let elapsed = start.elapsed();

    println!("{}", res.into_string());
    println!("Elapsed time: {:?}", elapsed);
}
