use std::sync::mpsc;
use std::thread;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod aggregation;
use aggregation::AggregationResults;

mod measurements;
pub use measurements::WeatherStations;

/// One billion.
pub const BILLION: usize = 1_000_000_000;

/// The number of rows each thread processes in chunks.
const CHUNK_SIZE: usize = 1 << 20;

/// Given an iterator that yields measurements for weather stations, aggregate each weather
/// station's data.
///
/// TODO(student): This is purposefully an very bad way to compute aggregations (namely, completely
/// sequentially). If you don't want to time out, you will need to introduce parallelism in some
/// manner. And even after you introduce parallelism, there are many different things you can do to
/// speed this up dramatically.
pub fn aggregate<'a, I>(measurements: I) -> AggregationResults
where
    I: Iterator<Item = (&'a str, f64)> + Send,
{
    let mut chunk_results = Vec::with_capacity(BILLION / CHUNK_SIZE);

    // Create an `mpsc` channel that threads will use to send their aggregation results back to the
    // main (current) thread.
    // Note that you can achieve the same effect via a mutex-protected vector.
    let (tx, rx) = mpsc::channel();

    // This scope is just an more ergonomic way to spawn threads and wait for all of them to finish
    // (join all threads), while also allowing the threads to access local data (like the
    // measurements iterator and the channel, for example).
    thread::scope(|s| {
        let chunks = measurements.chunks(CHUNK_SIZE);

        // This iterator is kind of hairy, so if you decide to change this code, make sure you read
        // the documentation for `Itertools::chunks`.
        for chunk in &chunks {
            // Collect the chunk measurements into a vector of measurements. This might seem
            // inefficient, but it is likely that you will achieve more by optimizing other parts of
            // your code first.
            let chunk = chunk.collect_vec();

            // Spawn a thread to process each chunk.
            s.spawn(|| {
                let mut results = AggregationResults::new();

                for (station, measurement) in chunk {
                    results.insert_measurement(station, measurement);
                }

                tx.send(results)
            });
        }
    });

    // The receiver will continue to wait unless all transmitters have been dropped, so since the
    // main thread does not need one, we should drop this now.
    drop(tx);

    // Collect the aggregation results.
    while let Ok(msg) = rx.recv() {
        chunk_results.push(msg);
    }

    // Once we have the results, use `rayon`'s `ParallelIterator` to combine all of them (reduce)
    // into a single value.
    chunk_results
        .into_par_iter()
        .reduce(AggregationResults::new, |mut a, b| {
            a.merge_aggregation(b);
            a
        })
}
