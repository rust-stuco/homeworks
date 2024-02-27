#![doc = include_str!("../README.md")]

/// This module contains 4 iterators that you will implement!
///
/// Implement this module _before_ the [`hofs`] module.
pub mod iterators;

/// This module contains some functions that require higher-order functions to implement them.
///
/// Implement this module _after_ the [`iterators`] module.
pub mod hofs;

#[cfg(test)]
pub mod tests;
