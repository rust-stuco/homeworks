#![doc = include_str!("../README.md")]

use std::fs::File;
use std::{
    io,
    io::{BufReader, Read},
};

/// A trait that shares methods of summarizing text like emails and tweets.
pub trait Summary {
    fn msg_len(&self) -> usize;

    fn summarize(&self) -> String;

    fn get_info(&self) -> String;
}

/// Given a file path, returns a [`String`] with the contents of the file.
pub fn read_file(file_path: String) -> Result<String, io::Error> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut file_str = String::new();
    reader.read_to_string(&mut file_str)?;
    Ok(file_str)
}

pub mod reader;
