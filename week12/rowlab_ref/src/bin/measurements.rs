//! Taken and modified from https://github.com/PurpleMyst/1brc.rs/blob/main/src/bin/generate_data.rs
//!
//! Note that if this were a real command line executable, we would instead make the constants
//! command line arguments so that the user could provide their own files. However, since this is an
//! autograded assignment and we need specific file names, we are keeping these fixed.

use anyhow::{Result, anyhow};
use indicatif::{ProgressIterator, ProgressStyle};
use rand::prelude::*;
use rand_distr::Normal;
use regex::Regex;
use std::{
    fs::File,
    io::{BufWriter, prelude::*},
};

/// The output file to write the measurements into.
const OUT_FILE: &str = "../measurements.txt";

/// The file containing the different weather stations and their average temperatures.
const STATIONS_FILE: &str = "src/bin/stations.txt";

/// The regex pattern to read in the possible weather stations and their average temperatures.
const STATIONS_PATTERN: &str = r#"new WeatherStation\("([^*]+)", ([^)]+)\)"#;

/// The number of rows in the output measurements file.
const MEASUREMENTS: usize = 10000;

/// The template for the progress bar.
const PROGRESS_TEMPLATE: &str =
    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} ETA {eta_precise} ({per_sec})";

/// The standard deviation for the normally-distributed temperatures.
const STANDARD_DEVIATION: f64 = 20.0;

#[derive(Debug)]
struct WeatherStation<'a> {
    /// A weather station identifier.
    id: &'a str,
    /// The distribution in which we will randomly sample temperatures from.
    distr: Normal<f64>,
}

impl<'a> WeatherStation<'a> {
    /// Creates a new `WeatherStation`.
    pub fn new(id: &'a str, mean: f64) -> Self {
        Self {
            id,
            distr: Normal::new(mean, STANDARD_DEVIATION).unwrap(),
        }
    }

    /// Randomly samples a temperature measurement for the given `WeatherStation`.
    pub fn measure(&self) -> f64 {
        self.distr.sample(&mut rand::rng())
    }
}

fn main() -> Result<()> {
    // This regex will be used to parse the weather stations list.
    let re = Regex::new(STATIONS_PATTERN).unwrap();

    // Read in the weather stations.
    let stations_content =
        std::fs::read_to_string(STATIONS_FILE).expect("unable to read stations file");

    // Parse the weather station list using regex.
    let stations = stations_content
        .lines()
        .map(|line| {
            re.captures(line)
                .map(|cap| {
                    WeatherStation::new(
                        cap.get(1).unwrap().as_str(),
                        cap.get(2).unwrap().as_str().parse().unwrap(),
                    )
                })
                .ok_or_else(|| anyhow!("Invalid line: {line:?}"))
        })
        .collect::<Result<Vec<WeatherStation>, _>>()?;

    // The output measurements file writer.
    let mut writer = BufWriter::new(File::create(OUT_FILE)?);
    let mut rng = rand::rng();

    for _ in (0..MEASUREMENTS)
        .progress_with_style(ProgressStyle::default_bar().template(PROGRESS_TEMPLATE)?)
    {
        let station = stations.choose(&mut rng).unwrap();
        let measurement = station.measure();
        writeln!(writer, "{};{:.1}", station.id, measurement)?;
    }

    Ok(())
}
