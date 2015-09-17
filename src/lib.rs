//! Reimplement Henry Baker's [no-iterator same-fringe solution](http://home.pipeline.com/~hbaker1/Iterator.html)

// Need Rust nightly, not beta or stable.
#![feature(iter_order)]
#![feature(slice_splits)]

pub mod tree;
pub mod immutable;
pub mod iterators;

pub mod algorithm;
