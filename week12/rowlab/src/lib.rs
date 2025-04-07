mod aggregation;
use aggregation::AggregationResults;

mod measurements;
pub use measurements::WeatherStations;

/// One billion.
pub const BILLION: usize = 1_000_000_000;

/// Given an iterator that yields measurements for weather stations, aggregate each weather
/// station's data.
///
/// TODO(student): This is purposefully an very bad way to compute aggregations (namely, completely
/// sequentially). If you don't want to time out, you will need to introduce parallelism in some
/// manner. And even after you introduce parallelism, there are many different things you can do to
/// speed this up dramatically.
///
/// Also note that you are likely going to be bottlenecked by the input iterator. This is expected.
/// In the real 1 billion row challenge, the measurements came from a file, which would possibly be
/// even slower in some scenarios. Of course, if you make use of specific linux OS syscalls
/// (specifically `mmap`), you could eliminate a large amount of overhead. Regardless, for this
/// assignment the lower bound is approximately the same as the time it takes to run this function
/// but with the `s.spawn` completely commented out.
pub fn aggregate<'a, I>(measurements: I) -> AggregationResults
where
    I: Iterator<Item = (&'a str, f64)>,
{
    let mut results = AggregationResults::new();

    for (station, measurement) in measurements {
        results.insert_measurement(station, measurement);
    }

    results
}
