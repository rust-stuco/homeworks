use rowlab::{BILLION, WeatherStations, aggregate};
use std::time::Instant;

fn main() {
    let stations = WeatherStations::new();
    let measurements = stations.measurements();

    let start = Instant::now();

    let res = aggregate(measurements.take(BILLION));

    let elapsed = start.elapsed();

    println!("{}", res.into_string());
    println!("Elapsed time: {:?}", elapsed);
}
