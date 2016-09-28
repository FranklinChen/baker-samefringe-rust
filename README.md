# Rust implementation of Henry Baker's same-fringe code

[![Build Status](https://travis-ci.org/FranklinChen/baker-samefringe-rust.png)](https://travis-ci.org/FranklinChen/baker-samefringe-rust)

The interesting part of the mutually recursive generator-consumer solution is how there is no heap allocation needed, smart pointers, or manually handled stacks for tracing iteration state. Just the runtime stack is used (at the risk of stack overflow in case of super-deep trees), yet everything is safe, even though there are references into the stack and closures in progress. This shows off some special aspects of Rust's ownership system.

Of course, there are more idiomatic and modular ways to write `same_fringe`. In Rust, one standard way would be to laboriously create an `Iterator` with manually managed state, to support controlled mutation if desired, just as in C++.
