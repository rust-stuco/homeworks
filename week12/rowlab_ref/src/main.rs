use rowlab_ref::{WeatherStations, aggregate, BILLION};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let stations = WeatherStations::new();
    let measurements = stations.measurements();

    let res = aggregate(measurements.take(BILLION));

    let elapsed = start.elapsed();

    println!("{}", res.into_string());
    println!("Elapsed time: {:?}", elapsed);
}
