use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;

/// Aggregate statistics for a specific [`WeatherStation`].
///
/// TODO(student): This is purposefully not an ideal structure! Can you think of better ways to
/// store this data? Do the types make sense? Could you make optimizations via the types you use? Do
/// you even need all of these fields?
#[derive(Debug)]
pub struct StationAggregation {
    /// The minimum temperature measurement.
    min: f64,
    /// The maximum temperature measurement.
    max: f64,
    /// The average / mean temperature measurement.
    mean: f64,
    /// Helper field for calculating mean (sum_measurements / num_measurements).
    sum_measurements: f64,
    /// Helper field for calculating mean (sum_measurements / num_measurements).
    num_measurements: f64,
}

impl Display for StationAggregation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}/{:.1}/{:.1}", self.min, self.mean, self.max)
    }
}

impl StationAggregation {
    /// Creates a new `StationAggregation` for computing aggregations.
    pub fn new() -> Self {
        Self {
            min: f64::INFINITY,
            mean: 0.0,
            max: f64::NEG_INFINITY,
            sum_measurements: 0.0,
            num_measurements: 0.0,
        }
    }

    /// Updates the aggregation with a new measurement.
    ///
    /// TODO(student): Is processing measurements one-by-one the best way to compute aggregations?
    /// Remember that you are allowed to add other methods in this implementation block!
    pub fn add_measurement(&mut self, measurement: f64) {
        // Update the minimum and maximums.
        self.min = self.min.min(measurement);
        self.max = self.max.max(measurement);

        // Update the average.
        self.sum_measurements += measurement;
        self.num_measurements += 1.0;
        self.mean = self.sum_measurements / self.num_measurements;
    }
}

/// The aggregation results for the billion row challenge.
///
/// TODO(student): This is purposefully not an ideal structure! You are allowed to change what
/// types this struct contains. Think about what this structure should represent, and where the data
/// might best be located.
#[derive(Debug)]
pub struct AggregationResults {
    results: HashMap<String, StationAggregation>,
}

impl AggregationResults {
    /// Creates an empty `AggregationResult`.
    ///
    /// TODO(student): Think about what information you know in advance (before you start
    /// aggregating temperatures). Can you leverage that information to make the aggregation faster?
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    // Updates the metrics for the given station with a measurement.
    pub fn insert_measurement(&mut self, station: &str, measurement: f64) {
        // We don't use the `entry` API in the `Some` case since it would require us to always turn
        // `station` into an owned `String`, since `.entry()` requires an owned type.
        match self.results.get_mut(station) {
            Some(val) => val.add_measurement(measurement),
            None => self
                .results
                .entry(station.to_string())
                .or_default()
                .add_measurement(measurement),
        }
    }

    /// Converts the aggregations results into a [`String`].
    pub fn into_string(self) -> String {
        // Sort the results by weather station ID and join into the output string format.
        let sorted_results: Vec<_> = self
            .results
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .collect();

        let mut res = String::new();
        res.push('{');

        // Append each weather station's metrics to the output string.
        for (station, aggregation) in sorted_results {
            res.push_str(&station);
            res.push('=');
            // Note that implementing `Display` on `StationAggregation` means that you can call
            // `to_string` and it will do a similar thing as `Display::fmt`.
            res.push_str(&aggregation.to_string());
            res.push(',');
            res.push(' ');
        }

        // Remove the trailing comma and space.
        assert_eq!(
            res.pop(),
            Some(' '),
            "somehow didn't aggregate any stations"
        );
        assert_eq!(
            res.pop(),
            Some(','),
            "somehow didn't aggregate any stations"
        );

        res.push('}');

        res
    }
}

impl Default for StationAggregation {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AggregationResults {
    fn default() -> Self {
        Self::new()
    }
}
