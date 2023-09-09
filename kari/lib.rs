#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod commands;
pub mod context;
pub mod logger;
pub mod updater;

#[cfg(test)]
mod tests;