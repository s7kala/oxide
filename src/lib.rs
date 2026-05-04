#![cfg_attr(not(test), no_std)]

pub mod newline;

pub use newline::expand_newlines;