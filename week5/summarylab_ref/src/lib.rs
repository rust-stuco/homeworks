#![doc = include_str!("../README.md")]

pub trait Summary {
    fn msg_len(&self) -> usize;

    fn summarize(&self) -> String;

    fn get_info(&self) -> String;
}

pub mod reader;
