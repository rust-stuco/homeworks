use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, path::Path};

mod aggregation;
use aggregation::StationAggregation;

/// A weather station identifier.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct WeatherStation(String);

/// The aggregation results for the billion row challenge.
///
/// Note to student: This is purposefully not an ideal structure! You are allowed to change what
/// types this struct contains.
#[derive(Debug)]
pub struct AggregationResults {
    results: HashMap<WeatherStation, StationAggregation>,
}

impl AggregationResults {
    pub fn new<P: AsRef<Path>>(stations_path: P) -> Self {
        todo!()
    }

    /// Converts the aggregations results into a [`String`].
    ///
    /// TODO(student): Is this function efficient? Is there another way you could do this? Note that
    /// on _every single line here_ there is an opportunity for optimization.
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
            res.push_str(&station.0);
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

        dbg!(&res);

        todo!()
    }
}

/// Given a path to a file that contains temperature measurements for weather stations, aggregate
/// each weather station's data.
pub fn aggregate<P: AsRef<Path>>(measurements_path: P) -> AggregationResults {
    todo!()
}
