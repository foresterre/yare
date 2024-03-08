#![doc = include_str!("../README.md")]
#![deny(clippy::all)]

extern crate yare_macro as yare;

pub use yare::parameterized;

#[cfg(test)]
mod tests;
