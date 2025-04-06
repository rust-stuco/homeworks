use std::{fs::File, io::BufRead, path::Path};

mod aggregation;

use aggregation::AggregationResults;

/// Given a path to a file that contains temperature measurements for weather stations, aggregate
/// each weather station's data.
///
/// Panics if it encounters any malformed data in the measurements file.
///
/// TODO(student): This is purposefully an very bad way to compute aggregations (namely, completely
/// sequentially). If you don't want to time out, you will need to introduce parallelism in some
/// manner. And even after you introduce parallelism, there are many different things you can do to
/// speed this up dramatically.
///
/// Note that you don't need to try and maximize I/O for this assignment: maximizing CPU usage on
/// Gradescope will suffice.
pub fn aggregate(measurements_path: impl AsRef<Path>) -> AggregationResults {
    let measurements = File::open(measurements_path).expect("unable to read measurements");
    let buf_reader = std::io::BufReader::new(measurements);

    let mut results = AggregationResults::new();

    for line_res in buf_reader.lines() {
        let line = line_res.expect("Was unable to read the line");

        let mut split = line.trim().split(';');
        let station = split.next().expect("missing weather station");
        let temperature_str = split.next().expect("missing temperature");

        let parsed_temperature = temperature_str
            .parse::<f64>()
            .expect("unable to parse temperature");

        results.insert_measurement(station, parsed_temperature);
    }

    results
}
