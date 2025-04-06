use std::fmt::Display;

/// Aggregate statistics for a specific [`WeatherStation`].
///
/// Note to student: This is purposefully not an ideal structure!
///
/// Can you think of better ways to store this data? Do the types make sense? Could you make
/// optimizations via the types you use? Do you even need all of these fields?
#[derive(Debug, Clone)]
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
        write!(f, "{}/{}/{}", self.min, self.mean, self.max)
    }
}

impl StationAggregation {
    pub fn new() -> Self {
        Self {
            min: f64::INFINITY,
            mean: 0.0,
            max: f64::NEG_INFINITY,
            sum_measurements: 0.0,
            num_measurements: 0.0,
        }
    }

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

impl Default for StationAggregation {
    fn default() -> Self {
        Self::new()
    }
}
