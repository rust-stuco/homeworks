#![doc = include_str!("../README.md")]

pub trait Summary {
    fn summarize(&self) -> String;

    fn get_info(&self) -> String;

    fn msg_len(&self) -> usize;
}

pub mod reader;
